#![feature(custom_derive, plugin)]
#![feature(question_mark)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate rand;
extern crate serde;
extern crate serde_json;

use hyper::client::response::Response;
use hyper::Client;
use hyper::header::{Authorization, Basic, Headers};
use hyper::status::StatusCode;
use serde::de::Deserialize;
use std::collections::BTreeMap;
use std::io::Read;

pub mod errors;
pub mod model;

mod idempotency_header;
mod stripe_version_header;
mod structured_encoding;

use errors::error::Error;
use errors::stripe_error;
pub use model::*;
use idempotency_header::IdempotencyKey;
use stripe_version_header::StripeVersion;
use structured_encoding::StructuredEncoding;

const BASE_URL: &'static str = "https://api.stripe.com/v1";
const API_VERSION: &'static str = "2016-03-07";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct StripeClient {
    pub key: String,
    client: Client
}

impl StripeClient {
    pub fn new(key: &str) -> StripeClient {
        StripeClient {
            key: String::from(key),
            client: Client::new()
        }
    }

    /// https://stripe.com/docs/api#retrieve_account
    /// Fetch account associated with self.key
    pub fn retrieve_current_account(&self) -> Result<Account> {
        let url = StripeClient::endpoint("/account");
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;

        StripeClient::parse_response(res)
    }

    /// https://stripe.com/docs/api#retrieve_account
    pub fn retrieve_account(&self, account_id: &str) -> Result<Account> {
        let url = StripeClient::endpoint(&format!("/accounts/{}", account_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;

        StripeClient::parse_response(res)
    }

    /// https://stripe.com/docs/api#create_account
    pub fn create_account(&self, params: &BTreeMap<String, String>) -> Result<Account> {
        let url = StripeClient::endpoint("/accounts");
        let body = params.encoded_string();
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;

        StripeClient::parse_response::<Account>(res)
    }

    /// https://stripe.com/docs/api#update_account
    pub fn update_account(
        &self,
        account_id: &str,
        update_params: &BTreeMap<String, String>
    ) -> Result<Account> {
        let url = StripeClient::endpoint(&format!("/accounts/{}", account_id));
        let body = update_params.encoded_string();
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;

        StripeClient::parse_response::<Account>(res)
    }

    /// https://stripe.com/docs/api#delete_account
    pub fn delete_account(&self, account_id: &str) -> Result<Delete> {
        let url = StripeClient::endpoint(&format!("/accounts/{}", account_id));
        let res = self.client.delete(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Delete>(res)
    }

    /// https://stripe.com/docs/api#reject_account
    pub fn reject_account(
        &self,
        account_id: &str,
        reason: &AccountRejectReason
    ) -> Result<Account> {
        let url = StripeClient::endpoint(&format!("/accounts/{}/reject", account_id));
        let body = serde_json::to_string(reason)?;
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;

        StripeClient::parse_response::<Account>(res)
    }

    /// https://stripe.com/docs/api#list_accounts
    pub fn list_connected_accounts(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<Account>> {
        let url = StripeClient::endpoint(&format!("/accounts?{}", params.encoded_string()));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;

        StripeClient::parse_response::<ApiList<Account>>(res)
    }

    /// https://stripe.com/docs/api#retrieve_balance
    pub fn retrieve_balance(&self) -> Result<Balance> {
        let url = StripeClient::endpoint("/balance");
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Balance>(res)
    }

    /// https://stripe.com/docs/api#retrieve_balance_transaction
    pub fn retrieve_balance_transaction(
        &self,
        balance_transaction_id: &str
    ) -> Result<BalanceTransaction> {
        let url = StripeClient::endpoint(&format!("/balance/history/{}", balance_transaction_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<BalanceTransaction>(res)
    }

    /// https://stripe.com/docs/api#balance_history
    pub fn list_balance_history(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<BalanceTransaction>> {
        let url = StripeClient::endpoint(&format!("/balance/history?{}", params.encoded_string()));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<ApiList<BalanceTransaction>>(res)
    }

    /// https://stripe.com/docs/api#create_charge
    pub fn create_charge(
        &self,
        params: &BTreeMap<String, String>,
        idempotency_key: Option<&str>
    ) -> Result<Charge> {
        let url = StripeClient::endpoint("/charges");
        let body = params.encoded_string();
        let mut req_builder = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes());

        if let Some(idempotency_key) = idempotency_key {
            req_builder = req_builder.header(IdempotencyKey::new(idempotency_key));
        }

        let res = req_builder.send()?;

        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api#retrieve_charge
    pub fn retrieve_charge(&self, charge_id: &str) -> Result<Charge> {
        let url = StripeClient::endpoint(&format!("/charges/{}", charge_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api#update_charge
    pub fn update_charge(
        &self,
        charge_id: &str,
        update: BTreeMap<String, String>
    ) -> Result<Charge> {
        let url = StripeClient::endpoint(&format!("/charges/{}", charge_id));
        let body = update.encoded_string();
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;
        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api#capture_charge
    pub fn capture_charge(
        &self,
        charge_id: &str,
        params: &BTreeMap<String, String>
    ) -> Result<Charge> {
        let url = StripeClient::endpoint(&format!("/charges/{}", charge_id));
        let body = params.encoded_string();
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;

        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api#list_charges
    pub fn list_charges(&self, params: &BTreeMap<String, String>) -> Result<ApiList<Charge>> {
        let params = params.encoded_string();
        let url = StripeClient::endpoint(&format!("/charges?{}", params));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<ApiList<Charge>>(res)
    }

    /// https://stripe.com/docs/api#create_customer
    pub fn create_customer(
        &self,
        params: Option<&BTreeMap<String, String>>
    ) -> Result<Customer> {
        let url = StripeClient::endpoint("/customers");
        let body = params.map(|p| p.encoded_string()).unwrap_or(String::new());
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;
        StripeClient::parse_response::<Customer>(res)
    }

    /// https://stripe.com/docs/api#retrieve_customer
    pub fn retrieve_customer(&self, customer_id: &str) -> Result<Customer> {
        let url = StripeClient::endpoint(&format!("/customers/{}", customer_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Customer>(res)
    }

    /// https://stripe.com/docs/api#update_customer
    pub fn update_customer(
        &self,
        customer_id: &str,
        params: &BTreeMap<String, String>
    ) -> Result<Customer> {
        let url = StripeClient::endpoint(&format!("/customers/{}", customer_id));
        let body = params.encoded_string();
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;
        StripeClient::parse_response::<Customer>(res)
    }

    /// https://stripe.com/docs/api#delete_customer
    pub fn delete_customer(&self, customer_id: &str) -> Result<Delete> {
        let url = StripeClient::endpoint(&format!("/customers/{}", customer_id));
        let res = self.client.delete(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Delete>(res)
    }

    /// https://stripe.com/docs/api#list_customers
    pub fn list_customers(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<Customer>> {
        let url = StripeClient::endpoint(&format!("/customers?{}", params.encoded_string()));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<ApiList<Customer>>(res)
    }

    /// https://stripe.com/docs/api#retrieve_dispute
    pub fn retrieve_dispute(&self, dispute_id: &str) -> Result<Dispute> {
        let url = StripeClient::endpoint(&format!("/disputes/{}", dispute_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Dispute>(res)
    }

    /// https://stripe.com/docs/api#update_dispute
    pub fn update_dispute(
        &self,
        dispute_id: &str,
        evidence: &BTreeMap<String, String>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<Dispute> {
        let url = StripeClient::endpoint(&format!("/disputes/{}", dispute_id));
        let mut body = evidence
            .into_iter()
            .map(|(k, v)| (format!("evidence[{}]", k), v.clone()))
            .collect::<BTreeMap<_, _>>()
            .encoded_string();

        if let Some(metadata) = metadata {
            let metadata = metadata
                .into_iter()
                .map(|(k, v)| (format!("metadata[{}]", k), v.clone()))
                .collect::<BTreeMap<_, _>>()
                .encoded_string();
            body.push('&');
            body.push_str(&metadata);
        }

        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_bytes())
            .send()?;
        StripeClient::parse_response::<Dispute>(res)
    }

    /// https://stripe.com/docs/api#close_dispute
    pub fn close_dispute(&self, dispute_id: &str) -> Result<Dispute> {
        let url = StripeClient::endpoint(&format!("/disputes/{}/close", dispute_id));
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Dispute>(res)
    }

    /// https://stripe.com/docs/api#list_disputes
    pub fn list_disputes(&self, args: &BTreeMap<String, String>) -> Result<ApiList<Dispute>> {
        let url = StripeClient::endpoint(&format!("/disputes?{}", args.encoded_string()));
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<ApiList<Dispute>>(res)
    }

    /// https://stripe.com/docs/api#retrieve_event
    pub fn retrieve_event<T: StripeObject>(&self, event_id: &str) -> Result<Event> {
        let url = StripeClient::endpoint(&format!("/events/{}", event_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Event>(res)
    }

    pub fn retrieve_all<T: StripeObject>(
        &self,
        list: ApiList<T>,
        args: Option<BTreeMap<String, String>>
    ) -> Result<Vec<T>> {
        let mut data = vec![];
        let mut list = list;
        let mut args = args.unwrap_or(BTreeMap::new());
        loop {
            data.append(&mut list.data);
            if list.has_more {
                if let Some(t) = data.last() {
                    args.insert(String::from("starting_after"), String::from(t.id()));
                }
                let res = self.client.get(&list.url)
                    .headers(self.default_headers())
                    .send()?;
                list = StripeClient::parse_response::<ApiList<T>>(res)?;
            } else {
                break;
            }
        }
        Ok(data)
    }

    fn endpoint(suffix: &str) -> String {
        if suffix.starts_with("/") {
            format!("{}{}", BASE_URL, suffix)
        } else {
            format!("{}/{}", BASE_URL, suffix)
        }
    }

    fn parse_response<T: Deserialize>(mut res: Response) -> Result<T> {
        match res.status {
            StatusCode::Ok => {
                let mut body = String::new();
                res.read_to_string(&mut body)?;
                let t = serde_json::from_str(&body)?;
                Ok(t)
            },
            _ => {
                let mut body = String::new();
                res.read_to_string(&mut body)?;
                println!("{}", &body);
                let err = serde_json::from_str::<stripe_error::StripeErrorWrapper>(&body)?.error;
                Err(Error::StripeError(err))
            }
        }
    }

    fn default_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
                username: self.key.clone(),
                password: None
        }));
        headers.set(StripeVersion::new(API_VERSION));
        headers
    }
}

#[cfg(test)]
mod test {
    use hyper::Client;
    use super::StripeClient;
    use std::error::Error;

    #[test]
    fn it_works() {
        let key = "sk_test_GBE7dKKNklsixtSM4VG876qt";
        // let key = "not_a_key";
        let client = StripeClient::new(&key);
        match client.fetch_account() {
            Ok(account) => println!("{:?}", account),
            Err(err) => {
                println!("{}", &err);
                println!("{}", err.description());
            }
        }
        panic!("boom");
    }
}

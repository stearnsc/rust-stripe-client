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
        self.get("/account", None)
    }

    /// https://stripe.com/docs/api#retrieve_account
    pub fn retrieve_account(&self, account_id: &str) -> Result<Account> {
        self.get(&format!("/accounts/{}", account_id), None)
    }

    /// https://stripe.com/docs/api#create_account
    pub fn create_account(&self, params: &BTreeMap<String, String>) -> Result<Account> {
        let body = params.encoded_string();
        self.post("/accounts", Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#update_account
    pub fn update_account(
        &self,
        account_id: &str,
        update_params: &BTreeMap<String, String>
    ) -> Result<Account> {
        let body = update_params.encoded_string();
        self.post(&format!("/accounts/{}", account_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#delete_account
    pub fn delete_account(&self, account_id: &str) -> Result<Delete> {
        self.delete(&format!("/accounts/{}", account_id))
    }

    /// https://stripe.com/docs/api#reject_account
    pub fn reject_account(
        &self,
        account_id: &str,
        reason: &AccountRejectReason
    ) -> Result<Account> {
        let body = serde_json::to_string(reason)?;
        self.post(&format!("/accounts/{}/reject", account_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#list_accounts
    pub fn list_connected_accounts(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<Account>> {
        self.get("/accounts", Some(params))
    }

    /// https://stripe.com/docs/api#retrieve_balance
    pub fn retrieve_balance(&self) -> Result<Balance> {
        self.get("/balance", None)
    }

    /// https://stripe.com/docs/api#retrieve_balance_transaction
    pub fn retrieve_balance_transaction(
        &self,
        balance_transaction_id: &str
    ) -> Result<BalanceTransaction> {
        self.get(&format!("/balance/history/{}", balance_transaction_id), None)
    }

    /// https://stripe.com/docs/api#balance_history
    pub fn list_balance_history(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<BalanceTransaction>> {
        self.get("/balance/history", Some(params))
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
        self.get(&format!("/charges/{}", charge_id), None)
    }

    /// https://stripe.com/docs/api#update_charge
    pub fn update_charge(
        &self,
        charge_id: &str,
        update: &BTreeMap<String, String>
    ) -> Result<Charge> {
        let body = update.encoded_string();
        self.post(&format!("/charges/{}", charge_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#capture_charge
    pub fn capture_charge(
        &self,
        charge_id: &str,
        params: &BTreeMap<String, String>
    ) -> Result<Charge> {
        let body = params.encoded_string();
        self.post(&format!("/charges/{}", charge_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#list_charges
    pub fn list_charges(&self, params: &BTreeMap<String, String>) -> Result<ApiList<Charge>> {
        self.get("/charges", Some(params))
    }

    /// https://stripe.com/docs/api#create_customer
    pub fn create_customer(
        &self,
        params: Option<&BTreeMap<String, String>>
    ) -> Result<Customer> {
        let body = params.map(|p| p.encoded_string());
        self.post("/customers", body.as_ref().map(|b| b.as_bytes()))
    }

    /// https://stripe.com/docs/api#retrieve_customer
    pub fn retrieve_customer(&self, customer_id: &str) -> Result<Customer> {
        self.get(&format!("/customers/{}", customer_id), None)
    }

    /// https://stripe.com/docs/api#update_customer
    pub fn update_customer(
        &self,
        customer_id: &str,
        params: &BTreeMap<String, String>
    ) -> Result<Customer> {
        let body = params.encoded_string();
        self.post(&format!("/customers/{}", customer_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#delete_customer
    pub fn delete_customer(&self, customer_id: &str) -> Result<Delete> {
        self.delete(&format!("/customers/{}", customer_id))
    }

    /// https://stripe.com/docs/api#list_customers
    pub fn list_customers(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<Customer>> {
        self.get("/customers", Some(params))
    }

    /// https://stripe.com/docs/api#retrieve_dispute
    pub fn retrieve_dispute(&self, dispute_id: &str) -> Result<Dispute> {
        self.get(&format!("/disputes/{}", dispute_id), None)
    }

    /// https://stripe.com/docs/api#update_dispute
    pub fn update_dispute(
        &self,
        dispute_id: &str,
        evidence: &BTreeMap<String, String>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<Dispute> {
        let mut body = structured("evidence", evidence);
        if let Some(metadata) = metadata {
            body.extend(structured("metadata", metadata));
        }

        let body = body.encoded_string();
        self.post(&format!("/disputes/{}", dispute_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#close_dispute
    pub fn close_dispute(&self, dispute_id: &str) -> Result<Dispute> {
        self.post(&format!("/disputes/{}/close", dispute_id), None)
    }

    /// https://stripe.com/docs/api#list_disputes
    pub fn list_disputes(&self, args: &BTreeMap<String, String>) -> Result<ApiList<Dispute>> {
        self.get("/disputes", Some(args))
    }

    /// https://stripe.com/docs/api#retrieve_event
    pub fn retrieve_event(&self, event_id: &str) -> Result<Event> {
        self.get(&format!("/events/{}", event_id), None)
    }

    /// https://stripe.com/docs/api#list_events
    pub fn list_events(&self, args: &BTreeMap<String, String>) -> Result<ApiList<Event>> {
        self.get("/events", Some(args))
    }

    /// https://stripe.com/docs/api#create_refund
    pub fn create_refund(
        &self,
        charge_id: &str,
        args: &BTreeMap<String, String>,
        metadata: Option<&BTreeMap<String, String>>
    ) -> Result<Refund> {
        let mut args = args.clone();
        args.insert("charge".to_string(), charge_id.to_string());
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }

        let body = args.encoded_string();
        self.post("/refunds", Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#retrieve_refund
    pub fn retrieve_refund(&self, refund_id: &str) -> Result<Refund> {
        self.get(&format!("/refunds/{}", refund_id), None)
    }

    /// https://stripe.com/docs/api#update_refund
    pub fn update_refund(
        &self,
        refund_id: &str,
        update: &BTreeMap<String, String>,
        metadata: Option<&BTreeMap<String, String>>
    ) -> Result<Refund> {
        let mut args = update.clone();
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }

        let body = args.encoded_string();
        self.post(&format!("/refunds/{}", refund_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#list_refunds
    pub fn list_refunds(&self, args: &BTreeMap<String, String>) -> Result<ApiList<Refund>> {
        self.get("/refunds", Some(args))
    }

    /// https://stripe.com/docs/api#create_card_token
    pub fn create_card_token(
        &self,
        card_args: &BTreeMap<String, String>,
        customer_id: Option<String>
    ) -> Result<Token> {
        let mut args = BTreeMap::new();
        args.extend(structured("card", card_args));
        if let Some(customer_id) = customer_id {
            args.insert("customer".to_string(), customer_id);
        }

        let body = args.encoded_string();
        self.post("/tokens", Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#create_bank_account_token
    pub fn create_bank_account_token(
        &self,
        bank_account_args: &BTreeMap<String, String>,
        customer_id: Option<String>,
    ) -> Result<Token> {
        let mut args = BTreeMap::new();
        args.extend(structured("bank_account", bank_account_args));
        if let Some(customer_id) = customer_id {
            args.insert("customer".to_string(), customer_id);
        }

        let body = args.encoded_string();
        self.post("/tokens", Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#create_pii_token
    pub fn create_pii_token(&self, pii: Option<String>) -> Result<Token> {
        let body = pii.map(|pii| format!("pii[personal_id_number]={}", pii));
        self.post("/tokens", body.as_ref().map(|s| s.as_bytes()))
    }

    /// https://stripe.com/docs/api#retrieve_token
    pub fn retrieve_token(&self, token_id: &str) -> Result<Token> {
        self.get(&format!("/tokens/{}", token_id), None)
    }

    /// https://stripe.com/docs/api#create_transfer
    pub fn create_transfer(
        &self,
        transfer_args: &BTreeMap<String, String>
    ) -> Result<Transfer> {
        let body = transfer_args.encoded_string();
        self.post("/transfers", Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#retrieve_transfer
    pub fn retrieve_transfer(&self, transfer_id: &str) -> Result<Transfer> {
        self.get(&format!("/transfers/{}", transfer_id), None)
    }

    /// https://stripe.com/docs/api#update_transfer
    pub fn update_transfer(
        &self,
        transfer_id: &str,
        description: Option<&str>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<Transfer> {
        let mut args = BTreeMap::new();
        if let Some(description) = description {
            args.insert("description".to_string(), description.to_string());
        }
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }

        let body = args.encoded_string();
        self.post(&format!("/transfers/{}", transfer_id), Some(body.as_bytes()))
    }

    /// https://stripe.com/docs/api#list_transfers
    pub fn list_transfers(
        &self,
        options: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<Transfer>> {
        self.get("/transfers", options)
    }

    pub fn get<T: Deserialize>(
        &self,
        endpoint: &str,
        params: Option<&BTreeMap<String, String>>
    ) -> Result<T> {
        let url = if let Some(params) = params {
            StripeClient::endpoint(&format!("{}?{}", endpoint, params.encoded_string()))
        } else {
            StripeClient::endpoint(endpoint)
        };
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<T>(res)
    }

    pub fn post<T: Deserialize>(
        &self,
        endpoint: &str,
        body: Option<&[u8]>
    ) -> Result<T> {
        let mut req_builder = self.client.post(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers());
        if let Some(body) = body {
            req_builder = req_builder.body(body);
        }
        StripeClient::parse_response::<T>(req_builder.send()?)
    }

    pub fn delete<T: Deserialize>(
        &self,
        endpoint: &str
    ) -> Result<T> {
        let res = self.client.delete(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response(res)
    }

    pub fn retrieve_all<T: StripeObject>(
        &self,
        list: ApiList<T>,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<Vec<T>> {
        let mut data = vec![];
        let mut list = list;
        loop {
            data.append(&mut list.data);

            if list.has_more {
                list = self.fetch_next_page(&list, args)?;
            } else {
                break;
            }
        }
        Ok(data)
    }

    pub fn fetch_next_page<T: StripeObject>(
        &self,
        list: &ApiList<T>,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<T>> {
        let mut args = args.map(|a| a.clone()).unwrap_or(BTreeMap::new());
        if list.has_more {
            if let Some(stripe_object) = list.data.last() {
                args.insert(String::from("starting_after"), String::from(stripe_object.id()));
            }
            self.get(&list.url, Some(&args))
        } else {
            Ok(ApiList {
                data: vec![],
                has_more: false,
                total_count: list.total_count,
                url: (&list.url).clone()
            })
        }
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

fn structured(name: &str, map: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    map.into_iter()
        .map(|(k, v)| (format!("{}[{}]", name, k), v.clone()))
        .collect::<BTreeMap<_, _>>()
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

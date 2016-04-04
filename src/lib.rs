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

#[macro_use]
mod custom_ser;

pub mod errors;
pub mod model;

mod idempotency_header;
mod stripe_version_header;

use errors::error::Error;
use errors::stripe_error;
pub use model::*;
use idempotency_header::IdempotencyKey;
use stripe_version_header::StripeVersion;

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

    /// Fetch the account associated with this API key
    pub fn fetch_account(&self) -> Result<Account> {
        let url = StripeClient::endpoint("/account");
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;

        StripeClient::parse_response(res)
    }

    /// Fetch account by account_id
    pub fn fetch_account_by_id(&self, account_id: &str) -> Result<Account> {
        let url = StripeClient::path(format!("/accounts/{}", account_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;

        StripeClient::parse_response(res)
    }

    pub fn create_account(
        &self,
        country: Option<String>,
        email: Option<String>,
        managed: bool
    ) -> Result<Account> {
        let url = StripeClient::endpoint("/accounts");

        let mut fields = BTreeMap::new();
        if let Some(country) = country {
            fields.insert(String::from("country"), country);
        }
        if let Some(email) = email {
            fields.insert(String::from("email"), email);
        }
        fields.insert(String::from("managed"), managed.to_string());

        let body = StripeClient::encode_body(&fields);

        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_slice())
            .send()?;

        StripeClient::parse_response::<Account>(res)
    }

    pub fn update_account(
        &self,
        account_id: &str,
        update_params: &BTreeMap<String, String>
    ) -> Result<Account> {
        let url = StripeClient::path(format!("/accounts/{}", account_id));
        let body = StripeClient::encode_body(update_params);

        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_slice())
            .send()?;

        StripeClient::parse_response::<Account>(res)
    }

    pub fn delete_account(&self, account_id: &str) -> Result<AccountDeleted> {
        let url = StripeClient::path(format!("/accounts/{}", account_id));
        let res = self.client.delete(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<AccountDeleted>(res)
    }

    pub fn reject_account(
        &self,
        account_id: &str,
        reason: &AccountRejectReason
    ) -> Result<Account> {
        let url = StripeClient::path(format!("/accounts/{}/reject", account_id));
        let body: Vec<u8> = serde_json::to_string(reason)?.into_bytes();
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_slice())
            .send()?;

        StripeClient::parse_response::<Account>(res)
    }

    /// https://stripe.com/docs/api/curl#list_accounts
    pub fn list_connected_accounts(
        &self,
        ending_before_account_id: Option<String>,
        starting_after_account_id: Option<String>,
        limit: Option<i64>
    ) -> Result<ApiList<Account>> {
        let mut params = BTreeMap::new();
        if let Some(ending_before) = ending_before_account_id {
            params.insert(String::from("ending_before"), ending_before);
        }
        if let Some(starting_after) = starting_after_account_id {
            params.insert(String::from("starting_after"), starting_after);
        }
        if let Some(limit) = limit {
            params.insert(String::from("limit"), limit.to_string());
        }

        let url = StripeClient::path(format!("accounts?{}", StripeClient::encode_params(&params)));

        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;

        StripeClient::parse_response::<ApiList<Account>>(res)
    }

    /// https://stripe.com/docs/api/curl#retrieve_balance
    pub fn retrieve_balance(&self) -> Result<Balance> {
        let url = StripeClient::endpoint("/balance");
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Balance>(res)
    }

    /// https://stripe.com/docs/api/curl#retrieve_balance_transaction
    pub fn retrieve_balance_transaction(
        &self,
        balance_transaction_id: &str
    ) -> Result<BalanceTransaction> {
        let url = StripeClient::path(format!("/balance/history/{}", balance_transaction_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<BalanceTransaction>(res)
    }

    /// https://stripe.com/docs/api/curl#balance_history
    pub fn list_balance_history(
        &self,
        params: &BTreeMap<String, String>
    ) -> Result<ApiList<BalanceTransaction>> {
        let params = StripeClient::encode_params(params);
        let url = StripeClient::path(format!("/balance/history?{}", params));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<ApiList<BalanceTransaction>>(res)
    }

    /// https://stripe.com/docs/api/curl#create_charge
    pub fn create_charge(
        &self,
        create_charge: CreateCharge,
        idempotency_key: Option<&str>
    ) -> Result<Charge> {
        let url = StripeClient::endpoint("/charges");
        let params = create_charge.as_map()?;
        let body = StripeClient::encode_body(&params);
        let mut req_builder = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_slice());

        if let Some(idempotency_key) = idempotency_key {
            req_builder = req_builder.header(IdempotencyKey::new(idempotency_key));
        }

        let res = req_builder.send()?;

        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api/curl#retrieve_charge
    pub fn retrieve_charge(&self, charge_id: &str) -> Result<Charge> {
        let url = StripeClient::path(format!("/charges/{}", charge_id));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api/curl#update_charge
    pub fn update_charge(&self, charge_id: &str, update: UpdateCharge) -> Result<Charge> {
        let url = StripeClient::path(format!("/charges/{}", charge_id));
        let params = update.as_map()?;
        let body = StripeClient::encode_body(&params);
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_slice())
            .send()?;
        StripeClient::parse_response::<Charge>(res)
    }

    pub fn capture_charge(
        &self,
        charge_id: &str,
        amount: Option<i64>,
        application_fee: Option<i64>,
        receipt_email: Option<String>,
        statement_descriptor: Option<String>,
    ) -> Result<Charge> {
        let url = StripeClient::path(format!("/charges/{}", charge_id));
        let mut params = BTreeMap::new();
        if let Some(amount) = amount {
            params.insert(String::from("amount"), amount.to_string());
        }
        if let Some(application_fee) = application_fee {
            params.insert(String::from("application_fee"), application_fee.to_string());
        }
        if let Some(receipt_email) = receipt_email {
            params.insert(String::from("receipt_email"), receipt_email);
        }
        if let Some(statement_descriptor) = statement_descriptor {
            params.insert(String::from("statement_descriptor"), statement_descriptor);
        }
        let body = StripeClient::encode_body(&params);
        let res = self.client.post(&url)
            .headers(self.default_headers())
            .body(body.as_slice())
            .send()?;

        StripeClient::parse_response::<Charge>(res)
    }

    /// https://stripe.com/docs/api/curl#list_charges
    pub fn list_charges(&self, params: &BTreeMap<String, String>) -> Result<ApiList<Charge>> {
        let params = StripeClient::encode_params(params);
        let url = StripeClient::path(format!("/charges?{}", params));
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<ApiList<Charge>>(res)
    }

    fn encode_params(params: &BTreeMap<String, String>) -> String {
        let mut param_string = String::new();
        if params.is_empty() {
            param_string
        } else {
            for (key, value) in params {
                param_string.push_str(key);
                param_string.push('=');
                param_string.push_str(value);
                param_string.push('&');
            }
            param_string.pop();
            param_string
        }
    }

    fn encode_body(map: &BTreeMap<String, String>) -> Vec<u8> {
        let mut body = String::new();
        for (key, value) in map {
            body.push_str(key);
            body.push('=');
            body.push_str(value);
            body.push('&');
        }
        body.pop();
        body.into_bytes()
    }

    fn path(suffix: String) -> String {
        if suffix.starts_with("/") {
            format!("{}{}", BASE_URL, suffix)
        } else {
            format!("{}/{}", BASE_URL, suffix)
        }
    }

    fn endpoint(suffix: &str) -> String {
        StripeClient::path(String::from(suffix))
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

#![feature(custom_derive, plugin)]
#![feature(question_mark)]
#![plugin(serde_macros)]

extern crate hyper;
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
pub mod idempotency_header;
pub mod model;

use errors::error::Error;
use errors::stripe_error;
use model::account::Account;
use idempotency_header::IdempotencyKey;

const BASE_URL: &'static str = "https://api.stripe.com/v1";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct StripeClient {
    pub key: String,
    client: Client
}

impl StripeClient {
    pub fn new(key: &str, client: Client) -> StripeClient {
        StripeClient {
            key: String::from(key),
            client: client
        }
    }

    pub fn get_account(&self) -> Result<Account> {
        let url = StripeClient::endpoint("/account");
        let mut res = self.client.get(&url)
            .header(self.authorization_header())
            .send()?;

        StripeClient::parse_response(&mut res)
    }

    pub fn create_account(
        &self,
        country: Option<String>,
        email: Option<String>,
        managed: bool
    ) -> Result<Account> {
        if !managed && email.is_none() {
            panic!("Email is required for non-managed accounts");
        } else {
            let url = StripeClient::endpoint("/accounts");

            let mut fields = BTreeMap::new();
            if let Some(country) = country {
                fields.insert(String::from("country"), country);
            }
            if let Some(email) = email {
                fields.insert(String::from("email"), email);
            }
            fields.insert(String::from("managed"), managed.to_string());
            let body = StripeClient::form_encode(&fields);

            let mut res = self.client.post(&url)
                .body(body.as_slice())
                .send()?;

            StripeClient::parse_response(&mut res)
        }
    }

    fn form_encode(map: &BTreeMap<String, String>) -> Vec<u8> {
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

    fn endpoint(suffix: &str) -> String {
        if suffix.starts_with("/") {
            format!("{}{}", BASE_URL, suffix)
        } else {
            format!("{}/{}", BASE_URL, suffix)
        }
    }

    fn parse_response<T: Deserialize>(res: &mut Response) -> Result<T> {
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
                let err = stripe_error::parse(&body)?;
                Err(Error::StripeError(err))
            }
        }

    }

    fn authorization_header(&self) -> Authorization<Basic> {
        Authorization(Basic {
            username: self.key.clone(),
            password: None
        })
    }

    fn idempotent(&self, headers: &mut Headers, key: &str) {
        headers.set(IdempotencyKey::new(key));
    }
}

#[cfg(test)]
mod test {
    use hyper::Client;
    use super::StripeClient;
    use std::error::Error;

    #[test]
    fn it_works() {
        // let key = "sk_test_GBE7dKKNklsixtSM4VG876qt";
        let key = "not_a_key";
        let client = StripeClient::new(&key, Client::new());
        match client.get_account() {
            Ok(account) => println!("{:?}", account),
            Err(err) => {
                println!("{}", &err);
                println!("{}", err.description());
            }
        }
        panic!("boom");
    }
}

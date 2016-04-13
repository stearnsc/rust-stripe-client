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

pub mod either;
pub mod errors;
pub mod model;

mod idempotency_header;
mod stripe_version_header;
mod structured_encoding;
mod time_constraint;
mod util;

pub use either::Either;
pub use time_constraint::TimeConstraint;

use either::Either::{Left, Right};
use errors::error::Error;
use errors::stripe_error;
use model::*;
use idempotency_header::IdempotencyKey;
use stripe_version_header::StripeVersion;
use structured_encoding::StructuredEncoding;
use util::*;

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
        args: &BTreeMap<String, String>
    ) -> Result<ApiList<BalanceTransaction>> {
        self.get("/balance/history", Some(args))
    }

    /// https://stripe.com/docs/api#create_charge
    pub fn create_charge(
        &self,
        args: &BTreeMap<String, String>,
        idempotency_key: Option<&str>
    ) -> Result<Charge> {
        let url = StripeClient::endpoint("/charges");
        let body = args.encoded_string();
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
        args: &BTreeMap<String, String>
    ) -> Result<Charge> {
        self.post(&format!("/charges/{}", charge_id), Some(args))
    }

    /// https://stripe.com/docs/api#capture_charge
    pub fn capture_charge(
        &self,
        charge_id: &str,
        args: &BTreeMap<String, String>
    ) -> Result<Charge> {
        self.post(&format!("/charges/{}", charge_id), Some(args))
    }

    /// https://stripe.com/docs/api#list_charges
    pub fn list_charges(
        &self,
        args: Option<&BTreeMap<String, String>>,
        created_constraint: Option<&TimeConstraint>,
        source_type: Option<SourceType>,
    ) -> Result<ApiList<Charge>> {
        let mut args = args.map(|p| p.clone()).unwrap_or(BTreeMap::new());
        if let Some(created_constraint) = created_constraint {
            args.extend(structured("created", &created_constraint.into()));
        }
        if let Some(source_type) = source_type {
            args.insert(format!("source[object]"), serde_json::to_string(&source_type)?);
        }
        self.get("/charges", if args.is_empty() { None } else { Some(&args) })
    }

    /// https://stripe.com/docs/api#create_customer
    pub fn create_customer(
        &self,
        args: Option<&BTreeMap<String, String>>,
        metadata: Option<&BTreeMap<String, String>>,
        shipping: Option<&Shipping>,
        card_token_or_args: Option<Either<String, &BTreeMap<String, String>>>
    ) -> Result<Customer> {
        let args = args.map(|a| a.clone());
        let metadata = metadata.map(|m| structured("metadata", m));
        let shipping = shipping.map(|s| structured("shipping", &s.into()));
        let card = card_token_or_args.map(|c| match c {
            Left(token) => {
                let mut map = BTreeMap::new();
                map.insert("source".to_string(), token);
                map
            },
            Right(card) => structured("source", card),
        });
        let args = or_join(args, or_join(metadata, or_join(shipping, card)));
        self.post("/customers", args.as_ref())
    }

    /// https://stripe.com/docs/api#retrieve_customer
    pub fn retrieve_customer(&self, customer_id: &str) -> Result<Customer> {
        self.get(&format!("/customers/{}", customer_id), None)
    }

    /// https://stripe.com/docs/api#update_customer
    pub fn update_customer(
        &self,
        customer_id: &str,
        args: Option<&BTreeMap<String, String>>,
        metadata: Option<&BTreeMap<String, String>>,
        shipping: Option<&Shipping>,
        card_token_or_args: Option<Either<String, &BTreeMap<String, String>>>
    ) -> Result<Customer> {
        let mut args = args.map(|a| a.clone()).unwrap_or(BTreeMap::new());
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }
        if let Some(shipping) = shipping {
            args.extend(structured("shipping", &shipping.into()));
        }
        if let Some(card) = card_token_or_args { match card {
            Left(token) => { args.insert("source".to_string(), token); }
            Right(card) => { args.extend(structured("source", card)); }
        }}
        self.post(&format!("/customers/{}", customer_id), Some(&args))
    }

    /// https://stripe.com/docs/api#delete_customer
    pub fn delete_customer(&self, customer_id: &str) -> Result<Delete> {
        self.delete(&format!("/customers/{}", customer_id))
    }

    /// https://stripe.com/docs/api#list_customers
    pub fn list_customers(
        &self,
        args: Option<&BTreeMap<String, String>>,
        created_constraint: Option<&TimeConstraint>,
    ) -> Result<ApiList<Customer>> {
        let args = args.map(|p| p.clone());
        let created_constraint = created_constraint.map(|c| structured("created", &c.into()));
        self.get("/customers", or_join(args, created_constraint).as_ref())
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
        let mut args = structured("evidence", evidence);
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }
        self.post(&format!("/disputes/{}", dispute_id), Some(&args))
    }

    /// https://stripe.com/docs/api#close_dispute
    pub fn close_dispute(&self, dispute_id: &str) -> Result<Dispute> {
        self.post(&format!("/disputes/{}/close", dispute_id), None)
    }

    /// https://stripe.com/docs/api#list_disputes
    pub fn list_disputes(
        &self,
        args: Option<&BTreeMap<String, String>>,
        created_constraint: Option<&TimeConstraint>,
    ) -> Result<ApiList<Dispute>> {
        let args = args.map(|a| a.clone());
        let created_constraint = created_constraint.map(|c| structured("created", &c.into()));
        self.get("/disputes", or_join(args, created_constraint).as_ref())
    }

    /// https://stripe.com/docs/api#retrieve_event
    pub fn retrieve_event(&self, event_id: &str) -> Result<Event> {
        self.get(&format!("/events/{}", event_id), None)
    }

    /// https://stripe.com/docs/api#list_events
    pub fn list_events(
        &self,
        args: Option<&BTreeMap<String, String>>,
        created_constraint: Option<&TimeConstraint>,
    ) -> Result<ApiList<Event>> {
        let args = args.map(|a| a.clone());
        let created_constraint = created_constraint.map(|c| structured("created", &c.into()));
        self.get("/events", or_join(args, created_constraint).as_ref())
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
        self.post("/refunds", Some(&args))
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
        self.post(&format!("/refunds/{}", refund_id), Some(&args))
    }

    /// https://stripe.com/docs/api#list_refunds
    pub fn list_refunds(&self, args: Option<&BTreeMap<String, String>>) -> Result<ApiList<Refund>> {
        self.get("/refunds", args)
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
        self.post("/tokens", Some(&args))
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

        self.post("/tokens", Some(&args))
    }

    /// https://stripe.com/docs/api#create_pii_token
    pub fn create_pii_token(&self, pii: Option<String>) -> Result<Token> {
        let args = pii.map(|pii| {
            let mut args = BTreeMap::new();
            args.insert("pii[personal_id_number]".to_string(), pii);
            args
        });
        self.post("/tokens", args.as_ref())
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
        self.post("/transfers", Some(transfer_args))
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

        self.post(&format!("/transfers/{}", transfer_id), Some(&args))
    }

    /// https://stripe.com/docs/api#list_transfers
    pub fn list_transfers(
        &self,
        created_constraint: Option<&TimeConstraint>,
        date_constraint: Option<&TimeConstraint>,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<Transfer>> {
        let args = args.map(|a| a.clone());
        let created_constraint = created_constraint.map(|c| structured("created", &c.into()));
        let date_constraint = date_constraint.map(|c| structured("date", &c.into()));
        let args = or_join(args, or_join(created_constraint, date_constraint));
        self.get("/transfers", args.as_ref())
    }

    /// https://stripe.com/docs/api#create_transfer_reversal
    pub fn create_transfer_reversal(
        &self,
        transfer_id: &str,
        args: Option<&BTreeMap<String, String>>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<TransferReversal> {
        let args = args.map(|a| a.clone());
        let metadata = metadata.map(|m| structured("metadata", m));
        self.post(
            &format!("/transfers/{}/reversals", transfer_id),
            or_join(args, metadata).as_ref()
        )
    }

    /// https://stripe.com/docs/api#retrieve_transfer_reversal
    pub fn retrieve_transfer_reversal(
        &self,
        transfer_id: &str,
        reversal_id: &str,
    ) -> Result<TransferReversal> {
        self.get(&format!("/transfers/{}/reversals/{}", transfer_id, reversal_id), None)
    }

    /// https://stripe.com/docs/api#update_transfer_reversal
    pub fn update_transfer_reversal(
        &self,
        transfer_id: &str,
        reversal_id: &str,
        description: Option<String>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<TransferReversal> {
        let description = description.map(|d| {
            let mut map = BTreeMap::new();
            map.insert("description".to_string(), d);
            map
        });
        let metadata = metadata.map(|m| structured("metadata", m));
        let body = or_join(description, metadata);
        self.get(&format!("/transfers/{}/reversals/{}", transfer_id, reversal_id), body.as_ref())
    }

    /// https://stripe.com/docs/api#list_transfer_reversals
    pub fn list_transfer_reversals(
        &self,
        transfer_id: &str,
        args: Option<&BTreeMap<String, String>>,
    ) -> Result<ApiList<TransferReversal>> {
        self.get(&format!("/transfers/{}/reversals", transfer_id), args)
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
    pub fn create_account(&self, args: &BTreeMap<String, String>) -> Result<Account> {
        self.post("/accounts", Some(args))
    }

    /// https://stripe.com/docs/api#update_account
    pub fn update_account(
        &self,
        account_id: &str,
        args: &BTreeMap<String, String>
    ) -> Result<Account> {
        self.post(&format!("/accounts/{}", account_id), Some(args))
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
        let mut args = BTreeMap::new();
        args.insert("reason".to_string(), serde_json::to_string(reason)?);
        self.post(&format!("/accounts/{}/reject", account_id), Some(&args))
    }

    /// https://stripe.com/docs/api#list_accounts
    pub fn list_accounts(
        &self,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<Account>> {
        self.get("/accounts", args)
    }

    /// https://stripe.com/docs/api#create_fee_refund
    pub fn create_fee_refund(
        &self,
        fee_id: &str,
        amount: Option<i64>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<FeeRefund> {
        let amount = amount.map(|amount| {
            let mut map = BTreeMap::new();
            map.insert("amount".to_string(), amount.to_string());
            map
        });
        let metadata = metadata.map(|m| structured("metadata", m));
        let args = or_join(amount, metadata);
        self.post(&format!("/application_fees/{}/refunds", fee_id), args.as_ref())
    }

    /// https://stripe.com/docs/api#retrieve_fee_refund
    pub fn retrieve_fee_refund(&self, fee_id: &str, refund_id: &str) -> Result<FeeRefund> {
        self.get(&format!("/application_fees/{}/refunds/{}", fee_id, refund_id), None)
    }

    /// https://stripe.com/docs/api#update_fee_refund
    pub fn update_fee_refund(
        &self,
        fee_id: &str,
        refund_id: &str,
        metadata: &BTreeMap<String, String>
    ) -> Result<FeeRefund> {
        let args = structured("metadata", metadata);
        self.post(&format!("/application_fees/{}/refunds/{}", fee_id, refund_id), Some(&args))
    }

    /// https://stripe.com/docs/api#list_fee_refunds
    pub fn list_fee_refunds(
        &self,
        fee_id: &str,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<FeeRefund>> {
        self.get(&format!("/application_fees/{}/refunds", fee_id), args)
    }

    /// https://stripe.com/docs/api#retrieve_application_fee
    pub fn retrieve_application_fee(&self, fee_id: &str) -> Result<ApplicationFee> {
        self.get(&format!("/application_fees/{}", fee_id), None)
    }

    /// https://stripe.com/docs/api#list_application_fees
    pub fn list_application_fees(
        &self,
        created_constraint: Option<&TimeConstraint>,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<ApplicationFee>> {
        let created_constraint = created_constraint.map(|c| structured("created", &c.into()));
        let args = args.map(|a| a.clone());
        self.get("/application_fees", or_join(created_constraint, args).as_ref())
    }

    /// https://stripe.com/docs/api#create_recipient
    pub fn create_recipient(
        &self,
        args: &BTreeMap<String, String>,
        bank_account_token_or_args: Option<Either<String, &BTreeMap<String, String>>>,
        card_token_or_args: Option<Either<String, &BTreeMap<String, String>>>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<Recipient> {
        let mut args = args.clone();
        if let Some(bank_account) = bank_account_token_or_args { match bank_account {
            Left(token)    => { args.insert("bank_account".to_string(), token); },
            Right(account) => { args.extend(structured("bank_account", account)); },
        }}
        if let Some(card) = card_token_or_args { match card {
            Left(token) => { args.insert("card".to_string(), token); },
            Right(card) => { args.extend(structured("card", card)); },
        }}
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }
        self.post("/recipients", Some(&args))
    }

    /// https://stripe.com/docs/api#retrieve_recipient
    pub fn retrieve_recipient(&self, recipient_id: &str) -> Result<Recipient> {
        self.get(&format!("/recipients/{}", recipient_id), None)
    }

    /// https://stripe.com/docs/api#update_recipient
    pub fn update_recipient(
        &self,
        recipient_id: &str,
        args: Option<&BTreeMap<String, String>>,
        bank_account_token_or_args: Option<Either<String, &BTreeMap<String, String>>>,
        card_token_or_args: Option<Either<String, &BTreeMap<String, String>>>,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<Recipient> {
        let mut args = args.map(|a| a.clone()).unwrap_or(BTreeMap::new());
        if let Some(bank_account) = bank_account_token_or_args { match bank_account {
            Left(token)    => { args.insert("bank_account".to_string(), token); },
            Right(account) => { args.extend(structured("bank_account", account)); },
        }}
        if let Some(card) = card_token_or_args { match card {
            Left(token) => { args.insert("card".to_string(), token); },
            Right(card) => { args.extend(structured("card", card)); },
        }}
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }
        self.post(&format!("/recipients/{}", recipient_id), Some(&args))
    }

    /// https://stripe.com/docs/api#delete_recipient
    pub fn delete_recipient(&self, recipient_id: &str) -> Result<Delete> {
        self.delete(&format!("/recipients/{}", recipient_id))
    }

    /// https://stripe.com/docs/api#list_recipients
    pub fn list_recipients(
        &self,
        created_constraint: Option<&TimeConstraint>,
        args: Option<&BTreeMap<String, String>>,
    ) -> Result<ApiList<Recipient>> {
        let created_constraint = created_constraint.map(|c| structured("created", &c.into()));
        let args = args.map(|a| a.clone());
        self.get("/recipients", or_join(created_constraint, args).as_ref())
    }

    /// https://stripe.com/docs/api#list_country_specs
    pub fn list_country_specs(
        &self,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<CountrySpec>> {
        self.get("/country_specs", args)
    }

    /// https://stripe.com/docs/api#retrieve_country_spec
    pub fn retrieve_country_spec(&self, country_id: &str) -> Result<CountrySpec> {
        self.get(&format!("/country_specs/{}", country_id), None)
    }

    /// https://stripe.com/docs/api#account_create_bank_account
    pub fn account_create_bank_account(
        &self,
        account_id: &str,
        bank_account_token_or_args: Either<String, &BTreeMap<String, String>>,
        default_for_currency: bool,
        metadata: Option<&BTreeMap<String, String>>,
    ) -> Result<BankAccount> {
        let mut args = BTreeMap::new();
        match bank_account_token_or_args {
            Left(token) => { args.insert("external_account".to_string(), token); },
            Right(account) => { args.extend(structured("external_account", account)); },
        }
        args.insert("default_for_currency".to_string(), default_for_currency.to_string());
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }
        self.post(&format!("/accounts/{}/external_accounts", account_id), Some(&args))
    }

    /// https://stripe.com/docs/api#account_retrieve_bank_account
    pub fn account_retrieve_bank_account(
        &self,
        account_id: &str,
        bank_account_id: &str
    ) -> Result<BankAccount> {
        self.get(&format!("/accounts/{}/external_accounts/{}", account_id, bank_account_id), None)
    }

    /// https://stripe.com/docs/api#account_update_bank_account
    pub fn account_update_bank_account(
        &self,
        account_id: &str,
        bank_account_id: &str,
        args: &BTreeMap<String, String>,
        metadata: Option<&BTreeMap<String, String>>
    ) -> Result<BankAccount> {
        let mut args = args.clone();
        if let Some(metadata) = metadata {
            args.extend(structured("metadata", metadata));
        }
        self.post(
            &format!("/accounts/{}/external_accounts/{}", account_id, bank_account_id),
            Some(&args)
        )
    }

    /// https://stripe.com/docs/api#account_delete_bank_account
    pub fn account_delete_bank_account(
        &self,
        account_id: &str,
        bank_account_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/accounts/{}/external_accounts/{}", account_id, bank_account_id))
    }

    /// https://stripe.com/docs/api#account_list_bank_accounts
    pub fn account_list_bank_accounts(
        &self,
        account_id: &str,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<BankAccount>> {
        self.get(&format!("/accounts/{}/external_accounts", account_id), args)
    }

    /// https://stripe.com/docs/api#account_create_card
    // pub fn account_create_card(&self, )

    pub fn get<T: Deserialize>(
        &self,
        endpoint: &str,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<T> {
        let url = if let Some(args) = args {
            StripeClient::endpoint(&format!("{}?{}", endpoint, args.encoded_string()))
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
        args: Option<&BTreeMap<String, String>>,
    ) -> Result<T> {
        let body = args.map(|b| b.encoded_string());
        let mut req_builder = self.client.post(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers());

        if let Some(ref body) = body.as_ref() {
            req_builder = req_builder.body(body.as_bytes());
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

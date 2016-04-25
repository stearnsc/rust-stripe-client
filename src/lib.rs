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
mod url_encodable;
mod time_constraint;

pub use either::Either;
pub use time_constraint::TimeConstraint;

use either::Either::{Left, Right};
use errors::error::Error;
use errors::stripe_error;
use model::*;
use idempotency_header::IdempotencyKey;
use stripe_version_header::StripeVersion;
use url_encodable::UrlEncodable;

const BASE_URL: &'static str = "https://api.stripe.com/v1";
const API_VERSION: &'static str = "2016-03-07";

pub type Result<T> = std::result::Result<T, Error>;

const NO_PARAMS: &'static Option<(String, String)> = &None;
const TOTAL_COUNT_PARAM: (&'static str, &'static str) = ("include[]", "total_count");

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
        self.get("/balance", NO_PARAMS)
    }

    /// https://stripe.com/docs/api#retrieve_balance_transaction
    pub fn retrieve_balance_transaction(
        &self,
        balance_transaction_id: &str
    ) -> Result<BalanceTransaction> {
        self.get(&format!("/balance/history/{}", balance_transaction_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#balance_history
    pub fn list_balance_history(
        &self,
        args: BTreeMap<String, String>
    ) -> Result<ApiList<BalanceTransaction>> {
        self.get("/balance/history", &args)
    }

    /// https://stripe.com/docs/api#create_charge
    pub fn create_charge(
        &self,
        args: BTreeMap<String, String>,
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
    pub fn retrieve_charge(
        &self,
        charge_id: &str
    ) -> Result<Charge> {
        self.get(&format!("/charges/{}", charge_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_charge
    pub fn update_charge(
        &self,
        charge_id: &str,
        args: BTreeMap<String, String>
    ) -> Result<Charge> {
        self.post(&format!("/charges/{}", charge_id), &args)
    }

    /// https://stripe.com/docs/api#capture_charge
    pub fn capture_charge(
        &self,
        charge_id: &str,
        args: BTreeMap<String, String>
    ) -> Result<Charge> {
        self.post(&format!("/charges/{}", charge_id), &args)
    }

    /// https://stripe.com/docs/api#list_charges
    pub fn list_charges(
        &self,
        args: Option<BTreeMap<String, String>>,
        created_constraint: Option<TimeConstraint>,
        source_type: Option<SourceType>,
    ) -> Result<ApiList<Charge>> {
        let source_type = match source_type {
            Some(st) => Some(("source[object]", serde_json::to_string(&st)?)),
            None => None,
        };
        self.get("/charges", &(
            TOTAL_COUNT_PARAM,
            args,
            UrlEncodable::named("created", &created_constraint),
            source_type
        ))
    }

    /// https://stripe.com/docs/api#create_customer
    pub fn create_customer(
        &self,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
        shipping: Option<Shipping>,
        card_token_or_args: Option<Either<String, BTreeMap<String, String>>>
    ) -> Result<Customer> {
        self.post("/customers", &(
            args,
            UrlEncodable::named("metadata", &metadata),
            UrlEncodable::named("shipping", &shipping),
            card_token_or_args.map(|c| token_or_args("source", c))
        ))
    }

    /// https://stripe.com/docs/api#retrieve_customer
    pub fn retrieve_customer(&self, customer_id: &str) -> Result<Customer> {
        self.get(&format!("/customers/{}", customer_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_customer
    pub fn update_customer(
        &self,
        customer_id: &str,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
        shipping: Option<Shipping>,
        card_token_or_args: Option<Either<String, BTreeMap<String, String>>>
    ) -> Result<Customer> {
        self.post(&format!("/customers/{}", customer_id), &(
            args,
            UrlEncodable::named("metadata", &metadata),
            UrlEncodable::named("shipping", &shipping),
            card_token_or_args.map(|ct| token_or_args("source", ct))
        ))
    }

    /// https://stripe.com/docs/api#delete_customer
    pub fn delete_customer(&self, customer_id: &str) -> Result<Delete> {
        self.delete(&format!("/customers/{}", customer_id))
    }

    /// https://stripe.com/docs/api#list_customers
    pub fn list_customers(
        &self,
        args: Option<BTreeMap<String, String>>,
        created_constraint: Option<TimeConstraint>,
    ) -> Result<ApiList<Customer>> {
        self.get("/customers", &(
            TOTAL_COUNT_PARAM,
            args,
            UrlEncodable::named("created", &created_constraint)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_dispute
    pub fn retrieve_dispute(
        &self,
        dispute_id: &str
    ) -> Result<Dispute> {
        self.get(&format!("/disputes/{}", dispute_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_dispute
    pub fn update_dispute(
        &self,
        dispute_id: &str,
        evidence: BTreeMap<String, String>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<Dispute> {
        self.post(&format!("/disputes/{}", dispute_id), &(
            UrlEncodable::named("evidence", &evidence),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#close_dispute
    pub fn close_dispute(
        &self,
        dispute_id: &str
    ) -> Result<Dispute> {
        self.post(&format!("/disputes/{}/close", dispute_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#list_disputes
    pub fn list_disputes(
        &self,
        args: Option<BTreeMap<String, String>>,
        created_constraint: Option<TimeConstraint>,
    ) -> Result<ApiList<Dispute>> {
        self.get("/disputes", &(
            args,
            UrlEncodable::named("created", &created_constraint)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_event
    pub fn retrieve_event(&self, event_id: &str) -> Result<Event> {
        self.get(&format!("/events/{}", event_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#list_events
    pub fn list_events(
        &self,
        args: Option<BTreeMap<String, String>>,
        created_constraint: Option<TimeConstraint>,
    ) -> Result<ApiList<Event>> {
        self.get("/events", &(
            TOTAL_COUNT_PARAM,
            args,
            UrlEncodable::named("created", &created_constraint)
        ))
    }

    /// https://stripe.com/docs/api#create_refund
    pub fn create_refund(
        &self,
        charge_id: &str,
        args: BTreeMap<String, String>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<Refund> {
        self.post("/refunds", &(
            ("charge", charge_id),
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_refund
    pub fn retrieve_refund(
        &self,
        refund_id: &str
    ) -> Result<Refund> {
        self.get(&format!("/refunds/{}", refund_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_refund
    pub fn update_refund(
        &self,
        refund_id: &str,
        update: BTreeMap<String, String>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<Refund> {
        self.post(&format!("/refunds/{}", refund_id), &(
            update,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#list_refunds
    pub fn list_refunds(
        &self,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<Refund>> {
        self.get("/refunds", &args)
    }

    /// https://stripe.com/docs/api#create_card_token
    pub fn create_card_token(
        &self,
        card_args: BTreeMap<String, String>,
        customer_id: Option<String>
    ) -> Result<Token> {
        self.post("/tokens", &(
            UrlEncodable::named("card", &card_args),
            customer_id.map(|id| ("customer", id))
        ))
    }

    /// https://stripe.com/docs/api#create_bank_account_token
    pub fn create_bank_account_token(
        &self,
        bank_account_args: BTreeMap<String, String>,
        customer_id: Option<String>,
    ) -> Result<Token> {
        self.post("/tokens", &(
            UrlEncodable::named("bank_account", &bank_account_args),
            customer_id.map(|id| ("customer", id))
        ))
    }

    /// https://stripe.com/docs/api#create_pii_token
    pub fn create_pii_token(
        &self,
        pii: Option<String>
    ) -> Result<Token> {
        self.post("/tokens",
            &pii.map(|pii| ("pii[personal_id_number]", pii))
        )
    }

    /// https://stripe.com/docs/api#retrieve_token
    pub fn retrieve_token(
        &self,
        token_id: &str
    ) -> Result<Token> {
        self.get(&format!("/tokens/{}", token_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#create_transfer
    pub fn create_transfer(
        &self,
        transfer_args: BTreeMap<String, String>
    ) -> Result<Transfer> {
        self.post("/transfers", &transfer_args)
    }

    /// https://stripe.com/docs/api#retrieve_transfer
    pub fn retrieve_transfer(
        &self,
        transfer_id: &str
    ) -> Result<Transfer> {
        self.get(&format!("/transfers/{}", transfer_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_transfer
    pub fn update_transfer(
        &self,
        transfer_id: &str,
        description: Option<String>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<Transfer> {
        self.post(&format!("/transfers/{}", transfer_id), &(
            description.map(|d| ("description", d)),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#list_transfers
    pub fn list_transfers(
        &self,
        created_constraint: Option<TimeConstraint>,
        date_constraint: Option<TimeConstraint>,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<Transfer>> {
        self.get("/transfers", &(
            args,
            UrlEncodable::named("created", &created_constraint),
            UrlEncodable::named("date", &date_constraint)
        ))
    }

    /// https://stripe.com/docs/api#create_transfer_reversal
    pub fn create_transfer_reversal(
        &self,
        transfer_id: &str,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<TransferReversal> {
        self.post(&format!("/transfers/{}/reversals", transfer_id), &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_transfer_reversal
    pub fn retrieve_transfer_reversal(
        &self,
        transfer_id: &str,
        reversal_id: &str,
    ) -> Result<TransferReversal> {
        self.get(&format!("/transfers/{}/reversals/{}", transfer_id, reversal_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_transfer_reversal
    pub fn update_transfer_reversal(
        &self,
        transfer_id: &str,
        reversal_id: &str,
        description: Option<String>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<TransferReversal> {
        self.get(&format!("/transfers/{}/reversals/{}", transfer_id, reversal_id), &(
            description.map(|d| ("description", d)),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#list_transfer_reversals
    pub fn list_transfer_reversals(
        &self,
        transfer_id: &str,
        args: Option<BTreeMap<String, String>>,
    ) -> Result<ApiList<TransferReversal>> {
        self.get(&format!("/transfers/{}/reversals", transfer_id), &args)
    }

    /// https://stripe.com/docs/api#retrieve_account
    /// Fetch account associated with self.key
    pub fn retrieve_current_account(&self) -> Result<Account> {
        self.get("/account", NO_PARAMS)
    }

    /// https://stripe.com/docs/api#retrieve_account
    pub fn retrieve_account(
        &self,
        account_id: &str
    ) -> Result<Account> {
        self.get(&format!("/accounts/{}", account_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#create_account
    pub fn create_account(
        &self,
        args: BTreeMap<String, String>
    ) -> Result<Account> {
        self.post("/accounts", &args)
    }

    /// https://stripe.com/docs/api#update_account
    pub fn update_account(
        &self,
        account_id: &str,
        args: BTreeMap<String, String>
    ) -> Result<Account> {
        self.post(&format!("/accounts/{}", account_id), &args)
    }

    /// https://stripe.com/docs/api#delete_account
    pub fn delete_account(
        &self,
        account_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/accounts/{}", account_id))
    }

    /// https://stripe.com/docs/api#reject_account
    pub fn reject_account(
        &self,
        account_id: &str,
        reason: AccountRejectReason
    ) -> Result<Account> {
        self.post(&format!("/accounts/{}/reject", account_id),
            &("reason", serde_json::to_string(&reason)?)
        )
    }

    /// https://stripe.com/docs/api#list_accounts
    pub fn list_accounts(
        &self,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<Account>> {
        self.get("/accounts", &args)
    }

    /// https://stripe.com/docs/api#create_fee_refund
    pub fn create_fee_refund(
        &self,
        fee_id: &str,
        amount: Option<i64>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<FeeRefund> {
        self.post(&format!("/application_fees/{}/refunds", fee_id), &(
            amount.map(|a| ("amount", a.to_string())),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_fee_refund
    pub fn retrieve_fee_refund(
        &self,
        fee_id: &str,
        refund_id: &str
    ) -> Result<FeeRefund> {
        self.get(&format!("/application_fees/{}/refunds/{}", fee_id, refund_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_fee_refund
    pub fn update_fee_refund(
        &self,
        fee_id: &str,
        refund_id: &str,
        metadata: BTreeMap<String, String>
    ) -> Result<FeeRefund> {
        self.post(&format!("/application_fees/{}/refunds/{}", fee_id, refund_id),
            &UrlEncodable::named("metadata", &metadata)
        )
    }

    /// https://stripe.com/docs/api#list_fee_refunds
    pub fn list_fee_refunds(
        &self,
        fee_id: &str,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<FeeRefund>> {
        self.get(&format!("/application_fees/{}/refunds", fee_id), &args)
    }

    /// https://stripe.com/docs/api#retrieve_application_fee
    pub fn retrieve_application_fee(&self, fee_id: &str) -> Result<ApplicationFee> {
        self.get(&format!("/application_fees/{}", fee_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#list_application_fees
    pub fn list_application_fees(
        &self,
        created_constraint: Option<TimeConstraint>,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<ApplicationFee>> {
        self.get("/application_fees", &(
            TOTAL_COUNT_PARAM,
            UrlEncodable::named("created", &created_constraint),
            args
        ))
    }

    /// https://stripe.com/docs/api#create_recipient
    pub fn create_recipient(
        &self,
        args: BTreeMap<String, String>,
        bank_account_token_or_args: Option<Either<String, BTreeMap<String, String>>>,
        card_token_or_args: Option<Either<String, BTreeMap<String, String>>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<Recipient> {
        self.post("/recipients", &(
            args,
            bank_account_token_or_args.map(|b| token_or_args("bank_account", b)),
            card_token_or_args.map(|c| token_or_args("card", c)),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_recipient
    pub fn retrieve_recipient(
        &self,
        recipient_id: &str
    ) -> Result<Recipient> {
        self.get(&format!("/recipients/{}", recipient_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_recipient
    pub fn update_recipient(
        &self,
        recipient_id: &str,
        args: Option<BTreeMap<String, String>>,
        bank_account_token_or_args: Option<Either<String, BTreeMap<String, String>>>,
        card_token_or_args: Option<Either<String, BTreeMap<String, String>>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<Recipient> {
        self.post(&format!("/recipients/{}", recipient_id), &(
            args,
            bank_account_token_or_args.map(|b| token_or_args("bank_account", b)),
            card_token_or_args.map(|c| token_or_args("card", c)),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#delete_recipient
    pub fn delete_recipient(
        &self,
        recipient_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/recipients/{}", recipient_id))
    }

    /// https://stripe.com/docs/api#list_recipients
    pub fn list_recipients(
        &self,
        created_constraint: Option<TimeConstraint>,
        args: Option<BTreeMap<String, String>>,
    ) -> Result<ApiList<Recipient>> {
        self.get("/recipients", &(
            TOTAL_COUNT_PARAM,
            args,
            UrlEncodable::named("created", &created_constraint)
        ))
    }

    /// https://stripe.com/docs/api#list_country_specs
    pub fn list_country_specs(
        &self,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<CountrySpec>> {
        self.get("/country_specs", &args)
    }

    /// https://stripe.com/docs/api#retrieve_country_spec
    pub fn retrieve_country_spec(
        &self,
        country_id: &str
    ) -> Result<CountrySpec> {
        self.get(&format!("/country_specs/{}", country_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#account_create_bank_account
    pub fn account_create_bank_account(
        &self,
        account_id: &str,
        bank_account_token_or_args: Either<String, BTreeMap<String, String>>,
        default_for_currency: bool,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<BankAccount> {
        self.post(&format!("/accounts/{}/external_accounts", account_id), &(
            token_or_args("external_account", bank_account_token_or_args),
            ("default_for_currency", default_for_currency.to_string()),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#account_retrieve_bank_account
    pub fn account_retrieve_bank_account(
        &self,
        account_id: &str,
        bank_account_id: &str
    ) -> Result<BankAccount> {
        self.get(
            &format!("/accounts/{}/external_accounts/{}", account_id, bank_account_id),
            NO_PARAMS
        )
    }

    /// https://stripe.com/docs/api#account_update_bank_account
    pub fn account_update_bank_account(
        &self,
        account_id: &str,
        bank_account_id: &str,
        args: BTreeMap<String, String>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<BankAccount> {
        self.post(&format!("/accounts/{}/external_accounts/{}", account_id, bank_account_id), &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
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
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<BankAccount>> {
        self.get(&format!("/accounts/{}/external_accounts", account_id), &args)
    }

    /// https://stripe.com/docs/api#account_create_card
    pub fn account_create_card(
        &self,
        account_id: &str,
        card_token_or_args: Either<String, BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
        default_for_currency: bool
    ) -> Result<Card> {
        self.post(&format!("/accounts/{}/external_accounts", account_id), &(
            token_or_args("external_account", card_token_or_args),
            UrlEncodable::named("metadata", &metadata),
            ("default_for_currency", default_for_currency.to_string())
        ))
    }

    /// https://stripe.com/docs/api#account_retrieve_card
    pub fn account_retrieve_card(
        &self,
        account_id: &str,
        card_id: &str
    ) -> Result<Card> {
        self.get(&format!("/accounts/{}/cards/{}", account_id, card_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#account_update_card
    pub fn account_update_card(
        &self,
        account_id: &str,
        card_id: &str,
        args: BTreeMap<String, String>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<Card> {
        self.post(&format!("accounts/{}/external_accounts/{}", account_id, card_id), &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#account_delete_card
    pub fn account_delete_card(
        &self,
        account_id: &str,
        card_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("accounts/{}/external_accounts/{}", account_id, card_id))
    }

    /// https://stripe.com/docs/api#account_list_cards
    pub fn account_list_cards(
        &self,
        account_id: &str,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<Card>> {
        self.get(&format!("accounts/{}/external_accounts", account_id), &(
            TOTAL_COUNT_PARAM,
            ("object", "card"),
            args
        ))
    }

    /// https://stripe.com/docs/api#customer_create_bank_account
    pub fn customer_create_bank_account(
        &self,
        customer_id: &str,
        bank_account_token_or_args: Either<String, BTreeMap<String, String>>,
        default_for_currency: bool,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<BankAccount> {
        self.post(&format!("/customers/{}/sources", customer_id), &(
            token_or_args("source", bank_account_token_or_args),
            ("default_for_currency", default_for_currency.to_string()),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#customer_retrieve_bank_account
    pub fn customer_retrieve_bank_account(
        &self,
        customer_id: &str,
        bank_account_id: &str
    ) -> Result<BankAccount> {
        self.get(&format!("/customers/{}/sources/{}", customer_id, bank_account_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#customer_update_bank_account
    pub fn customer_update_bank_account(
        &self,
        customer_id: &str,
        bank_account_id: &str,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<BankAccount> {
        self.post(&format!("/customers/{}/sources/{}", customer_id, bank_account_id), &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#customer_delete_bank_account
    pub fn customer_delete_bank_account(
        &self,
        customer_id: &str,
        bank_account_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/customers/{}/sources/{}", customer_id, bank_account_id))
    }

    /// https://stripe.com/docs/api#customer_list_bank_accounts
    pub fn customer_list_bank_accounts(
        &self,
        customer_id: &str,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<BankAccount>> {
        self.get(&format!("/customers/{}/sources", customer_id), &(
            TOTAL_COUNT_PARAM,
            ("object", "card"),
            args
        ))
    }

    /// https://stripe.com/docs/api#create_bitcoin_receiver
    pub fn create_bitcoin_receiver(
        &self,
        args: BTreeMap<String, String>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<BitcoinReceiver> {
        self.post("/bitcoin/receivers", &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_bitcoin_receiver
    pub fn retrieve_bitcoin_receiver(
        &self,
        receiver_id: &str
    ) -> Result<BitcoinReceiver> {
        self.get(&format!("/bitcoin/receivers/{}", receiver_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#list_bitcoin_receivers
    pub fn list_bitcoin_receivers(
        &self,
        args: Option<BTreeMap<String, String>>,
    ) -> Result<ApiList<BitcoinReceiver>> {
        self.get("/bitcoin/receivers", &(
            args,
            TOTAL_COUNT_PARAM
        ))
    }

    /// https://stripe.com/docs/api#create_card
    pub fn create_card(
        &self,
        customer_id: &str,
        card_token_or_args: Either<String, BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<Card> {
        self.post(&format!("/customers/{}/sources", customer_id), &(
            token_or_args("source", card_token_or_args),
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_card
    pub fn retrieve_card(
        &self,
        customer_id: &str,
        card_id: &str
    ) -> Result<Card> {
        self.get(&format!("/customers/{}/sources/{}", customer_id, card_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_card
    pub fn update_card(
        &self,
        customer_id: &str,
        card_id: &str,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<Card> {
        self.post(&format!("/customers/{}/sources/{}", customer_id, card_id), &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#delete_card
    pub fn delete_card(
        &self,
        customer_id: &str,
        card_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/customers/{}/sources/{}", customer_id, card_id))
    }

    /// https://stripe.com/docs/api#list_cards
    pub fn list_cards(
        &self,
        customer_id: &str,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<Card>> {
        self.get(
            &format!("/customers/{}/sources", customer_id),
            &(args, TOTAL_COUNT_PARAM)
        )
    }

    /// https://stripe.com/docs/api#create_order
    pub fn create_order(
        &self,
        args: BTreeMap<String, String>,
        items: Option<Vec<OrderItem>>,
        metadata: Option<BTreeMap<String, String>>,
        shipping: Option<Shipping>
    ) -> Result<Order> {
        self.post("/orders", &(
            args,
            items.map(|items| UrlEncodable::structured_list("items", items)),
            UrlEncodable::named("metadata", &metadata),
            UrlEncodable::named("shipping", &shipping),
        ))
    }

    /// https://stripe.com/docs/api#retrieve_order
    pub fn retrieve_order(
        &self,
        order_id: &str
    ) -> Result<Order> {
        self.get(&format!("/orders/{}", order_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_order
    pub fn update_order(
        &self,
        order_id: &str,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<Order> {
        self.post(&format!("/orders/{}", order_id), &(
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#pay_order
    pub fn pay_order(
        &self,
        order_id: &str,
        customer: Option<String>,
        source: Option<Either<String, BTreeMap<String, String>>>,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>
    ) -> Result<Order> {
        self.post(&format!("/orders/{}/pay", order_id), &(
            customer.map(|customer| ("customer", customer)),
            source.map(|source| token_or_args("source", source)),
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#list_orders
    pub fn list_orders(
        &self,
        created_constraint: Option<TimeConstraint>,
        ids: Option<Vec<String>>,
        args: Option<BTreeMap<String, String>>,
        status_transitions: Option<BTreeMap<OrderStatus, TimeConstraint>>
    ) -> Result<ApiList<Order>> {
        self.get("/orders", &(
            TOTAL_COUNT_PARAM,
            UrlEncodable::named("created", &created_constraint),
            ids.map(|ids| UrlEncodable::list("ids", &ids)),
            args,
            status_transitions.map(|st| {
                UrlEncodable::named("status_transitions", &UrlEncodable::structured(&st))
            })
        ))
    }

    /// https://stripe.com/docs/api#create_product
    pub fn create_product(
        &self,
        name: String,
        attributes: Option<Vec<String>>,
        deactivate_on: Option<Vec<String>>,
        images: Option<Vec<String>>,
        metadata: Option<BTreeMap<String, String>>,
        dimensions: Option<Dimensions>,
        args: Option<BTreeMap<String, String>>
    ) -> Result<Product> {
        self.post("/products", &(
            ("name", name),
            attributes.map(|a| UrlEncodable::list("attributes", &a)),
            deactivate_on.map(|d| UrlEncodable::list("deactivate_on", &d)),
            images.map(|i| UrlEncodable::list("images", &i)),
            UrlEncodable::named("package_dimensions", &dimensions),
            UrlEncodable::named("metadata", &metadata),
            args
        ))
    }

    /// https://stripe.com/docs/api#retrieve_product
    pub fn retrieve_product(
        &self,
        product_id: &str
    ) -> Result<Product> {
        self.get(&format!("/products/{}", product_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_product
    pub fn update_product(
        &self,
        product_id: &str,
        args: Option<BTreeMap<String, String>>,
        deactivate_on: Option<Vec<String>>,
        images: Option<Vec<String>>,
        metadata: Option<BTreeMap<String, String>>,
        dimensions: Option<Dimensions>,
    ) -> Result<Product> {
        self.post(&format!("/products/{}", product_id), &(
            args,
            deactivate_on.map(|d| UrlEncodable::list("deactivate_on", &d)),
            images.map(|i| UrlEncodable::list("images", &i)),
            UrlEncodable::named("metadata", &metadata),
            UrlEncodable::named("package_dimensions", &dimensions),
        ))
    }

    /// https://stripe.com/docs/api#list_products
    pub fn list_products(
        &self,
        args: Option<BTreeMap<String, String>>,
        ids: Option<Vec<String>>,
    ) -> Result<ApiList<Product>> {
        self.get("/products", &(
            TOTAL_COUNT_PARAM,
            args,
            ids.map(|ids| UrlEncodable::list("ids", &ids))
        ))
    }

    /// https://stripe.com/docs/api#delete_product
    pub fn delete_product(
        &self,
        product_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/producst/{}", product_id))
    }

    /// https://stripe.com/docs/api#create_sku
    pub fn create_sku(
        &self,
        currency: &str,
        price: i64,
        product: &str,
        inventory: Inventory,
        attributes: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
        package_dimensions: Option<Dimensions>,
        args: Option<BTreeMap<String, String>>
    ) -> Result<Sku> {
        self.post("/skus", &(
            ("currency", currency),
            ("price", price.to_string()),
            ("product", product),
            UrlEncodable::named("inventory", &inventory),
            UrlEncodable::named("attributes", &attributes),
            UrlEncodable::named("metadata", &metadata),
            UrlEncodable::named("package_dimensions", &package_dimensions),
            args
        ))
    }

    /// https://stripe.com/docs/api#retrieve_sku
    pub fn retrieve_sku(
        &self,
        sku_id: &str
    ) -> Result<Sku> {
        self.get(&format!("/skus/{}", sku_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_sku
    pub fn update_sku(
        &self,
        sku_id: &str,
        args: Option<BTreeMap<String, String>>,
        inventory: Option<Inventory>,
        metadata: Option<BTreeMap<String, String>>,
        package_dimensions: Option<Dimensions>
    ) -> Result<Sku> {
        self.post(&format!("/skus/{}", sku_id), &(
            args,
            UrlEncodable::named("inventory", &inventory),
            UrlEncodable::named("metadata", &metadata),
            UrlEncodable::named("package_dimensions", &package_dimensions),
        ))
    }

    /// https://stripe.com/docs/api#list_skus
    pub fn list_skus(
        &self,
        attributes: Option<BTreeMap<String, String>>,
        ids: Option<Vec<String>>,
        args: Option<BTreeMap<String, String>>
    ) -> Result<ApiList<Sku>> {
        self.get("/skus", &(
            TOTAL_COUNT_PARAM,
            UrlEncodable::named("attributes", &attributes),
            ids.map(|ids| UrlEncodable::list("ids", &ids)),
            args
        ))
    }

    /// https://stripe.com/docs/api#delete_sku
    pub fn delete_sku(
        &self,
        sku_id: &str
    ) -> Result<Delete> {
        self.delete(&format!("/skus/{}", sku_id))
    }

    /// https://stripe.com/docs/api#create_coupon
    pub fn create_coupon(
        &self,
        duration: CouponDuration,
        args: Option<BTreeMap<String, String>>,
        metadata: Option<BTreeMap<String, String>>,
    ) -> Result<Coupon> {
        self.post("/coupons", &(
            ("duration", duration.to_string()),
            args,
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    /// https://stripe.com/docs/api#retrieve_coupon
    pub fn retrieve_coupon(
        &self,
        coupon_id: &str
    ) -> Result<Coupon> {
        self.get(&format!("/coupons/{}", coupon_id), NO_PARAMS)
    }

    /// https://stripe.com/docs/api#update_coupon
    pub fn update_coupon(
        &self,
        coupon_id: &str,
        metadata: BTreeMap<String, String>
    ) -> Result<Coupon> {
        self.post(&format!("/coupons/{}", coupon_id), &(
            UrlEncodable::named("metadata", &metadata)
        ))
    }

    pub fn get<T: Deserialize>(
        &self,
        endpoint: &str,
        args: &UrlEncodable
    ) -> Result<T> {
        let params = args.encoded_string();
        let url = if params.is_empty() {
            StripeClient::endpoint(endpoint)
        } else {
            StripeClient::endpoint(&format!("{}?{}", endpoint, params))
        };
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<T>(res)
    }

    pub fn post<T: Deserialize>(
        &self,
        endpoint: &str,
        args: &UrlEncodable,
    ) -> Result<T> {
        let body = args.encoded_string();
        let mut req_builder = self.client.post(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers());

        if !body.is_empty() {
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
            self.get(&list.url, &args)
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

fn token_or_args<T: UrlEncodable>(
    name: &str,
    token_or_args: Either<String, T>
) -> Vec<(String, String)> {
    match token_or_args {
        Left(token) => vec![(name.to_string(), token)],
        Right(args) => UrlEncodable::named(name, &args)
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

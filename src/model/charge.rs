use either::Either;
use errors::stripe_error::StripeErrorCode;
use serde::de::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::fmt;
use super::{
    Account, ApiList, ApplicationFee, BalanceTransaction, Currency, Customer, Dispute, Order,
    Refund, Shipping, Source, StripeObject, Transfer
};

#[derive(Clone, Debug, Deserialize)]
pub struct Charge {
    pub id: String,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application_fee: Option<Either<String, ApplicationFee>>,
    pub balance_transaction: Either<String, BalanceTransaction>,
    pub captured: bool,
    pub created: i64,
    pub currency: Currency,
    pub customer: Option<Either<String, Customer>>,
    pub description: Option<String>,
    pub destination: Option<Either<String, Account>>,
    pub dispute: Option<Dispute>,
    pub failure_code: Option<StripeErrorCode>,
    pub failure_message: Option<String>,
    pub fraud_details: Option<BTreeMap<String, String>>,
    pub invoice: Option<String>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub order: Option<Either<String, Order>>,
    pub paid: bool,
    pub receipt_email: String,
    pub receipt_number: String,
    pub refunded: bool,
    pub refunds: Option<ApiList<Refund>>,
    pub shipping: Option<Shipping>,
    pub source: Source,
    pub source_transfer: Option<Either<String, Transfer>>,
    pub statement_descriptor: Option<String>,
    pub status: ChargeStatus,
    pub transfer: Option<Either<String, Transfer>>
}

impl StripeObject for Charge {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum ChargeStatus {
    Succeeded,
    Pending,
    Failed,
    Unknown(String)
}

impl fmt::Display for ChargeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChargeStatus::Succeeded      => write!(f, "succeeded"),
            ChargeStatus::Pending        => write!(f, "pending"),
            ChargeStatus::Failed         => write!(f, "failed"),
            ChargeStatus::Unknown(ref s) => write!(f, "{}", s),
        }
    }
}

impl Deserialize for ChargeStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<ChargeStatus, D::Error>
        where D: Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "succeeded" => ChargeStatus::Succeeded,
            "pending"   => ChargeStatus::Pending,
            "failed"    => ChargeStatus::Failed,
            unknown     => ChargeStatus::Unknown(String::from(unknown)),
        })
    }
}

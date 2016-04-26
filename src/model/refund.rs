use serde;
use std::collections::BTreeMap;
use super::currency::Currency;
use super::StripeObject;

/// https://stripe.com/docs/api#refund_object
#[derive(Clone, Debug, Deserialize)]
pub struct Refund {
    pub id: String,
    pub amount: i64,
    pub balance_transaction: Option<String>,
    pub charge: String,
    pub created: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub reason: RefundReason,
    pub receipt_number: String
}

impl StripeObject for Refund {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    Unknown(String)
}

impl serde::Deserialize for RefundReason {
    fn deserialize<D>(deserializer: &mut D) -> Result<RefundReason, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "duplicate"             => RefundReason::Duplicate,
            "fraudulent"            => RefundReason::Fraudulent,
            "requested_by_customer" => RefundReason::RequestedByCustomer,
            unknown                 => RefundReason::Unknown(String::from(unknown)),
        })
    }
}

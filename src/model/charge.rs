use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::dispute::Dispute;
use super::refund::Refund;
use super::shipping::Shipping;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Charge {
    pub id: String,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application_fee: Option<String>,
    pub balance_transaction: String,
    pub captured: bool,
    pub created: i64,
    pub currency: String,
    pub customer: String,
    pub description: Option<String>,
    pub dispute: Option<Dispute>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub fraud_details: Option<BTreeMap<String, String>>,
    pub invoice: Option<String>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub order: Option<String>,
    pub paid: bool,
    pub receipt_email: String,
    pub receipt_number: String,
    pub refunded: bool,
    pub refunds: ApiList<Refund>,
    pub shipping: Option<Shipping>,
}

impl StripeObject for Charge {
    fn id(&self) -> &str {
        &self.id
    }
}

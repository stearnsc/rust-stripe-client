use either::Either;
use serde;
use std::collections::BTreeMap;
use std::fmt;
use super::api_list::ApiList;
use super::currency::Currency;
use super::charge::Charge;
use super::discount::Discount;
use super::period::Period;
use super::plan::Plan;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub amount_due: i64,
    pub application_fee: Option<i64>,
    pub attempt_count: i64,
    pub attempted: bool,
    pub charge: Either<String, Charge>,
    pub closed: bool,
    pub currency: Currency,
    pub customer: String,
    pub date: i64,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub ending_balance: Option<i64>,
    pub forgiven: bool,
    pub lines: ApiList<InvoiceLineItem>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub next_payment_attempt: Option<i64>,
    pub paid: bool,
    pub period_end: i64,
    pub period_start: i64,
    pub receipt_number: String,
    pub starting_balance: i64,
    pub statement_descriptor: Option<String>,
    pub subscription: Option<String>,
    pub subscription_proration_date: Option<i64>,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub tax_percent: Option<i64>,
    pub total: i64,
    pub webhooks_delivered_at: i64
}

impl StripeObject for Invoice {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct InvoiceLineItem {
    pub id: String,
    pub amount: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub discountable: bool,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub period: Period,
    pub plan: Option<Plan>,
    pub proration: bool,
    pub quantity: Option<i64>,
    pub subscription: Option<String>,
    #[serde(rename="type")]
    pub line_item_type: LineItemType,
}

impl StripeObject for InvoiceLineItem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum LineItemType {
    InvoiceItem,
    Subscription,
    Unknown(String),
}

impl fmt::Display for LineItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LineItemType::InvoiceItem    => write!(f, "invoiceitem"),
            LineItemType::Subscription   => write!(f, "subscription"),
            LineItemType::Unknown(ref s) => write!(f, "{}", s),
        }
    }
}

impl serde::Deserialize for LineItemType {
    fn deserialize<D>(deserializer: &mut D) -> Result<LineItemType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "invoiceitem"  => LineItemType::InvoiceItem,
            "subscription" => LineItemType::Subscription,
            unknown        => LineItemType::Unknown(String::from(unknown)),
        })
    }
}

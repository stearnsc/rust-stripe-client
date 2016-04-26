use std::collections::BTreeMap;
use super::currency::Currency;
use super::period::Period;
use super::plan::Plan;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Invoiceitem {
    pub id: String,
    pub amount: i64,
    pub currency: Currency,
    pub customer: String,
    pub date: i64,
    pub description: Option<String>,
    pub discountable: bool,
    pub invoice: String,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub period: Period,
    pub plan: Option<Plan>,
    pub proration: bool,
    pub quantity: Option<i64>,
    pub subscription: Option<String>
}

impl StripeObject for Invoiceitem {
    fn id(&self) -> &str {
        &self.id
    }
}

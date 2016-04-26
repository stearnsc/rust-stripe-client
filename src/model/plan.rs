use std::collections::BTreeMap;
use super::currency::Currency;
use super::interval::Interval;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Plan {
    pub id: String,
    pub amount: i64,
    pub created: i64,
    pub currency: Currency,
    pub interval: Interval,
    pub interval_count: i64,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub name: String,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<i64>
}

impl StripeObject for Plan {
    fn id(&self) -> &str {
        &self.id
    }
}

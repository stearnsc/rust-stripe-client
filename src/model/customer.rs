use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::discount::Discount;
use super::shipping::Shipping;
use super::source::Source;
use super::StripeObject;
use super::subscription::Subscription;

#[derive(Clone, Debug, Deserialize)]
pub struct Customer {
    id: String,
    account_balance: i64,
    created: i64,
    currency: String,
    default_source: Option<String>,
    delinquent: bool,
    description: Option<String>,
    discount: Option<Discount>,
    email: Option<String>,
    livemode: bool,
    metadata: Option<BTreeMap<String, String>>,
    shipping: Option<Shipping>,
    sources: Option<ApiList<Source>>,
    subscriptions: Option<ApiList<Subscription>>,
}

impl StripeObject for Customer {
    fn id(&self) -> &str {
        &self.id
    }
}

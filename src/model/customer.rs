use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::shipping::Shipping;
use super::source::Source;
use super::subscription::Subscription;
use super::discount::Discount;

#[derive(Debug, Clone, Deserialize)]
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

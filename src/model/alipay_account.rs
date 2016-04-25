use std::collections::BTreeMap;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct AlipayAccount {
    pub id: String,
    pub created: i64,
    pub fingerprint: String,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub payment_amount: i64,
    pub payment_currency: Option<String>,
    pub reusable: bool,
    pub used: bool,
    pub username: String,
}

impl StripeObject for AlipayAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

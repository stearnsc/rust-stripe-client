use std::collections::BTreeMap;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct FeeRefund {
    pub id: String,
    pub amount: i64,
    pub balance_transaction: Option<String>,
    pub created: i64,
    pub currency: String,
    pub fee: String,
    pub metadata: Option<BTreeMap<String, String>>,
}

impl StripeObject for FeeRefund {
    fn id(&self) -> &str {
        &self.id
    }
}

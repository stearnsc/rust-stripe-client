use std::collections::BTreeMap;
use super::currency::Currency;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct TransferReversal {
    pub id: String,
    pub amount: i64,
    pub balance_transaction: String,
    pub created: i64,
    pub currency: Currency,
    pub metadata: Option<BTreeMap<String, String>>,
    pub transfer: String,
}

impl StripeObject for TransferReversal {
    fn id(&self) -> &str {
        &self.id
    }
}

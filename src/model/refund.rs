use std::collections::BTreeMap;
use super::balance_transaction::BalanceTransaction;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
pub struct Refund {
    pub id: String,
    pub amount: i64,
    pub balance_transaction: Option<BalanceTransaction>,
    pub charge: String,
    pub created: i64,
    pub currency: String,
    pub description: Option<String>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub reason: String,
    pub receipt_number: String
}

impl StripeObject for Refund {
    fn id(&self) -> &str {
        &self.id
    }
}

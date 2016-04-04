use std::collections::BTreeMap;
use super::balance_transaction::BalanceTransaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

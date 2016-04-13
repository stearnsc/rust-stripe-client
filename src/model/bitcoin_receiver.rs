use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
pub struct BitcoinReceiver {
    pub id: String,
    pub active: bool,
    pub amount: i64,
    pub amount_received: i64,
    pub bitcoin_amount: i64,
    pub bitcoin_amount_received: i64,
    pub bitcoin_uri: String,
    pub created: i64,
    pub currency: String,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub filled: bool,
    pub inbound_address: String,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub payment: Option<String>,
    pub refund_address: Option<String>,
    pub transactions: Option<ApiList<BitcoinTransaction>>,
    pub uncaptured_funds: bool,
    pub used_for_payment: bool,
}

impl StripeObject for BitcoinReceiver {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitcoinTransaction {
    pub id: String,
    pub amount: i64,
    pub bitcoin_amount: i64,
    pub created: i64,
    pub currency: String,
    pub receiver: String
}

impl StripeObject for BitcoinTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

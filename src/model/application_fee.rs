use super::api_list::ApiList;
use super::fee_refund::FeeRefund;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationFee {
    pub id: String,
    pub account: String,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application: String,
    pub balance_transaction: String,
    pub charge: String,
    pub created: i64,
    pub livemode: bool,
    pub originating_transaction: Option<String>,
    pub refunded: bool,
    pub refunds: ApiList<FeeRefund>,
}

impl StripeObject for ApplicationFee {
    fn id(&self) -> &str {
        &self.id
    }
}

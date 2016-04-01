use super::api_list::ApiList;

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceTransaction {
    id: String,
    amount: i64,
    available_on: i64,
    created: i64,
    currency: String,
    description: String,
    fee: i64,
    fee_details: Vec<FeeDetails>,
    net: i64,
    souce: String,
    sourced_transfers: ApiList<SourcedTransfer>,
    status: String,
    #[serde(rename="type")]
    transaction_type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeDetails {
    amount: i64,
    application: String,
    currency: String,
    description: String,
    #[serde(rename="type")]
    fee_type: String,
}

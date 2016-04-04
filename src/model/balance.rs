use super::money::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub available: Vec<Money>,
    pub livemode: bool,
    pub pending: Vec<Money>
}

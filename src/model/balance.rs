use super::money::Money;

#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    pub available: Vec<Money>,
    pub livemode: bool,
    pub pending: Vec<Money>
}

use super::money::Money;

#[derive(Clone, Debug, Deserialize)]
pub struct Balance {
    pub available: Vec<Money>,
    pub livemode: bool,
    pub pending: Vec<Money>
}

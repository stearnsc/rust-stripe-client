use super::address::Address;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shipping {
    pub address: Address,
    pub carrier: String,
    pub name: String,
    pub phone: String,
    pub tracking_number: String
}

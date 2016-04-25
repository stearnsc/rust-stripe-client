use super::coupon::Coupon;

#[derive(Clone, Debug, Deserialize)]
pub struct Discount {
    pub coupon: Coupon,
    pub customer: String,
    pub end: Option<i64>,
    pub start: Option<i64>,
    pub subscription: Option<String>,
}

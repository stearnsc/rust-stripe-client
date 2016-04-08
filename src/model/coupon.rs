use serde;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Coupon {
    pub id: String,
    pub amount_off: Option<i64>,
    pub created: i64,
    pub currency: String,
    pub duration: CouponDuration,
    pub duration_in_months: Option<i64>,
    pub livemode: bool,
    pub max_redemptions: Option<i64>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub percent_off: i64,
    pub redeem_by: i64,
    pub times_redeemed: i64,
    pub valid: bool,
}

#[derive(Debug, Clone)]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
    Unknown(String),
}

impl serde::Deserialize for CouponDuration {
    fn deserialize<D>(deserializer: &mut D) -> Result<CouponDuration, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "forever"   => CouponDuration::Forever,
            "once"      => CouponDuration::Once,
            "repeating" => CouponDuration::Repeating,
            unknown     => CouponDuration::Unknown(String::from(unknown)),
        })
    }
}

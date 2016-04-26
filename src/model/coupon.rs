use serde;
use std::collections::BTreeMap;
use std::fmt;
use super::currency::Currency;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Coupon {
    pub id: String,
    pub amount_off: Option<i64>,
    pub created: i64,
    pub currency: Currency,
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

impl StripeObject for Coupon {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
    Unknown(String),
}

impl fmt::Display for CouponDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CouponDuration::Forever        => write!(f, "forever"),
            CouponDuration::Once           => write!(f, "once"),
            CouponDuration::Repeating      => write!(f, "repeating"),
            CouponDuration::Unknown(ref s) => write!(f, "{}", s),
        }
    }
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

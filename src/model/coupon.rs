use custom_ser::*;
use serde;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Coupon {
    id: String,
    amount_off: Option<i64>,
    created: i64,
    currency: String,
    duration: CouponDuration,
    duration_in_months: Option<i64>,
    livemode: bool,
    max_redemptions: Option<i64>,
    metadata: Option<BTreeMap<String, String>>,
    percent_off: i64,
    redeem_by: i64,
    times_redeemed: i64,
    valid: bool,
}

#[derive(Debug, Clone)]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
    Unknown(String),
}

impl CouponDuration {
    fn from_str(s: &str) -> CouponDuration {
        match s {
            "forever"   => CouponDuration::Forever,
            "once"      => CouponDuration::Once,
            "repeating" => CouponDuration::Repeating,
            unknown     => CouponDuration::Unknown(String::from(unknown)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            CouponDuration::Forever        => "forever",
            CouponDuration::Once           => "once",
            CouponDuration::Repeating      => "repeating",
            CouponDuration::Unknown(ref s) => s
        })
    }
}

simple_serde!(CouponDuration, CouponDuration::to_string, CouponDuration::from_str);

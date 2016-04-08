use custom_ser::*;
use serde;
use std::collections::BTreeMap;
use super::discount::Discount;
use super::interval::Interval;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub application_fee_percent: Option<f64>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub current_period_end: Option<i64>,
    pub current_period_start: Option<i64>,
    pub customer: String,
    pub discount: Discount,
    pub ended_at: Option<i64>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub plan: SubscriptionPlan,
    pub quantity: i64,
    pub start: i64,
    pub status: SubscriptionStatus,
    pub tax_percent: Option<f64>,
    pub trial_end: Option<i64>,
    pub trial_start: Option<i64>,
}

impl StripeObject for Subscription {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionPlan {
    pub id: String,
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub interval: Interval,
    pub interval_count: i64,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub name: String,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<i64>,
}

impl StripeObject for SubscriptionPlan {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Canceled,
    Unpaid,
    Unknown(String),
}

impl SubscriptionStatus {
    fn from_str(s: &str) -> SubscriptionStatus {
        match s {
            "trialing" => SubscriptionStatus::Trialing,
            "active"   => SubscriptionStatus::Active,
            "past_due" => SubscriptionStatus::PastDue,
            "canceled" => SubscriptionStatus::Canceled,
            "unpaid"   => SubscriptionStatus::Unpaid,
            unknown    => SubscriptionStatus::Unknown(String::from(unknown)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            SubscriptionStatus::Trialing       => "trialing",
            SubscriptionStatus::Active         => "active",
            SubscriptionStatus::PastDue        => "past_due",
            SubscriptionStatus::Canceled       => "canceled",
            SubscriptionStatus::Unpaid         => "unpaid",
            SubscriptionStatus::Unknown(ref s) => s,
        })
    }
}

simple_serde!(SubscriptionStatus, SubscriptionStatus::to_string, SubscriptionStatus::from_str);

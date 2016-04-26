use either::Either;
use serde;
use std::collections::BTreeMap;
use super::customer::Customer;
use super::discount::Discount;
use super::plan::Plan;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub application_fee_percent: Option<f64>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub current_period_end: Option<i64>,
    pub current_period_start: Option<i64>,
    pub customer: Either<String, Customer>,
    pub discount: Discount,
    pub ended_at: Option<i64>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub plan: Plan,
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

#[derive(Clone, Debug)]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Canceled,
    Unpaid,
    Unknown(String),
}

impl serde::Deserialize for SubscriptionStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<SubscriptionStatus, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "trialing" => SubscriptionStatus::Trialing,
            "active"   => SubscriptionStatus::Active,
            "past_due" => SubscriptionStatus::PastDue,
            "canceled" => SubscriptionStatus::Canceled,
            "unpaid"   => SubscriptionStatus::Unpaid,
            unknown    => SubscriptionStatus::Unknown(String::from(unknown)),
        })
    }
}

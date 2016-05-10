use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::currency::Currency;
use super::legal_entity::LegalEntity;
use super::StripeObject;
use super::verification::Verification;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub business_name: Option<String>,
    pub business_logo: Option<String>,
    pub business_url: Option<String>,
    pub charges_enabled: bool,
    pub country: String,
    pub debit_negative_balances: Option<bool>,
    pub decline_charge_on: Option<DeclineChargeOn>,
    pub default_currency: Currency,
    pub details_submitted: bool,
    pub display_name: Option<String>,
    pub email: String,
    pub external_accounts: Option<ApiList<ExternalAccount>>,
    pub legal_entity: Option<LegalEntity>,
    pub managed: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub product_description: Option<bool>,
    pub statement_descriptor: Option<String>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub timezone: String,
    pub tos_acceptance: Option<TosAcceptance>,
    pub transfer_schedule: Option<TransferSchedule>,
    pub transfers_enabled: bool,
    pub verification: Option<Verification>
}

impl StripeObject for Account {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeclineChargeOn {
    pub avs_failure: bool,
    pub cvs_failure: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExternalAccount {
    pub id: String,
    pub customer: String,
    pub account: String
}

impl StripeObject for ExternalAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TosAcceptance {
    date: i64,
    ip: Option<String>,
    user_agent: Option<String>
}

impl UrlEncodable for TosAcceptance {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let TosAcceptance { ref date, ref ip, ref user_agent } = *self;
        let mut v = vec![("date".to_string(), date.to_string())];
        if let Some(ref ip) = *ip {
            v.push(("ip".to_string(), ip.to_string()));
        }
        if let Some(ref user_agent) = *user_agent {
            v.push(("user_agent".to_string(), user_agent.to_string()));
        }
        v
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransferSchedule {
    delay_days: Option<i64>,
    interval: Option<String>,
    monthly_anchor: Option<i64>,
    weekly_anchor: Option<String>
}

impl UrlEncodable for TransferSchedule {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let TransferSchedule {
            ref delay_days, ref interval, ref monthly_anchor, ref weekly_anchor
        } = *self;

        let mut v = Vec::new();

        if let Some(ref delay_days) = *delay_days {
            v.push(("delay_days".to_string(), delay_days.to_string()));
        }

        if let Some(ref interval) = *interval {
            v.push(("interval".to_string(), interval.to_string()));
        }

        if let Some(ref monthly_anchor) = *monthly_anchor {
            v.push(("monthly_anchor".to_string(), monthly_anchor.to_string()));
        }

        if let Some(ref weekly_anchor) = *weekly_anchor {
            v.push(("weekly_anchor".to_string(), weekly_anchor.to_string()));
        }

        v
    }
}

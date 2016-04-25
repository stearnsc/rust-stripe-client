use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::legal_entity::LegalEntity;
use super::StripeObject;
use super::verification::Verification;

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
    pub default_currency: String,
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
    ip: String,
    user_agent: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransferSchedule {
    delay_days: i64,
    interval: String,
    monthly_anchor: i64,
    weekly_anchor: String
}


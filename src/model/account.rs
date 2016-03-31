use std::collections::BTreeMap;
use super::api_list::ApiList;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclineChargeOn {
    pub avs_failure: bool,
    pub cvs_failure: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalAccount {
    pub id: String,
    pub customer: String,
    pub account: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalEntity {
    #[serde(rename="type")]
    pub entity_type: String,
    pub address: Address,
    pub business_name: String,
    pub business_tax_id_provided: bool,
    pub dob: DateOfBirth,
    pub first_name: String,
    pub last_name: String,
    pub personal_address: Address,
    pub personal_id_number_provided: bool,
    pub ssn_last_4_provided: bool,
    pub verification: Verification,
    pub additional_owners: Vec<Owner>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub state: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateOfBirth {
    day: i64,
    month: i64,
    year: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    fields_needed: Vec<String>,
    due_by: i64,
    contacted: bool,
    disabled_reason: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    address: Address,
    dob: DateOfBirth,
    first_name: String,
    last_name: String,
    verification: Verification
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TosAcceptance {
    date: i64,
    ip: String,
    user_agent: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSchedule {
    delay_days: i64,
    interval: String,
    monthly_anchor: i64,
    weekly_anchor: String
}


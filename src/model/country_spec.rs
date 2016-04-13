use std::collections::BTreeMap;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
pub struct CountrySpec {
    pub id: String,
    pub default_currency: String,
    pub supported_bank_account_currencies: BTreeMap<String, Vec<String>>,
    pub supported_payment_currencies: Vec<String>,
    pub supported_payment_methods: Vec<String>,
    pub verification_fields: BTreeMap<String, VerificationFields>,
}

impl StripeObject for CountrySpec {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerificationFields {
    pub minimum: Vec<String>,
    pub additional: Vec<String>,
}

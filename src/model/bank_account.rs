use std::collections::BTreeMap;
use serde::ser::Error;
use serde;
use super::currency::Currency;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub account: Option<String>,
    pub account_holder_name: String,
    pub account_holder_type: AccountHolderType,
    pub bank_name: String,
    pub country: String,
    pub currency: Currency,
    pub fingerprint: String,
    pub last4: Option<String>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub routing_number: String,
    pub status: BankAccountStatus,
}

impl StripeObject for BankAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum AccountHolderType {
    Company,
    Individual,
    Unknown(String),
}

impl serde::Deserialize for AccountHolderType {
    fn deserialize<D>(deserializer: &mut D) -> Result<AccountHolderType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "company"    => AccountHolderType::Company,
            "individual" => AccountHolderType::Individual,
            unknown      => AccountHolderType::Unknown(String::from(unknown)),
        })
    }
}

impl serde::Serialize for AccountHolderType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            AccountHolderType::Company    => str::serialize("company", serializer),
            AccountHolderType::Individual => str::serialize("individual", serializer),
            _                             => Err(S::Error::invalid_value("expected [company, individual]")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BankAccountStatus {
    New,
    Validated,
    Verified,
    VerificationFailed,
    Errored,
    Unknown(String),
}

impl serde::Deserialize for BankAccountStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<BankAccountStatus, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "new"                 => BankAccountStatus::New,
            "validated"           => BankAccountStatus::Validated,
            "verified"            => BankAccountStatus::Verified,
            "verification_failed" => BankAccountStatus::VerificationFailed,
            "errored"             => BankAccountStatus::Errored,
            unknown               => BankAccountStatus::Unknown(String::from(unknown)),
        })
    }
}

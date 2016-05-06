use std::collections::BTreeMap;
use serde::ser::Error;
use serde;
use std::fmt;
use super::currency::Currency;
use super::StripeObject;
use url_encodable::UrlEncodable;

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

impl fmt::Display for AccountHolderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountHolderType::Company => write!(f, "company"),
            AccountHolderType::Individual => write!(f, "individual"),
            AccountHolderType::Unknown(ref unknown) => write!(f, "{}", unknown),
        }
    }
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

#[derive(Clone, Debug)]
pub struct NewBankAccount {
    pub account_number: String,
    pub country: String,
    pub currency: Currency,
    pub routing_number: Option<String>,
    pub account_holder_name: Option<String>,
    pub account_holder_type: Option<AccountHolderType>
}

impl NewBankAccount {
    pub fn new(account_number: String, country: String, currency: Currency) -> NewBankAccount {
        NewBankAccount {
            account_number: account_number,
            country: country,
            currency: currency,
            routing_number: None,
            account_holder_name: None,
            account_holder_type: None
        }
    }

    pub fn routing_number(mut self, routing_number: String) -> Self {
        self.routing_number = Some(routing_number);
        self
    }

    pub fn account_holder_name(mut self, account_holder_name: String) -> Self {
        self.account_holder_name = Some(account_holder_name);
        self
    }

    pub fn account_holder_type(mut self, account_holder_type: AccountHolderType) -> Self {
        self.account_holder_type = Some(account_holder_type);
        self
    }
}

impl UrlEncodable for NewBankAccount {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let NewBankAccount {
            ref account_number, ref country, ref currency, ref routing_number,
            ref account_holder_name, ref account_holder_type,
        } = *self;

        let mut pairs = vec![
            ("account_number".to_string(), account_number.to_string()),
            ("country".to_string(), country.to_string()),
            ("currency".to_string(), currency.to_string())
        ];

        if let Some(ref routing_number) = *routing_number {
            pairs.push(("routing_number".to_string(), routing_number.to_string()))
        }

        if let Some(ref account_holder_name) = *account_holder_name {
            pairs.push(("account_holder_name".to_string(), account_holder_name.to_string()))
        }

        if let Some(ref account_holder_type) = *account_holder_type {
            pairs.push(("account_holder_type".to_string(), account_holder_type.to_string()))
        }

        pairs
    }
}


















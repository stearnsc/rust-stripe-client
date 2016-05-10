use serde;
use serde::de::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub enum AccountRejectReason {
    Fraud,
    TermsOfService,
    Other
}

impl serde::Deserialize for AccountRejectReason {
    fn deserialize<D>(deserializer: &mut D) -> Result<AccountRejectReason, D::Error>
        where D: serde::Deserializer
    {
        match String::deserialize(deserializer)?.as_ref() {
            "fraud"            => Ok(AccountRejectReason::Fraud),
            "terms_of_service" => Ok(AccountRejectReason::TermsOfService),
            "other"            => Ok(AccountRejectReason::Other),
            _                  => Err(D::Error::invalid_value("Account Reject Reason must be one of [fraud, terms_of_service, other]"))
        }
    }
}

impl fmt::Display for AccountRejectReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountRejectReason::Fraud          => write!(f, "fraud"),
            AccountRejectReason::TermsOfService => write!(f, "terms_of_service"),
            AccountRejectReason::Other          => write!(f, "other"),
        }
    }
}

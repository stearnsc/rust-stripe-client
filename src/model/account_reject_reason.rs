use custom_ser::*;
use serde;

#[derive(Debug, Clone)]
pub enum AccountRejectReason {
    Fraud,
    TermsOfService,
    Other
}

impl AccountRejectReason {
    fn to_string(&self) -> String {
        String::from(match *self {
            AccountRejectReason::Fraud          => "fraud",
            AccountRejectReason::TermsOfService => "terms_of_service",
            AccountRejectReason::Other          => "other"
        })
    }

    fn from_str<E>(s: &str) -> Result<AccountRejectReason, E>
        where E: serde::de::Error
    {
        match s {
            "fraud"            => Ok(AccountRejectReason::Fraud),
            "terms_of_service" => Ok(AccountRejectReason::TermsOfService),
            "other"            => Ok(AccountRejectReason::Other),
            _                  => Err(E::invalid_value("Account Reject Reason must be one of [fraud, terms_of_service, other]"))
        }
    }
}

custom_string_serde!(AccountRejectReason, AccountRejectReason::to_string, AccountRejectReason::from_str);

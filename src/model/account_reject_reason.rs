use serde;
use serde::de::Error;

#[derive(Debug, Clone)]
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

impl serde::Serialize for AccountRejectReason {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            AccountRejectReason::Fraud          => str::serialize("fraud", serializer),
            AccountRejectReason::TermsOfService => str::serialize("terms_of_service", serializer),
            AccountRejectReason::Other          => str::serialize("other", serializer),
        }
    }
}

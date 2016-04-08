use std;
use serde;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StripeErrorWrapper {
    pub error: StripeError
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StripeError {
    #[serde(rename="type")]
    pub kind: StripeErrorKind,
    pub message: Option<String>,
    pub code: Option<StripeErrorCode>,
    pub param: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StripeErrorKind {
    ApiConnectionError,
    ApiError,
    AuthenticationError,
    CardError,
    InvalidRequestError,
    RateLimitError,
    Unknown(String)
}

impl serde::Deserialize for StripeErrorKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<StripeErrorKind, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "api_connection_error"  => StripeErrorKind::ApiConnectionError,
            "api_error"             => StripeErrorKind::ApiError,
            "authentication_error"  => StripeErrorKind::AuthenticationError,
            "card_error"            => StripeErrorKind::CardError,
            "invalid_request_error" => StripeErrorKind::InvalidRequestError,
            "rate_limit_error"      => StripeErrorKind::RateLimitError,
            unknown_kind            => StripeErrorKind::Unknown(String::from(unknown_kind))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StripeErrorCode {
    InvalidNumber,
    InvalidExpiryMonth,
    InvalidExpiryYear,
    InvalidCvC,
    IncorrectNumber,
    ExpiredCard,
    IncorrectCvc,
    IncorrectZip,
    CardDeclined,
    Missing,
    ProcessingError,
    Unknown(String)
}

impl serde::Deserialize for StripeErrorCode {
    fn deserialize<D>(deserializer: &mut D) -> Result<StripeErrorCode, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "invalid_number"       => StripeErrorCode::InvalidNumber,
            "invalid_expiry_month" => StripeErrorCode::InvalidExpiryMonth,
            "invalid_expirty_year" => StripeErrorCode::InvalidExpiryYear,
            "invalid_cvc"          => StripeErrorCode::InvalidCvC,
            "incorrect_number"     => StripeErrorCode::IncorrectNumber,
            "expired_card"         => StripeErrorCode::ExpiredCard,
            "incorrect_cvc"        => StripeErrorCode::IncorrectCvc,
            "incorrect_zip"        => StripeErrorCode::IncorrectZip,
            "card_declined"        => StripeErrorCode::CardDeclined,
            "missing"              => StripeErrorCode::Missing,
            "processing_error"     => StripeErrorCode::ProcessingError,
            unknown_code           => StripeErrorCode::Unknown(String::from(unknown_code))
        })
    }
}

impl std::error::Error for StripeError {
    fn description(&self) -> &str {
        match self.kind {
            StripeErrorKind::ApiConnectionError => "Unable to connect to Stripe's API",
            StripeErrorKind::ApiError => "An unexpected error has occurred",
            StripeErrorKind::AuthenticationError => "Invalid or missing authentication",
            StripeErrorKind::CardError => "The supplied card cannot be charged",
            StripeErrorKind::InvalidRequestError => "Invalid parameters in request",
            StripeErrorKind::RateLimitError => "Rate limit reached",
            StripeErrorKind::Unknown(ref msg) => msg
        }
    }
}

impl std::fmt::Display for StripeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "StripeError(kind={:?},message={:?},code={:?},param={:?})",
            self.kind,
            self.message,
            self.code,
            self.param
        )
    }
}

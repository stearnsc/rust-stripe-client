use std;
use serde;
use custom_ser::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StripeErrorWrapper {
    pub error: StripeError
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl StripeErrorKind {
    fn from_str<E>(s: &str) -> Result<StripeErrorKind, E>
        where E: serde::de::Error
    {
        Ok(match s {
            "api_connection_error"  => StripeErrorKind::ApiConnectionError,
            "api_error"             => StripeErrorKind::ApiError,
            "authentication_error"  => StripeErrorKind::AuthenticationError,
            "card_error"            => StripeErrorKind::CardError,
            "invalid_request_error" => StripeErrorKind::InvalidRequestError,
            "rate_limit_error"      => StripeErrorKind::RateLimitError,
            unknown_kind            => StripeErrorKind::Unknown(String::from(unknown_kind))
        })
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            StripeErrorKind::ApiConnectionError   => "api_connection_error",
            StripeErrorKind::ApiError             => "api_error",
            StripeErrorKind::AuthenticationError  => "authentication_error",
            StripeErrorKind::CardError            => "card_error",
            StripeErrorKind::InvalidRequestError  => "invalid_request_error",
            StripeErrorKind::RateLimitError       => "rate_limit_error",
            StripeErrorKind::Unknown(ref unknown) => unknown
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

impl StripeErrorCode {
    fn from_str<E>(s: &str) -> Result<StripeErrorCode, E>
        where E: serde::de::Error
    {
        Ok(match s {
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

    fn to_string(&self) -> String {
        String::from(match *self {
            StripeErrorCode::InvalidNumber      => "invalid_number",
            StripeErrorCode::InvalidExpiryMonth => "invalid_expiry_month",
            StripeErrorCode::InvalidExpiryYear  => "invalid_expirty_year",
            StripeErrorCode::InvalidCvC         => "invalid_cvc",
            StripeErrorCode::IncorrectNumber    => "incorrect_number",
            StripeErrorCode::ExpiredCard        => "expired_card",
            StripeErrorCode::IncorrectCvc       => "incorrect_cvc",
            StripeErrorCode::IncorrectZip       => "incorrect_zip",
            StripeErrorCode::CardDeclined       => "card_declined",
            StripeErrorCode::Missing            => "missing",
            StripeErrorCode::ProcessingError    => "processing_error",
            StripeErrorCode::Unknown(ref s)     => s
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

custom_string_serde!(StripeErrorKind, StripeErrorKind::to_string, StripeErrorKind::from_str);
custom_string_serde!(StripeErrorCode, StripeErrorCode::to_string, StripeErrorCode::from_str);

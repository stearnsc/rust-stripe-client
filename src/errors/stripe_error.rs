use std;
use serde_json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripeError {
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
            "StripeError(Kind={:?},message={:?},code={:?},param={:?})",
            self.kind,
            self.message,
            self.code,
            self.param
        )
    }
}

pub fn parse(s: &str) -> Result<StripeError, serde_json::error::Error> {
    let error = serde_json::from_str::<ErrorRaw>(s)?.error;

    let error_kind = match error.error_type.as_ref() {
        "api_connection_error"  => StripeErrorKind::ApiConnectionError,
        "api_error"             => StripeErrorKind::ApiError,
        "authentication_error"  => StripeErrorKind::AuthenticationError,
        "card_error"            => StripeErrorKind::CardError,
        "invalid_request_error" => StripeErrorKind::InvalidRequestError,
        "rate_limit_error"      => StripeErrorKind::RateLimitError,
        unknown_kind            => StripeErrorKind::Unknown(String::from(unknown_kind))
    };

    let error_code = error.code.map(|code| match code.as_ref() {
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
    });

    Ok(StripeError {
        kind: error_kind,
        message: error.message,
        code: error_code,
        param: error.param
    })
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ErrorRaw {
    error: StripeErrorRaw
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StripeErrorRaw {
    #[serde(rename="type")]
    error_type: String,
    message: Option<String>,
    code: Option<String>,
    param: Option<String>
}

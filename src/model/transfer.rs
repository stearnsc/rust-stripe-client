use serde;
use std::collections::BTreeMap;
use std::fmt;
use super::api_list::ApiList;
use super::currency::Currency;
use super::source_type::SourceType;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub amount: i64,
    pub amount_reversed: i64,
    pub application_fee: String,
    pub balance_transaction: String,
    pub created: i64,
    pub currency: Currency,
    pub date: i64,
    pub description: Option<String>,
    pub destination: String,
    pub destination_payment: Option<String>,
    pub failure_code: Option<TransferFailureCode>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub reversals: ApiList<Reversal>,
    pub reversed: bool,
    pub source_transaction: Option<String>,
    pub source_type: SourceType,
    pub statement_descriptor: Option<String>,
    pub status: TransferStatus,
    #[serde(rename="type")]
    pub transfer_type: TransferType
}

impl StripeObject for Transfer {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum TransferStatus {
    Paid,
    Pending,
    InTransit,
    Canceled,
    Failed,
    Other(String)
}

impl serde::Deserialize for TransferStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<TransferStatus, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "paid"       => TransferStatus::Paid,
            "pending"    => TransferStatus::Pending,
            "in_transit" => TransferStatus::InTransit,
            "canceled"   => TransferStatus::Canceled,
            "failed"     => TransferStatus::Failed,
            other        => TransferStatus::Other(String::from(other)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum TransferType {
    Card,
    BankAccount,
    StripeAccount,
    Other(String)
}

impl serde::Deserialize for TransferType {
    fn deserialize<D>(deserializer: &mut D) -> Result<TransferType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "card"          => TransferType::Card,
            "bank_account"  => TransferType::BankAccount,
            "strip_account" => TransferType::StripeAccount,
            other           => TransferType::Other(String::from(other)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum TransferFailureCode {
    InsufficientFunds,
    AccountClosed,
    NoAccount,
    InvalidAccountNumber,
    DebitNotAuthorized,
    BankOwnershipChanged,
    AccountFrozen,
    CouldNotProcess,
    BankAccountRestricted,
    InvalidCurrency,
    Other(String),
}

impl fmt::Display for TransferFailureCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransferFailureCode::InsufficientFunds     => write!(f, "insufficient_funds"),
            TransferFailureCode::AccountClosed         => write!(f, "account_closed"),
            TransferFailureCode::NoAccount             => write!(f, "no_account"),
            TransferFailureCode::InvalidAccountNumber  => write!(f, "invalid_account_number"),
            TransferFailureCode::DebitNotAuthorized    => write!(f, "debit_not_authorized"),
            TransferFailureCode::BankOwnershipChanged  => write!(f, "bank_ownership_changed"),
            TransferFailureCode::AccountFrozen         => write!(f, "account_frozen"),
            TransferFailureCode::CouldNotProcess       => write!(f, "could_not_process"),
            TransferFailureCode::BankAccountRestricted => write!(f, "bank_account_restricted"),
            TransferFailureCode::InvalidCurrency       => write!(f, "invalid_currency"),
            TransferFailureCode::Other(ref other)      => write!(f, "{}", other),
        }
    }
}

impl serde::Deserialize for TransferFailureCode {
    fn deserialize<D>(deserializer: &mut D) -> Result<TransferFailureCode, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "insufficient_funds"      => TransferFailureCode::InsufficientFunds,
            "account_closed"          => TransferFailureCode::AccountClosed,
            "no_account"              => TransferFailureCode::NoAccount,
            "invalid_account_number"  => TransferFailureCode::InvalidAccountNumber,
            "debit_not_authorized"    => TransferFailureCode::DebitNotAuthorized,
            "bank_ownership_changed"  => TransferFailureCode::BankOwnershipChanged,
            "account_frozen"          => TransferFailureCode::AccountFrozen,
            "could_not_process"       => TransferFailureCode::CouldNotProcess,
            "bank_account_restricted" => TransferFailureCode::BankAccountRestricted,
            "invalid_currency"        => TransferFailureCode::InvalidCurrency,
            other                     => TransferFailureCode::Other(String::from(other)),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reversal {
    pub id: String,
    pub amount: i64,
    pub balance_transaction: String,
    pub created: i64,
    pub currency: Currency,
    pub metadata: BTreeMap<String, String>,
    pub transfer: String
}

impl StripeObject for Reversal {
    fn id(&self) -> &str {
        &self.id
    }
}

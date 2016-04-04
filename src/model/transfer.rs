use custom_ser::*;
use serde;
use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::source_type::SourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub amount: i64,
    pub amount_reversed: i64,
    pub application_fee: String,
    pub balance_transaction: String,
    pub created: i64,
    pub currency: String,
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

#[derive(Debug, Clone)]
pub enum TransferStatus {
    Paid,
    Pending,
    InTransit,
    Canceled,
    Failed,
    Other(String)
}

impl TransferStatus {
    fn from_str(s: &str) -> TransferStatus {
        match s {
            "paid"       => TransferStatus::Paid,
            "pending"    => TransferStatus::Pending,
            "in_transit" => TransferStatus::InTransit,
            "canceled"   => TransferStatus::Canceled,
            "failed"     => TransferStatus::Failed,
            other        => TransferStatus::Other(String::from(other))
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            TransferStatus::Paid         => "paid",
            TransferStatus::Pending      => "pending",
            TransferStatus::InTransit    => "in_transit",
            TransferStatus::Canceled     => "canceled",
            TransferStatus::Failed       => "failed",
            TransferStatus::Other(ref s) => s,
        })
    }
}

simple_serde!(TransferStatus, TransferStatus::to_string, TransferStatus::from_str);

#[derive(Debug, Clone)]
pub enum TransferType {
    Card,
    BankAccount,
    StripeAccount,
    Other(String)
}

impl TransferType {
    fn from_str(s: &str) -> TransferType {
        match s {
            "card"          => TransferType::Card,
            "bank_account"  => TransferType::BankAccount,
            "strip_account" => TransferType::StripeAccount,
            other           => TransferType::Other(String::from(other)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            TransferType::Card          => "card",
            TransferType::BankAccount   => "bank_account",
            TransferType::StripeAccount => "strip_account",
            TransferType::Other(ref s)  => s,
        })
    }
}

simple_serde!(TransferType, TransferType::to_string, TransferType::from_str);

#[derive(Debug, Clone)]
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

impl TransferFailureCode {
    fn from_str(s: &str) -> TransferFailureCode {
        match s {
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
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            TransferFailureCode::InsufficientFunds     => "insufficient_funds",
            TransferFailureCode::AccountClosed         => "account_closed",
            TransferFailureCode::NoAccount             => "no_account",
            TransferFailureCode::InvalidAccountNumber  => "invalid_account_number",
            TransferFailureCode::DebitNotAuthorized    => "debit_not_authorized",
            TransferFailureCode::BankOwnershipChanged  => "bank_ownership_changed",
            TransferFailureCode::AccountFrozen         => "account_frozen",
            TransferFailureCode::CouldNotProcess       => "could_not_process",
            TransferFailureCode::BankAccountRestricted => "bank_account_restricted",
            TransferFailureCode::InvalidCurrency       => "invalid_currency",
            TransferFailureCode::Other(ref s)          => s,
        })
    }
}

simple_serde!(TransferFailureCode, TransferFailureCode::to_string, TransferFailureCode::from_str);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reversal {
    pub id: String,
    pub amount: i64,
    pub balance_transaction: String,
    pub created: i64,
    pub currency: String,
    pub metadata: BTreeMap<String, String>,
    pub transfer: String
}

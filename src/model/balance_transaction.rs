use serde;
use std::fmt;
use super::api_list::ApiList;
use super::currency::Currency;
use super::StripeObject;
use super::transfer::Transfer;

#[derive(Clone, Debug, Deserialize)]
pub struct BalanceTransaction {
    pub id: String,
    pub amount: i64,
    pub available_on: i64,
    pub created: i64,
    pub currency: Currency,
    pub description: String,
    pub fee: i64,
    pub fee_details: Vec<FeeDetails>,
    pub net: i64,
    pub souce: String,
    pub sourced_transfers: ApiList<Transfer>,
    pub status: TransactionStatus,
    #[serde(rename="type")]
    pub transaction_type: TransactionType
}

impl StripeObject for BalanceTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeeDetails {
    pub amount: i64,
    pub application: String,
    pub currency: Currency,
    pub description: String,
    #[serde(rename="type")]
    pub fee_type: String,
}

#[derive(Clone, Debug)]
pub enum TransactionStatus {
    Available,
    Pending,
    Other(String)
}

impl serde::Deserialize for TransactionStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<TransactionStatus, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "available" => TransactionStatus::Available,
            "pending"   => TransactionStatus::Pending,
            other       => TransactionStatus::Other(String::from(other)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum TransactionType {
    ApplicationFee,
    ApplicationFeeRefund,
    Charge,
    Payment,
    PaymentRefund,
    Refund,
    Transfer,
    TransferCancel,
    TransferFailure,
    TransferRefund,
    Other(String)
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransactionType::ApplicationFee       => write!(f, "application_fee"),
            TransactionType::ApplicationFeeRefund => write!(f, "application_fee_refund"),
            TransactionType::Charge               => write!(f, "charge"),
            TransactionType::Payment              => write!(f, "payment"),
            TransactionType::PaymentRefund        => write!(f, "payment_refund"),
            TransactionType::Refund               => write!(f, "refund"),
            TransactionType::Transfer             => write!(f, "transfer"),
            TransactionType::TransferCancel       => write!(f, "transfer_cancel"),
            TransactionType::TransferFailure      => write!(f, "transfer_failure"),
            TransactionType::TransferRefund       => write!(f, "transfer_refund"),
            TransactionType::Other(ref unknown)   => write!(f, "{}", unknown),
        }
    }
}

impl serde::Deserialize for TransactionType {
    fn deserialize<D>(deserializer: &mut D) -> Result<TransactionType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "application_fee"        => TransactionType::ApplicationFee,
            "application_fee_refund" => TransactionType::ApplicationFeeRefund,
            "charge"                 => TransactionType::Charge,
            "payment"                => TransactionType::Payment,
            "payment_refund"         => TransactionType::PaymentRefund,
            "refund"                 => TransactionType::Refund,
            "transfer"               => TransactionType::Transfer,
            "transfer_cancel"        => TransactionType::TransferCancel,
            "transfer_failure"       => TransactionType::TransferFailure,
            "transfer_refund"        => TransactionType::TransferRefund,
            other                    => TransactionType::Other(String::from(other)),
        })
    }
}

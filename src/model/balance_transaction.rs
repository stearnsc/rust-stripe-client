use custom_ser::*;
use serde;
use super::api_list::ApiList;
use super::transfer::Transfer;

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceTransaction {
    pub id: String,
    pub amount: i64,
    pub available_on: i64,
    pub created: i64,
    pub currency: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct FeeDetails {
    pub amount: i64,
    pub application: String,
    pub currency: String,
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

impl TransactionStatus {
    fn from_str(s: &str) -> TransactionStatus {
        match s {
            "available" => TransactionStatus::Available,
            "pending"   => TransactionStatus::Pending,
            other       => TransactionStatus::Other(String::from(other))
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            TransactionStatus::Available => "available",
            TransactionStatus::Pending => "pending",
            TransactionStatus::Other(ref s) => s
        })
    }
}

simple_serde!(TransactionStatus, TransactionStatus::to_string, TransactionStatus::from_str);

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

impl TransactionType {
    fn from_str(s: &str) -> TransactionType {
        match s {
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
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            TransactionType::ApplicationFee       => "application_fee",
            TransactionType::ApplicationFeeRefund => "application_fee_refund",
            TransactionType::Charge               => "charge",
            TransactionType::Payment              => "payment",
            TransactionType::PaymentRefund        => "payment_refund",
            TransactionType::Refund               => "refund",
            TransactionType::Transfer             => "transfer",
            TransactionType::TransferCancel       => "transfer_cancel",
            TransactionType::TransferFailure      => "transfer_failure",
            TransactionType::TransferRefund       => "transfer_refund",
            TransactionType::Other(ref other)     => other,
        })
    }
}

simple_serde!(TransactionType, TransactionType::to_string, TransactionType::from_str);

use custom_ser::*;
use serde;
use std::collections::BTreeMap;
use super::balance_transaction::BalanceTransaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub id: String,
    pub amount: i64,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: String,
    pub created: i64,
    pub currency: String,
    pub evidence: DisputeEvidence,
    pub evidence_details: EvidenceDetails,
    pub is_charge_refundable: bool,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub reason: DisputeReason,
    pub status: DisputeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeEvidence {
    access_activity_log: Option<String>,
    billing_address: Option<String>,
    cancellation_policy: Option<String>,
    cancellation_policy_disclosure: Option<String>,
    cancellation_rebuttal: Option<String>,
    customer_communication: Option<String>,
    customer_email_address: Option<String>,
    customer_name: Option<String>,
    customer_purchase_ip: Option<String>,
    customer_signature: Option<String>,
    duplicate_charge_documentation: Option<String>,
    duplicate_charge_explanation: Option<String>,
    duplicate_charge_id: Option<String>,
    product_description: Option<String>,
    receipt: Option<String>,
    refund_policy: Option<String>,
    refund_policy_disclosure: Option<String>,
    refund_refusal_explanation: Option<String>,
    service_date: Option<String>,
    service_documentation: Option<String>,
    shipping_address: Option<String>,
    shipping_carrier: Option<String>,
    shipping_date: Option<String>,
    shipping_documentation: Option<String>,
    shipping_tracking_number: Option<String>,
    uncategorized_file: Option<String>,
    uncategorized_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceDetails {
    due_by: Option<i64>,
    has_evidence: bool,
    past_due: bool,
    submission_count: i64
}

#[derive(Clone, Debug)]
pub enum DisputeReason {
    Duplicate,
    Fraudulent,
    SubscriptionCanceled,
    ProductUnacceptable,
    ProductNotReceived,
    Unrecognized,
    CreditNotProcessed,
    IncorrectAccountDetails,
    InsufficientFunds,
    BankCannotProcess,
    DebitNotAuthorized,
    General,
    Other(String),
}

impl DisputeReason {
    fn from_str(s: &str) -> DisputeReason {
        match s {
            "duplicate"                 => DisputeReason::Duplicate,
            "fraudulent"                => DisputeReason::Fraudulent,
            "subscription_canceled"     => DisputeReason::SubscriptionCanceled,
            "product_unacceptable"      => DisputeReason::ProductUnacceptable,
            "product_not_received"      => DisputeReason::ProductNotReceived,
            "unrecognized"              => DisputeReason::Unrecognized,
            "credit_not_processed"      => DisputeReason::CreditNotProcessed,
            "incorrect_account_details" => DisputeReason::IncorrectAccountDetails,
            "insufficient_funds"        => DisputeReason::InsufficientFunds,
            "bank_cannot_process"       => DisputeReason::BankCannotProcess,
            "debit_not_authorized"      => DisputeReason::DebitNotAuthorized,
            "general"                   => DisputeReason::General,
            other                       => DisputeReason::Other(String::from(other))
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            DisputeReason::Duplicate               => "duplicate",
            DisputeReason::Fraudulent              => "fraudulent",
            DisputeReason::SubscriptionCanceled    => "subscription_canceled",
            DisputeReason::ProductUnacceptable     => "product_unacceptable",
            DisputeReason::ProductNotReceived      => "product_not_received",
            DisputeReason::Unrecognized            => "unrecognized",
            DisputeReason::CreditNotProcessed      => "credit_not_processed",
            DisputeReason::IncorrectAccountDetails => "incorrect_account_details",
            DisputeReason::InsufficientFunds       => "insufficient_funds",
            DisputeReason::BankCannotProcess       => "bank_cannot_process",
            DisputeReason::DebitNotAuthorized      => "debit_not_authorized",
            DisputeReason::General                 => "general",
            DisputeReason::Other(ref s)            => s
        })
    }
}

simple_serde!(DisputeReason, DisputeReason::to_string, DisputeReason::from_str);

#[derive(Clone, Debug)]
pub enum DisputeStatus {
    WarningNeedsResponse,
    WarningUnderReview,
    WarningClosed,
    NeedsResponse,
    ResponseDisabled,
    UnderReview,
    ChargeRefunded,
    Won,
    Lost,
    Other(String),
}

impl DisputeStatus {
    fn from_str(s: &str) -> DisputeStatus {
        match s {
            "warning_needs_response" => DisputeStatus::WarningNeedsResponse,
            "warning_under_review"   => DisputeStatus::WarningUnderReview,
            "warning_closed"         => DisputeStatus::WarningClosed,
            "needs_response"         => DisputeStatus::NeedsResponse,
            "response_disabled"      => DisputeStatus::ResponseDisabled,
            "under_review"           => DisputeStatus::UnderReview,
            "charge_refunded"        => DisputeStatus::ChargeRefunded,
            "won"                    => DisputeStatus::Won,
            "lost"                   => DisputeStatus::Lost,
            other                    => DisputeStatus::Other(String::from(other)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            DisputeStatus::WarningNeedsResponse => "warning_needs_response",
            DisputeStatus::WarningUnderReview   => "warning_under_review",
            DisputeStatus::WarningClosed        => "warning_closed",
            DisputeStatus::NeedsResponse        => "needs_response",
            DisputeStatus::ResponseDisabled     => "response_disabled",
            DisputeStatus::UnderReview          => "under_review",
            DisputeStatus::ChargeRefunded       => "charge_refunded",
            DisputeStatus::Won                  => "won",
            DisputeStatus::Lost                 => "lost",
            DisputeStatus::Other(ref s)         => s,
        })
    }
}

simple_serde!(DisputeStatus, DisputeStatus::to_string, DisputeStatus::from_str);

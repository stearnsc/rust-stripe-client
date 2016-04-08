use serde;
use std::collections::BTreeMap;
use super::balance_transaction::BalanceTransaction;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
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

impl StripeObject for Dispute {
    fn id(&self) -> &str {
        &self.id
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct DisputeEvidence {
    pub access_activity_log: Option<String>,
    pub billing_address: Option<String>,
    pub cancellation_policy: Option<String>,
    pub cancellation_policy_disclosure: Option<String>,
    pub cancellation_rebuttal: Option<String>,
    pub customer_communication: Option<String>,
    pub customer_email_address: Option<String>,
    pub customer_name: Option<String>,
    pub customer_purchase_ip: Option<String>,
    pub customer_signature: Option<String>,
    pub duplicate_charge_documentation: Option<String>,
    pub duplicate_charge_explanation: Option<String>,
    pub duplicate_charge_id: Option<String>,
    pub product_description: Option<String>,
    pub receipt: Option<String>,
    pub refund_policy: Option<String>,
    pub refund_policy_disclosure: Option<String>,
    pub refund_refusal_explanation: Option<String>,
    pub service_date: Option<String>,
    pub service_documentation: Option<String>,
    pub shipping_address: Option<String>,
    pub shipping_carrier: Option<String>,
    pub shipping_date: Option<String>,
    pub shipping_documentation: Option<String>,
    pub shipping_tracking_number: Option<String>,
    pub uncategorized_file: Option<String>,
    pub uncategorized_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
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

impl serde::Deserialize for DisputeReason {
    fn deserialize<D>(deserializer: &mut D) -> Result<DisputeReason, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
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
            other                       => DisputeReason::Other(String::from(other)),
        })
    }
}

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

impl serde::Deserialize for DisputeStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<DisputeStatus, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
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
        })
    }
}

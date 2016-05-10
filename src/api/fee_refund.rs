use call_args::CallArgs;
use model::{ApiList, FeeRefund};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateFeeRefundRequest<'a> {
    client: &'a StripeClient,
    application_fee_id: String,
    args: CallArgs
}

impl<'a> CreateFeeRefundRequest<'a> {
    pub fn new(client: &'a StripeClient, application_fee_id: String) -> CreateFeeRefundRequest<'a> {
        CreateFeeRefundRequest {
            client: client,
            application_fee_id: application_fee_id,
            args: CallArgs::new()
        }
    }

    pub fn amount(mut self, amount: i64) -> Self {
        self.args.add_arg("amount", amount);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<FeeRefund> for CreateFeeRefundRequest<'a> {
    fn call(self) -> Result<FeeRefund> {
        self.client.post(
            format!("/application_fees/{}/refunds", self.application_fee_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct RetrieveFeeRefundRequest<'a> {
    client: &'a StripeClient,
    fee_id: String,
    refund_id: String
}

impl<'a> RetrieveFeeRefundRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        fee_id: String,
        refund_id: String
    ) -> RetrieveFeeRefundRequest<'a> {
        RetrieveFeeRefundRequest {
            client: client,
            fee_id: fee_id,
            refund_id: refund_id
        }
    }
}

impl<'a> ApiCall<FeeRefund> for RetrieveFeeRefundRequest<'a> {
    fn call(self) -> Result<FeeRefund> {
        self.client.get(
            format!("/application_fees/{}/refunds/{}", self.fee_id, self.refund_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct UpdateFeeRefundRequest<'a> {
    client: &'a StripeClient,
    fee_id: String,
    refund_id: String,
    args: CallArgs
}

impl<'a> UpdateFeeRefundRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        fee_id: String,
        refund_id: String
    ) -> UpdateFeeRefundRequest<'a> {
        UpdateFeeRefundRequest {
            client: client,
            fee_id: fee_id,
            refund_id: refund_id,
            args: CallArgs
        }
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<OBJECT> for UpdateFeeRefundRequest<'a> {
    fn call(self) -> Result<OBJECT> {
        self.client.post(
            format!("/application_fees/{}/refunds/{}", self.fee_id, self.refund_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct ListFeeRefundsRequest<'a> {
    client: &'a StripeClient,
    fee_id: String,
    args: CallArgs
}

impl<'a> ListFeeRefundsRequest<'a> {
    pub fn new(client: &'a StripeClient, fee_id: String) -> ListFeeRefundsRequest<'a> {
        ListFeeRefundsRequest {
            client: client,
            fee_id: fee_id
        }
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }
}

impl<'a> ApiCall<ApiList<FeeRefund>> for ListFeeRefundsRequest<'a> {
    fn call(self) -> Result<ApiList<FeeRefund>> {
        self.client.get(format!("/application_fees/{}/refunds", self.fee_id), &self.args)
    }
}

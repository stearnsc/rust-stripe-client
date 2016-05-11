use call_args::CallArgs;
use model::{ApiList, FeeRefund};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateFeeRefundCall<'a> {
    client: &'a StripeClient,
    application_fee_id: String,
    args: CallArgs
}

impl<'a> CreateFeeRefundCall<'a> {
    pub fn new(client: &'a StripeClient, application_fee_id: String) -> CreateFeeRefundCall<'a> {
        CreateFeeRefundCall {
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

impl<'a> ApiCall<FeeRefund> for CreateFeeRefundCall<'a> {
    fn call(self) -> Result<FeeRefund> {
        self.client.post(
            format!("/application_fees/{}/refunds", self.application_fee_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct RetrieveFeeRefundCall<'a> {
    client: &'a StripeClient,
    fee_id: String,
    refund_id: String
}

impl<'a> RetrieveFeeRefundCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        fee_id: String,
        refund_id: String
    ) -> RetrieveFeeRefundCall<'a> {
        RetrieveFeeRefundCall {
            client: client,
            fee_id: fee_id,
            refund_id: refund_id
        }
    }
}

impl<'a> ApiCall<FeeRefund> for RetrieveFeeRefundCall<'a> {
    fn call(self) -> Result<FeeRefund> {
        self.client.get(
            format!("/application_fees/{}/refunds/{}", self.fee_id, self.refund_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct UpdateFeeRefundCall<'a> {
    client: &'a StripeClient,
    fee_id: String,
    refund_id: String,
    args: CallArgs
}

impl<'a> UpdateFeeRefundCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        fee_id: String,
        refund_id: String
    ) -> UpdateFeeRefundCall<'a> {
        UpdateFeeRefundCall {
            client: client,
            fee_id: fee_id,
            refund_id: refund_id,
            args: CallArgs::new()
        }
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<FeeRefund> for UpdateFeeRefundCall<'a> {
    fn call(self) -> Result<FeeRefund> {
        self.client.post(
            format!("/application_fees/{}/refunds/{}", self.fee_id, self.refund_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct ListFeeRefundsCall<'a> {
    client: &'a StripeClient,
    fee_id: String,
    args: CallArgs
}

impl<'a> ListFeeRefundsCall<'a> {
    pub fn new(client: &'a StripeClient, fee_id: String) -> ListFeeRefundsCall<'a> {
        ListFeeRefundsCall {
            client: client,
            fee_id: fee_id,
            args: CallArgs::from(("include[]", "total_count"))
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

impl<'a> ApiCall<ApiList<FeeRefund>> for ListFeeRefundsCall<'a> {
    fn call(self) -> Result<ApiList<FeeRefund>> {
        self.client.get(format!("/application_fees/{}/refunds", self.fee_id), &self.args)
    }
}

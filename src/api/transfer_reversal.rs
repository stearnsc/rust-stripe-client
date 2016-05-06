use call_args::CallArgs;
use model::{ApiList, TransferReversal};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateTransferReveralRequest<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    args: CallArgs
}

impl<'a> CreateTransferReveralRequest<'a> {
    pub fn new(client: &'a StripeClient, transfer_id: String) -> CreateTransferReveralRequest<'a> {
        CreateTransferReveralRequest {
            client: client,
            transfer_id: transfer_id
            args: CallArgs::new()
        }
    }

    pub fn amount(mut self, amount: i64) -> Self {
        self.args.add_arg("amount", amount);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_named("metadata", metadata);
        self
    }

    pub fn refund_application_fee(mut self, refund_application_fee: bool) -> Self {
        self.args.add_arg("refund_application_fee", refund_application_fee);
        self
    }
}

impl<'a> ApiCall<TransferReversal> for CreateTransferReveralRequest<'a> {
    fn call(self) -> Result<TransferReversal> {
        self.client.post(format!("/transfers/{}/reversals", self.transfer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveTransferReversalRequest<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    reversal_id: String,
}

impl<'a> RetrieveTransferReversalRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        transfer_id: String,
        reversal_id: String
    ) -> RetrieveTransferReversalRequest<'a> {
        RetrieveTransferReversalRequest {
            client: client,
            transfer_id: transfer_id,
            reversal_id: reversal_id
        }
    }
}

impl<'a> ApiCall<OBJECT> for RetrieveTransferReversalRequest<'a> {
    fn call(self) -> Result<OBJECT> {
        self.client.get(
            format!("/transfers/{}/reversals/{}", self.transfer_id, self.reversal_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct UpdateTransferReversalRequest<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    reversal_id: String,
    args: CallArgs
}

impl<'a> UpdateTransferReversalRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        transfer_id: String,
        reversal_id: String
    ) -> UpdateTransferReversalRequest<'a> {
        UpdateTransferReversalRequest {
            client: client,
            transfer_id: transfer_id,
            reversal_id: reversal_id,
            args: CallArgs::new()
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_named("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<TransferReversal> for UpdateTransferReversalRequest<'a> {
    fn call(self) -> Result<TransferReversal> {
        self.client.post(
            format!("/transfers/{}/reversals/{}", self.transfer_id, self.reversal_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct ListTransferReversalsRequest<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    args: CallArgs
}

impl<'a> ListTransferReversalsRequest<'a> {
    pub fn new(client: &'a StripeClient, transfer_id: String) -> ListTransferReversalsRequest<'a> {
        ListTransferReversalsRequest {
            client: client,
            transfer_id: String,
            args: CallArgs::new()
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

impl<'a> ApiCall<ApiList<TransferReversal>> for ListTransferReversalsRequest<'a> {
    fn call(self) -> Result<ApiList<TransferReversal>> {
        self.get(format!("/transfers/{}/reversals", self.transfer_id), &args)
    }
}

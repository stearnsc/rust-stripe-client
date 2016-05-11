use call_args::CallArgs;
use model::{ApiList, TransferReversal};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateTransferReversalCall<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    args: CallArgs
}

impl<'a> CreateTransferReversalCall<'a> {
    pub fn new(client: &'a StripeClient, transfer_id: String) -> CreateTransferReversalCall<'a> {
        CreateTransferReversalCall {
            client: client,
            transfer_id: transfer_id,
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
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn refund_application_fee(mut self, refund_application_fee: bool) -> Self {
        self.args.add_arg("refund_application_fee", refund_application_fee);
        self
    }
}

impl<'a> ApiCall<TransferReversal> for CreateTransferReversalCall<'a> {
    fn call(self) -> Result<TransferReversal> {
        self.client.post(format!("/transfers/{}/reversals", self.transfer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveTransferReversalCall<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    reversal_id: String,
}

impl<'a> RetrieveTransferReversalCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        transfer_id: String,
        reversal_id: String
    ) -> RetrieveTransferReversalCall<'a> {
        RetrieveTransferReversalCall {
            client: client,
            transfer_id: transfer_id,
            reversal_id: reversal_id
        }
    }
}

impl<'a> ApiCall<TransferReversal> for RetrieveTransferReversalCall<'a> {
    fn call(self) -> Result<TransferReversal> {
        self.client.get(
            format!("/transfers/{}/reversals/{}", self.transfer_id, self.reversal_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct UpdateTransferReversalCall<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    reversal_id: String,
    args: CallArgs
}

impl<'a> UpdateTransferReversalCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        transfer_id: String,
        reversal_id: String
    ) -> UpdateTransferReversalCall<'a> {
        UpdateTransferReversalCall {
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
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<TransferReversal> for UpdateTransferReversalCall<'a> {
    fn call(self) -> Result<TransferReversal> {
        self.client.post(
            format!("/transfers/{}/reversals/{}", self.transfer_id, self.reversal_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct ListTransferReversalsCall<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    args: CallArgs
}

impl<'a> ListTransferReversalsCall<'a> {
    pub fn new(client: &'a StripeClient, transfer_id: String) -> ListTransferReversalsCall<'a> {
        ListTransferReversalsCall {
            client: client,
            transfer_id: transfer_id,
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

impl<'a> ApiCall<ApiList<TransferReversal>> for ListTransferReversalsCall<'a> {
    fn call(self) -> Result<ApiList<TransferReversal>> {
        self.client.get(format!("/transfers/{}/reversals", self.transfer_id), &self.args)
    }
}

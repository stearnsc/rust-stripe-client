use call_args::CallArgs;
use model::{ApiList, Currency, SourceType, Transfer, TransferStatus};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateTransferCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateTransferCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        amount: i64,
        currency: Currency,
        destination: String
    ) -> CreateTransferCall<'a> {
        CreateTransferCall {
            client: client,
            args: CallArgs(vec![
                ("amount".to_string(), amount.to_string()),
                ("currency".to_string(), currency.to_string()),
                ("destination".to_string(), destination)
            ])
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

    pub fn source_transaction(mut self, source_transaction: String) -> Self {
        self.args.add_arg("source_transaction", source_transaction);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }

    pub fn source_type(mut self, source_type: SourceType) -> Self {
        self.args.add_arg("source_type", source_type);
        self
    }
}

impl<'a> ApiCall<Transfer> for CreateTransferCall<'a> {
    fn call(self) -> Result<Transfer> {
        self.client.post("/transfers", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveTransferCall<'a> {
    client: &'a StripeClient,
    transfer_id: String
}

impl<'a> RetrieveTransferCall<'a> {
    pub fn new(client: &'a StripeClient, transfer_id: String) -> RetrieveTransferCall<'a> {
        RetrieveTransferCall {
            client: client,
            transfer_id: transfer_id
        }
    }
}

impl<'a> ApiCall<Transfer> for RetrieveTransferCall<'a> {
    fn call(self) -> Result<Transfer> {
        self.client.get(format!("/transfers/{}", self.transfer_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateTransferCall<'a> {
    client: &'a StripeClient,
    transfer_id: String,
    args: CallArgs
}

impl<'a> UpdateTransferCall<'a> {
    pub fn new(client: &'a StripeClient, transfer_id: String) -> UpdateTransferCall<'a> {
        UpdateTransferCall {
            client: client,
            transfer_id: transfer_id,
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

impl<'a> ApiCall<Transfer> for UpdateTransferCall<'a> {
    fn call(self) -> Result<Transfer> {
        self.client.post(format!("transfers/{}", self.transfer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListTransfersCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListTransfersCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListTransfersCall<'a> {
        ListTransfersCall {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
        self
    }

    pub fn created_exact(mut self, created: i64) -> Self {
        self.args.add_arg("created", created);
        self
    }

    pub fn date(mut self, date: TimeConstraint) -> Self {
        self.args.add_object("date", date);
        self
    }

    pub fn date_exact(mut self, date: i64) -> Self {
        self.args.add_arg("date", date);
        self
    }

    pub fn destination(mut self, destination: String) -> Self {
        self.args.add_arg("destination", destination);
        self
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn recipient(mut self, recipient: String) -> Self {
        self.args.add_arg("recipient", recipient);
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }

    pub fn status(mut self, status: TransferStatus) -> Self {
        self.args.add_arg("status", status);
        self
    }
}

impl<'a> ApiCall<ApiList<Transfer>> for ListTransfersCall<'a> {
    fn call(self) -> Result<ApiList<Transfer>> {
        self.client.get("/transfers", &self.args)
    }
}











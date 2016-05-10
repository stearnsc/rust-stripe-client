use call_args::CallArgs;
use model::{ApiList, Refund, RefundReason};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateRefundRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> CreateRefundRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        charge_id: String,
    ) -> CreateRefundRequest<'a> {
        CreateRefundRequest {
            client: client
            args: CallArgs(vec![("charge".to_string(), charge)])
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

    pub fn reason(mut self, reason: RefundReason) -> Self {
        self.args.add_arg("reason", reason);
        self
    }

    pub fn refund_application_fee(mut self, refund_application_fee: bool) -> Self {
        self.args.add_arg("refund_application_fee", refund_application_fee);
        self
    }

    pub fn reverse_transfer(mut self, reverse_transfer: bool) -> Self {
        self.args.add_arg("reverse_transfer", reverse_transfer);
        self
    }
}

impl<'a> ApiCall<Refund> for CreateRefundRequest<'a> {
    fn call(self) -> Result<Refund> {
        self.client.post("/refunds", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveRefundRequest<'a> {
    client: &'a StripeClient,
    refund_id: String
}

impl<'a> RetrieveRefundRequest<'a> {
    pub fn new(client: &'a StripeClient, refund_id: String) -> RetrieveRefundRequest<'a> {
        RetrieveRefundRequest {
            client: client
            refund_id: refund_id
        }
    }
}

impl<'a> ApiCall<Refund> for RetrieveRefundRequest<'a> {
    fn call(self) -> Result<Refund> {
        self.client.get(format!("/refunds/{}", self.refund_id))
    }
}

#[derive(Debug)]
pub struct UpdateRefundRequest<'a> {
    client: &'a StripeClient,
    refund_id: String,
    args: CallArgs
}

impl<'a> UpdateRefundRequest<'a> {
    pub fn new(client: &'a StripeClient, refund_id: String) -> UpdateRefundRequest<'a> {
        UpdateRefundRequest {
            client: client
            refund_id: refund_id
            args: CallArgs::new()
        }
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Refund> for UpdateRefundRequest<'a> {
    fn call(self) -> Result<Refund> {
        self.client.post(format!("/refunds/{}", self.refund_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListRefundRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListRefundRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListRefundRequest<'a> {
        ListRefundRequest {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn charge(mut self, charge: String) -> Self {
        self.args.add_arg("charge", charge);
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

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }
}

impl<'a> ApiCall<ApiList<Refund>> for ListRefundRequest<'a> {
    fn call(self) -> Result<ApiList<Refund>> {
        self.client.get("/refunds", &self.args)
    }
}

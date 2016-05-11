use call_args::CallArgs;
use model::{ApiList, Refund, RefundReason};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateRefundCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> CreateRefundCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        charge_id: String,
    ) -> CreateRefundCall<'a> {
        CreateRefundCall {
            client: client,
            args: CallArgs(vec![("charge".to_string(), charge_id)])
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

impl<'a> ApiCall<Refund> for CreateRefundCall<'a> {
    fn call(self) -> Result<Refund> {
        self.client.post("/refunds", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveRefundCall<'a> {
    client: &'a StripeClient,
    refund_id: String
}

impl<'a> RetrieveRefundCall<'a> {
    pub fn new(client: &'a StripeClient, refund_id: String) -> RetrieveRefundCall<'a> {
        RetrieveRefundCall {
            client: client,
            refund_id: refund_id
        }
    }
}

impl<'a> ApiCall<Refund> for RetrieveRefundCall<'a> {
    fn call(self) -> Result<Refund> {
        self.client.get(format!("/refunds/{}", self.refund_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateRefundCall<'a> {
    client: &'a StripeClient,
    refund_id: String,
    args: CallArgs
}

impl<'a> UpdateRefundCall<'a> {
    pub fn new(client: &'a StripeClient, refund_id: String) -> UpdateRefundCall<'a> {
        UpdateRefundCall {
            client: client,
            refund_id: refund_id,
            args: CallArgs::new()
        }
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Refund> for UpdateRefundCall<'a> {
    fn call(self) -> Result<Refund> {
        self.client.post(format!("/refunds/{}", self.refund_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListRefundCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListRefundCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListRefundCall<'a> {
        ListRefundCall {
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

impl<'a> ApiCall<ApiList<Refund>> for ListRefundCall<'a> {
    fn call(self) -> Result<ApiList<Refund>> {
        self.client.get("/refunds", &self.args)
    }
}

use call_args::CallArgs;
use model::{ApiList, ApplicationFee};
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct RetrieveApplicationFeeRequest<'a> {
    client: &'a StripeClient,
    fee_id: String
}

impl<'a> RetrieveApplicationFeeRequest<'a> {
    pub fn new(client: &'a StripeClient, fee_id: String) -> RetrieveApplicationFeeRequest<'a> {
        RetrieveApplicationFeeRequest {
            client: client,
            fee_id: fee_id
        }
    }
}

impl<'a> ApiCall<ApplicationFee> for RetrieveApplicationFeeRequest<'a> {
    fn call(self) -> Result<ApplicationFee> {
        self.client.get(format!("/application_fees/{}", self.fee_id), &())
    }
}

#[derive(Debug)]
pub struct ListApplicationFeesRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListApplicationFeesRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListApplicationFeesRequest<'a> {
        ListApplicationFeesRequest {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn charge(mut self, charge: String) -> Self {
        self.args.add_arg("charge", charge);
        self
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
        self
    }

    pub fn created_exact(mut self, created: i64) -> Self {
        self.args.add_arg("created", created);
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

impl<'a> ApiCall<ApiList<ApplicationFee>> for ListApplicationFeesRequest<'a> {
    fn call(self) -> Result<ApiList<ApplicationFee>> {
        self.client.get("/application_fees", &self.args)
    }
}

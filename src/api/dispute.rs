use call_args::CallArgs;
use model::{ApiList, Delete, Dispute};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct RetrieveDisputeRequest<'a> {
    client: &'a StripeClient,
    dispute_id: String
}

impl<'a> RetrieveDisputeRequest<'a> {
    pub fn new(client: &'a StripeClient, dispute_id: String) -> RetrieveDisputeRequest<'a> {
        RetrieveDisputeRequest {
            client: client,
            dispute_id: dispute_id
        }
    }
}

impl<'a> ApiCall<Dispute> for RetrieveDisputeRequest<'a> {
    fn call(self) -> Result<Dispute> {
        self.client.get(format!("/disputes/{}", self.dispute_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateDisputeRequest<'a> {
    client: &'a StripeClient,
    dispute_id: String,
    args: CallArgs,
}

impl<'a> UpdateDisputeRequest<'a> {
    pub fn new(client: &'a StripeClient, dispute_id: String) -> UpdateDisputeRequest<'a> {
        UpdateDisputeRequest {
            client: client,
            dispute_id: dispute_id,
            args: CallArgs::new()
        }
    }

    pub fn evidence(mut self, evidence: BTreeMap<String, String>) -> Self {
        self.args.add_named("evidence", evidence);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_named("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Dispute> for UpdateDisputeRequest<'a> {
    fn call(self) -> Result<Dispute> {
        self.client.post(format!("/disputes/{}", self.dispute_id), &self.args)
    }
}

#[derive(Debug)]
pub struct CloseDisputeRequest<'a> {
    client: &'a StripeClient,
    dispute_id: String
}

impl<'a> CloseDisputeRequest<'a> {
    pub fn new(client: &'a StripeClient, dispute_id: String) -> CloseDisputeRequest<'a> {
        CloseDisputeRequest {
            client: client,
            dispute_id: dispute_id
        }
    }
}

impl<'a> ApiCall<Dispute> for CloseDisputeRequest<'a> {
    fn call(self) -> Result<Dispute> {
        self.client.post(format!("/disputes/{}/close", self.dispute_id), &())
    }
}

#[derive(Debug)]
pub struct ListDisputesRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListDisputesRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListDisputesRequest<'a> {
        ListDisputesRequest {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_named("created", created);
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

impl<'a> ApiCall<ApiList<Dispute>> for ListDisputesRequest<'a> {
    fn call(self) -> Result<ApiList<Dispute>> {
        self.client.get("/disputes", &self.args)
    }
}

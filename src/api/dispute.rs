use call_args::CallArgs;
use model::{ApiList, Dispute};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct RetrieveDisputeCall<'a> {
    client: &'a StripeClient,
    dispute_id: String
}

impl<'a> RetrieveDisputeCall<'a> {
    pub fn new(client: &'a StripeClient, dispute_id: String) -> RetrieveDisputeCall<'a> {
        RetrieveDisputeCall {
            client: client,
            dispute_id: dispute_id
        }
    }
}

impl<'a> ApiCall<Dispute> for RetrieveDisputeCall<'a> {
    fn call(self) -> Result<Dispute> {
        self.client.get(format!("/disputes/{}", self.dispute_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateDisputeCall<'a> {
    client: &'a StripeClient,
    dispute_id: String,
    args: CallArgs,
}

impl<'a> UpdateDisputeCall<'a> {
    pub fn new(client: &'a StripeClient, dispute_id: String) -> UpdateDisputeCall<'a> {
        UpdateDisputeCall {
            client: client,
            dispute_id: dispute_id,
            args: CallArgs::new()
        }
    }

    pub fn evidence(mut self, evidence: BTreeMap<String, String>) -> Self {
        self.args.add_object("evidence", evidence);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Dispute> for UpdateDisputeCall<'a> {
    fn call(self) -> Result<Dispute> {
        self.client.post(format!("/disputes/{}", self.dispute_id), &self.args)
    }
}

#[derive(Debug)]
pub struct CloseDisputeCall<'a> {
    client: &'a StripeClient,
    dispute_id: String
}

impl<'a> CloseDisputeCall<'a> {
    pub fn new(client: &'a StripeClient, dispute_id: String) -> CloseDisputeCall<'a> {
        CloseDisputeCall {
            client: client,
            dispute_id: dispute_id
        }
    }
}

impl<'a> ApiCall<Dispute> for CloseDisputeCall<'a> {
    fn call(self) -> Result<Dispute> {
        self.client.post(format!("/disputes/{}/close", self.dispute_id), &())
    }
}

#[derive(Debug)]
pub struct ListDisputesCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListDisputesCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListDisputesCall<'a> {
        ListDisputesCall {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
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

impl<'a> ApiCall<ApiList<Dispute>> for ListDisputesCall<'a> {
    fn call(self) -> Result<ApiList<Dispute>> {
        self.client.get("/disputes", &self.args)
    }
}

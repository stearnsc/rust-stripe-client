use call_args::CallArgs;
use model::{ApiList, Currency, Delete, Interval, Plan};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient, TimeConstraint};

#[derive(Debug)]
pub struct CreatePlanCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreatePlanCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        id: String,
        amount: i64,
        currency: Currency,
        interval: Interval,
        name: String
    ) -> CreatePlanCall<'a> {
        CreatePlanCall {
            client: client,
            args: CallArgs::from((
                ("id", id),
                ("amount", amount.to_string()),
                ("currency", currency.to_string()),
                ("interval", interval.to_string()),
                ("name", name)
            ))
        }
    }

    pub fn interval_count(mut self, interval_count: i64) -> Self {
        self.args.add_arg("interval_count", interval_count);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }

    pub fn trial_period_days(mut self, trial_period_days: i64) -> Self {
        self.args.add_arg("trial_period_days", trial_period_days);
        self
    }
}

impl<'a> ApiCall<Plan> for CreatePlanCall<'a> {
    fn call(self) -> Result<Plan> {
        self.client.post("/plans", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrievePlanCall<'a> {
    client: &'a StripeClient,
    plan_id: String
}

impl<'a> RetrievePlanCall<'a> {
    pub fn new(client: &'a StripeClient, plan_id: String) -> RetrievePlanCall<'a> {
        RetrievePlanCall {
            client: client,
            plan_id: plan_id
        }
    }
}

impl<'a> ApiCall<Plan> for RetrievePlanCall<'a> {
    fn call(self) -> Result<Plan> {
        self.client.get(format!("/plans/{}", self.plan_id), &())
    }
}

#[derive(Debug)]
pub struct UpdatePlanCall<'a> {
    client: &'a StripeClient,
    plan_id: String,
    args: CallArgs
}

impl<'a> UpdatePlanCall<'a> {
    pub fn new(client: &'a StripeClient, plan_id: String) -> UpdatePlanCall<'a> {
        UpdatePlanCall {
            client: client,
            plan_id: plan_id,
            args: CallArgs::new()
        }
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.args.add_arg("name", name);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }
}

impl<'a> ApiCall<Plan> for UpdatePlanCall<'a> {
    fn call(self) -> Result<Plan> {
        self.client.post(format!("/plans/{}", self.plan_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeletePlanCall<'a> {
    client: &'a StripeClient,
    plan_id: String
}

impl<'a> DeletePlanCall<'a> {
    pub fn new(client: &'a StripeClient, plan_id: String) -> DeletePlanCall<'a> {
        DeletePlanCall {
            client: client,
            plan_id: plan_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeletePlanCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/plans/{}", self.plan_id))
    }
}

#[derive(Debug)]
pub struct ListPlansCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListPlansCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListPlansCall<'a> {
        ListPlansCall {
            client: client,
            args: CallArgs::from(("include[]", "total_count"))
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

impl<'a> ApiCall<ApiList<Plan>> for ListPlansCall<'a> {
    fn call(self) -> Result<ApiList<Plan>> {
        self.client.get("/plans", &self.args)
    }
}

use call_args::CallArgs;
use model::{ApiList, Event};
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct RetrieveEventCall<'a> {
    client: &'a StripeClient,
    event_id: String
}

impl<'a> RetrieveEventCall<'a> {
    pub fn new(client: &'a StripeClient, event_id: String) -> RetrieveEventCall<'a> {
        RetrieveEventCall {
            client: client,
            event_id: event_id
        }
    }
}

impl<'a> ApiCall<Event> for RetrieveEventCall<'a> {
    fn call(self) -> Result<Event> {
        self.client.get(&format!("/events/{}", self.event_id), &())
    }
}

#[derive(Debug)]
pub struct ListEventCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListEventCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListEventCall<'a> {
        ListEventCall {
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

    pub fn event_type(mut self, event_type: String) -> Self {
        self.args.add_arg("type", event_type);
        self
    }
}

impl<'a> ApiCall<ApiList<Event>> for ListEventCall<'a> {
    fn call(self) -> Result<ApiList<Event>> {
        self.client.get("/events", &self.args)
    }
}

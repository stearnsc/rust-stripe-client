use call_args::CallArgs;
use model::{ApiList, NewCard, Subscription};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateSubscriptionRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> CreateSubscriptionRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        plan_id: String
    ) -> CreateSubscriptionRequest<'a> {
        CreateSubscriptionRequest {
            client: client,
            customer_id: customer_id,
            args: CallArgs::from(("plan", plan_id))
        }
    }

    pub fn application_fee_percent(mut self, application_fee_percent: f64) -> Self {
        self.args.add_arg("application_fee_percent", format!("{:.2}", application_fee_percent));
        self
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn source(mut self, source_id: String) -> Self {
        self.args.add_arg("source", source_id);
        self
    }

    pub fn card(mut self, card: NewCard) -> Self {
        self.args.add_object("source", card);
        self
    }

    pub fn quantity(mut self, quantity: i64) -> Self {
        self.args.add_arg("quantity", quantity);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn tax_percent(mut self, tax_percent: f64) -> Self {
        self.args.add_arg("tax_percent", format!("{:.2}", tax_percent));
        self
    }

    pub fn trial_end(mut self, trial_end: i64) -> Self {
        self.args.add_arg("trial_end", trial_end);
        self
    }
}

impl<'a> ApiCall<Subscription> for CreateSubscriptionRequest<'a> {
    fn call(self) -> Result<Subscription> {
        self.client.post(format!("/customers/{}/subscriptions", self.customer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveSubscriptionRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    subscription_id: String
}

impl<'a> RetrieveSubscriptionRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        subscription_id: String
    ) -> RetrieveSubscriptionRequest<'a> {
        RetrieveSubscriptionRequest {
            client: client,
            customer_id: customer_id,
            subscription_id: subscription_id
        }
    }
}

impl<'a> ApiCall<Subscription> for RetrieveSubscriptionRequest<'a> {
    fn call(self) -> Result<Subscription> {
        self.client.get(
            format!("/customers/{}/subscriptions/{}", self.customer_id, self.subscription_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct UpdateSubscriptionRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    subscription_id: String,
    args: CallArgs
}

impl<'a> UpdateSubscriptionRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        subscription_id: String
    ) -> UpdateSubscriptionRequest<'a> {
        UpdateSubscriptionRequest {
            client: client,
            customer_id: customer_id,
            subscription_id: subscription_id,
            args: CallArgs::new()
        }
    }

    pub fn application_fee_percent(mut self, application_fee_percent: f64) -> Self {
        self.args.add_arg("application_fee_percent", format!("{:.2}", application_fee_percent));
        self
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn plan(mut self, plan: String) -> Self {
        self.args.add_arg("plan", plan);
        self
    }

    pub fn prorate(mut self, prorate: bool) -> Self {
        self.args.add_arg("prorate", prorate);
        self
    }

    pub fn proration_date(mut self, proration_date: i64) -> Self {
        self.args.add_arg("proration_date", proration_date);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn quantity(mut self, quantity: i64) -> Self {
        self.args.add_arg("quantity", quantity);
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.args.add_arg("source", source);
        self
    }

    pub fn new_source(mut self, source: NewCard) -> Self {
        self.args.add_object("source", source);
        self
    }

    pub fn tax_percent(mut self, tax_percent: f64) -> Self {
        self.args.add_arg("tax_percent", format!("{:.2}", tax_percent));
        self
    }

    pub fn trial_end(mut self, trial_end: i64) -> Self {
        self.args.add_arg("trial_end", trial_end);
        self
    }

    pub fn trial_end_now(mut self) -> Self {
        self.args.add_arg("trial_end", "now");
        self
    }
}

impl<'a> ApiCall<Subscription> for UpdateSubscriptionRequest<'a> {
    fn call(self) -> Result<Subscription> {
        self.client.post(
            format!("/customers/{}/subscriptions/{}", self.customer_id, self.subscription_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct CancelSubscriptionRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    subscription_id: String,
    args: CallArgs
}

impl<'a> CancelSubscriptionRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        subscription_id: String
    ) -> CancelSubscriptionRequest<'a> {
        CancelSubscriptionRequest {
            client: client,
            customer_id: customer_id,
            subscription_id: subscription_id,
            args: CallArgs::new()
        }
    }

    pub fn at_period_end(mut self, at_period_end: bool) -> Self {
        self.args.add_arg("at_period_end", at_period_end);
        self
    }
}

impl<'a> ApiCall<Subscription> for CancelSubscriptionRequest<'a> {
    fn call(self) -> Result<Subscription> {
        self.client.delete_with_args(
            format!("/customers/{}/subscriptions/{}", self.customer_id, self.subscription_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct ListActiveSubscriptionsRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> ListActiveSubscriptionsRequest<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> ListActiveSubscriptionsRequest<'a> {
        ListActiveSubscriptionsRequest {
            client: client,
            customer_id: customer_id,
            args: CallArgs::from(("include[]", "total_count"))
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

impl<'a> ApiCall<ApiList<Subscription>> for ListActiveSubscriptionsRequest<'a> {
    fn call(self) -> Result<ApiList<Subscription>> {
        self.client.get(format!("/customers/{}/subscriptions", self.customer_id), &self.args)
    }
}

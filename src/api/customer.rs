use call_args::CallArgs;
use model::{ApiList, Customer, Delete, NewCard, Shipping};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateCustomerRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> CreateCustomerRequest<'a> {
    pub fn new(client: &'a StripeClient) -> CreateCustomerRequest<'a> {
        CreateCustomerRequest {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn account_balance(mut self, account_balance: i64) -> Self {
        self.args.add_arg("account_balance", account_balance);
        self
    }

    pub fn business_vat_id(mut self, business_vat_id: String) -> Self {
        self.args.add_arg("business_vat_id", business_vat_id);
        self
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.args.add_arg("email", email);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_named("metadata", metadata);
        self
    }

    pub fn plan(mut self, plan: String) -> Self {
        self.args.add_arg("plan", plan);
        self
    }

    pub fn quantity(mut self, quantity: i64) -> Self {
        self.args.add_arg("quantity", quantity);
        self
    }

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_named("shipping", shipping);
        self
    }

    pub fn source_token(mut self, source: String) -> Self {
        self.args.add_arg("source", source);
        self
    }

    pub fn source_card(mut self, source: NewCard) -> Self {
        self.args.add_named("source", source);
        self
    }

    pub fn tax_percent(mut self, tax_percent: f64) -> Self {
        self.args.add_arg("tax_percent", format!("{:.*}", 2, tax_percent));
        self
    }

    pub fn trial_end(mut self, trial_end: i64) -> Self {
        self.args.add_arg("trial_end", trial_end);
        self
    }
}

impl<'a> ApiCall<Customer> for CreateCustomerRequest<'a> {
    fn call(self) -> Result<Customer> {
        self.client.post("/customers", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveCustomerRequest<'a> {
    client: &'a StripeClient,
    customer_id: String
}

impl<'a> RetrieveCustomerRequest<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> RetrieveCustomerRequest<'a> {
        RetrieveCustomerRequest {
            client: client,
            customer_id: customer_id
        }
    }
}

impl<'a> ApiCall<Customer> for RetrieveCustomerRequest<'a> {
    fn call(self) -> Result<Customer> {
        self.client.get(format!("/customers/{}", self.customer_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateCustomerRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> UpdateCustomerRequest<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> UpdateCustomerRequest<'a> {
        UpdateCustomerRequest {
            client: client,
            customer_id: customer_id,
            args: CallArgs::new()
        }
    }

    pub fn account_balance(mut self, account_balance: String) -> Self {
        self.args.add_arg("account_balance", account_balance);
        self
    }

    pub fn business_vat_id(mut self, business_vat_id: String) -> Self {
        self.args.add_arg("business_vat_id", business_vat_id);
        self
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn default_source(mut self, default_source: String) -> Self {
        self.args.add_arg("default_source", default_source);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.args.add_arg("email", email);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_named("metadata", metadata);
        self
    }

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_named("shipping", shipping);
        self
    }

    pub fn source_token(mut self, source_token: String) -> Self {
        self.args.add_arg("source", source_token);
        self
    }

    pub fn source_card(mut self, source_card: NewCard) -> Self {
        self.args.add_named("source", source_card);
        self
    }
}

impl<'a> ApiCall<Customer> for UpdateCustomerRequest<'a> {
    fn call(self) -> Result<Customer> {
        self.client.post(format!("/customers/{}", self.customer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteCustomerRequest<'a> {
    client: &'a StripeClient,
    customer_id: String
}

impl<'a> DeleteCustomerRequest<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> DeleteCustomerRequest<'a> {
        DeleteCustomerRequest {
            client: client,
            customer_id: customer_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteCustomerRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/customers/{}", self.customer_id))
    }
}

#[derive(Debug)]
pub struct ListCustomersRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListCustomersRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListCustomersRequest<'a> {
        ListCustomersRequest {
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

impl<'a> ApiCall<ApiList<Customer>> for ListCustomersRequest<'a> {
    fn call(self) -> Result<ApiList<Customer>> {
        self.client.get("/customers", &self.args)
    }
}

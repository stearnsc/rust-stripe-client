use call_args::CallArgs;
use model::{ApiList, Invoice, InvoiceLineItem};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient, TimeConstraint};

#[derive(Debug)]
pub struct CreateInvoiceCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateInvoiceCall<'a> {
    pub fn new(client: &'a StripeClient, customer: String) -> CreateInvoiceCall<'a> {
        CreateInvoiceCall {
            client: client,
            args: CallArgs::from(("customer", customer))
        }
    }

    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.args.add_arg("application_fee", application_fee);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
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

    pub fn subscription(mut self, subscription: String) -> Self {
        self.args.add_arg("subscription", subscription);
        self
    }

    pub fn tax_percent(mut self, tax_percent: f64) -> Self {
        self.args.add_arg("tax_percent", tax_percent);
        self
    }
}

impl<'a> ApiCall<Invoice> for CreateInvoiceCall<'a> {
    fn call(self) -> Result<Invoice> {
        self.client.post("/invoices", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveInvoiceCall<'a> {
    client: &'a StripeClient,
    invoice_id: String
}

impl<'a> RetrieveInvoiceCall<'a> {
    pub fn new(client: &'a StripeClient, invoice_id: String) -> RetrieveInvoiceCall<'a> {
        RetrieveInvoiceCall {
            client: client,
            invoice_id: invoice_id
        }
    }
}

impl<'a> ApiCall<Invoice> for RetrieveInvoiceCall<'a> {
    fn call(self) -> Result<Invoice> {
        self.client.get(format!("/incoices/{}", self.invoice_id), &())
    }
}

#[derive(Debug)]
pub struct RetrieveInvoiceLinesCall<'a> {
    client: &'a StripeClient,
    invoice_id: String,
    args: CallArgs
}

impl<'a> RetrieveInvoiceLinesCall<'a> {
    pub fn new(client: &'a StripeClient, invoice_id: String) -> RetrieveInvoiceLinesCall<'a> {
        RetrieveInvoiceLinesCall {
            client: client,
            invoice_id: invoice_id,
            args: CallArgs::new()
        }
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
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

    pub fn subscription(mut self, subscription: String) -> Self {
        self.args.add_arg("subscription", subscription);
        self
    }

    pub fn subscription_plan(mut self, subscription_plan: String) -> Self {
        self.args.add_arg("subscription_plan", subscription_plan);
        self
    }

    pub fn subscription_prorate(mut self, subscription_prorate: bool) -> Self {
        self.args.add_arg("subscription_prorate", subscription_prorate);
        self
    }

    pub fn subscription_proration_date(mut self, subscription_proration_date: i64) -> Self {
        self.args.add_arg("subscription_proration_date", subscription_proration_date);
        self
    }

    pub fn subscription_quantity(mut self, subscription_quantity: String) -> Self {
        self.args.add_arg("subscription_quantity", subscription_quantity);
        self
    }

    pub fn subscription_trial_end(mut self, subscription_trial_end: i64) -> Self {
        self.args.add_arg("subscription_trial_end", subscription_trial_end);
        self
    }
}

impl<'a> ApiCall<ApiList<InvoiceLineItem>> for RetrieveInvoiceLinesCall<'a> {
    fn call(self) -> Result<ApiList<InvoiceLineItem>> {
        self.client.get(format!("/invoices/{}/lines", self.invoice_id), &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveUpcomingInvoiceCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> RetrieveUpcomingInvoiceCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> RetrieveUpcomingInvoiceCall<'a> {
        RetrieveUpcomingInvoiceCall {
            client: client,
            args: CallArgs::from(("customer", customer_id))
        }
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn subscription(mut self, subscription: String) -> Self {
        self.args.add_arg("subscription", subscription);
        self
    }

    pub fn subscription_plan(mut self, subscription_plan: String) -> Self {
        self.args.add_arg("subscription_plan", subscription_plan);
        self
    }

    pub fn subscription_prorate(mut self, subscription_prorate: bool) -> Self {
        self.args.add_arg("subscription_prorate", subscription_prorate);
        self
    }

    pub fn subscription_proration_date(mut self, subscription_proration_date: i64) -> Self {
        self.args.add_arg("subscription_proration_date", subscription_proration_date);
        self
    }

    pub fn subscription_quantity(mut self, subscription_quantity: i64) -> Self {
        self.args.add_arg("subscription_quantity", subscription_quantity);
        self
    }

    pub fn subscription_trial_end(mut self, subscription_trial_end: i64) -> Self {
        self.args.add_arg("subscription_trial_end", subscription_trial_end);
        self
    }
}

impl<'a> ApiCall<Invoice> for RetrieveUpcomingInvoiceCall<'a> {
    fn call(self) -> Result<Invoice> {
        self.client.get("/invoices/upcoming", &self.args)
    }
}

#[derive(Debug)]
pub struct UpdateInvoiceCall<'a> {
    client: &'a StripeClient,
    invoice_id: String,
    args: CallArgs
}

impl<'a> UpdateInvoiceCall<'a> {
    pub fn new(client: &'a StripeClient, invoice_id: String) -> UpdateInvoiceCall<'a> {
        UpdateInvoiceCall {
            client: client,
            invoice_id: invoice_id,
            args: CallArgs::new()
        }
    }

    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.args.add_arg("application_fee", application_fee);
        self
    }

    pub fn closed(mut self, closed: bool) -> Self {
        self.args.add_arg("closed", closed);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn forgiven(mut self, forgiven: bool) -> Self {
        self.args.add_arg("forgiven", forgiven);
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

    pub fn tax_percent(mut self, tax_percent: f64) -> Self {
        self.args.add_arg("tax_percent", tax_percent);
        self
    }
}

impl<'a> ApiCall<Invoice> for UpdateInvoiceCall<'a> {
    fn call(self) -> Result<Invoice> {
        self.client.post(format!("/invoices/{}", self.invoice_id), &self.args)
    }
}

#[derive(Debug)]
pub struct PayInvoiceCall<'a> {
    client: &'a StripeClient,
    invoice_id: String
}

impl<'a> PayInvoiceCall<'a> {
    pub fn new(client: &'a StripeClient, invoice_id: String) -> PayInvoiceCall<'a> {
        PayInvoiceCall {
            client: client,
            invoice_id: invoice_id
        }
    }
}

impl<'a> ApiCall<Invoice> for PayInvoiceCall<'a> {
    fn call(self) -> Result<Invoice> {
        self.client.post(format!("/invoices/{}/pay", self.invoice_id), &())
    }
}

#[derive(Debug)]
pub struct ListInvoicesCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListInvoicesCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListInvoicesCall<'a> {
        ListInvoicesCall {
            client: client,
            args: CallArgs::from(("include[]", "total_count"))
        }
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
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

impl<'a> ApiCall<ApiList<Invoice>> for ListInvoicesCall<'a> {
    fn call(self) -> Result<ApiList<Invoice>> {
        self.client.get("/invoices", &self.args)
    }
}

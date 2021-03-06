use call_args::CallArgs;
use model::{ApiList, Currency, Delete, Invoiceitem};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient, TimeConstraint};


#[derive(Debug)]
pub struct CreateInvoiceitemCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateInvoiceitemCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        amount: i64,
        currency: Currency,
        customer: String
    ) -> CreateInvoiceitemCall<'a> {
        CreateInvoiceitemCall {
            client: client,
            args: CallArgs::from((
                ("amount", amount.to_string()),
                ("currency", currency.to_string()),
                ("customer", customer)
            ))
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn discountable(mut self, discountable: bool) -> Self {
        self.args.add_arg("discountable", discountable);
        self
    }

    pub fn invoice(mut self, invoice: String) -> Self {
        self.args.add_arg("invoice", invoice);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn subscription(mut self, subscription: String) -> Self {
        self.args.add_arg("subscription", subscription);
        self
    }
}

impl<'a> ApiCall<Invoiceitem> for CreateInvoiceitemCall<'a> {
    fn call(self) -> Result<Invoiceitem> {
        self.client.post("/invoiceitems", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveInvoiceitemCall<'a> {
    client: &'a StripeClient,
    invoiceitem_id: String
}

impl<'a> RetrieveInvoiceitemCall<'a> {
    pub fn new(client: &'a StripeClient, invoiceitem_id: String) -> RetrieveInvoiceitemCall<'a> {
        RetrieveInvoiceitemCall {
            client: client,
            invoiceitem_id: invoiceitem_id
        }
    }
}

impl<'a> ApiCall<Invoiceitem> for RetrieveInvoiceitemCall<'a> {
    fn call(self) -> Result<Invoiceitem> {
        self.client.get(format!("/invoiceitems/{}", self.invoiceitem_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateInvoiceitemCall<'a> {
    client: &'a StripeClient,
    invoiceitem_id: String,
    args: CallArgs
}

impl<'a> UpdateInvoiceitemCall<'a> {
    pub fn new(client: &'a StripeClient, invoiceitem_id: String) -> UpdateInvoiceitemCall<'a> {
        UpdateInvoiceitemCall {
            client: client,
            invoiceitem_id: invoiceitem_id,
            args: CallArgs::new()
        }
    }

    pub fn amount(mut self, amount: i64) -> Self {
        self.args.add_arg("amount", amount);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn discountable(mut self, discountable: bool) -> Self {
        self.args.add_arg("discountable", discountable);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Invoiceitem> for UpdateInvoiceitemCall<'a> {
    fn call(self) -> Result<Invoiceitem> {
        self.client.post(format!("/invoiceitems/{}", self.invoiceitem_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteInvoiceitemCall<'a> {
    client: &'a StripeClient,
    invoiceitem_id: String
}

impl<'a> DeleteInvoiceitemCall<'a> {
    pub fn new(client: &'a StripeClient, invoiceitem_id: String) -> DeleteInvoiceitemCall<'a> {
        DeleteInvoiceitemCall {
            client: client,
            invoiceitem_id: invoiceitem_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteInvoiceitemCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/invoiceitems/{}", self.invoiceitem_id))
    }
}

#[derive(Debug)]
pub struct ListInvoiceitemsCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListInvoiceitemsCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListInvoiceitemsCall<'a> {
        ListInvoiceitemsCall {
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
}

impl<'a> ApiCall<ApiList<Invoiceitem>> for ListInvoiceitemsCall<'a> {
    fn call(self) -> Result<ApiList<Invoiceitem>> {
        self.client.get("/invoiceitems", &self.args)
    }
}

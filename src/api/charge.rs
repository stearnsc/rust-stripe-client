use call_args::CallArgs;
use hyper::header::Headers;
use idempotency_header::IdempotencyKey;
use model::{ApiList, Charge, Currency, Shipping, SourceType};
use std::collections::BTreeMap;
use time_constraint::TimeConstraint;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateChargeCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
    currency: Currency,
    idempotency_key: Option<String>
}

impl<'a> CreateChargeCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        amount: i64,
        currency: Currency,
    ) -> CreateChargeCall<'a> {
        CreateChargeCall {
            client: client,
            args: CallArgs(vec![
                ("amount".to_string(), amount.to_string()),
                ("currency".to_string(), currency.to_string()),
            ]),
            currency: currency,
            idempotency_key: None
        }
    }

    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.args.add_arg("application_fee", application_fee);
        self
    }


    pub fn capture(mut self, capture: bool) -> Self {
        self.args.add_arg("capture", capture);
        self
    }


    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }


    pub fn destination(mut self, destination: String) -> Self {
        self.args.add_arg("destination", destination);
        self
    }

    pub fn expand(mut self, expand: String) -> Self {
        self.args.add_arg("expand[]", expand);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn receipt_email(mut self, receipt_email: String) -> Self {
        self.args.add_arg("receipt_email", receipt_email);
        self
    }

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_object("shipping", shipping);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.args.add_arg("source", source);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }

    pub fn idempotency_key(mut self, idempotency_key: String) -> Self {
        self.idempotency_key = Some(idempotency_key);
        self
    }
}

impl<'a> ApiCall<Charge> for CreateChargeCall<'a> {
    fn call(self) -> Result<Charge> {
        let endpoint = "/charges";

        if let Some(idempotency_key) = self.idempotency_key {
            let mut headers = Headers::new();
            headers.set(IdempotencyKey::new(&idempotency_key));

            self.client.post_with_custom_headers(endpoint, &self.args, headers)
        } else {
            self.client.post(endpoint, &self.args)
        }
    }
}

#[derive(Debug)]
pub struct RetrieveChargeCall<'a> {
    client: &'a StripeClient,
    charge_id: String,
    args: CallArgs
}

impl<'a> RetrieveChargeCall<'a> {
    pub fn new(client: &'a StripeClient, charge_id: String) -> RetrieveChargeCall<'a> {
        RetrieveChargeCall {
            client: client,
            charge_id: charge_id,
            args: CallArgs::new()
        }
    }

    pub fn expand(mut self, expand: String) -> Self {
        self.args.add_arg("expand[]", expand);
        self
    }

}

impl<'a> ApiCall<Charge> for RetrieveChargeCall<'a> {
    fn call(self) -> Result<Charge> {
        self.client.get(&format!("/charges/{}", self.charge_id), &self.args)
    }
}

#[derive(Debug)]
pub struct UpdateChargeCall<'a> {
    client: &'a StripeClient,
    charge_id: String,
    args: CallArgs,
}

impl<'a> UpdateChargeCall<'a> {
    pub fn new(client: &'a StripeClient, charge_id: String) -> UpdateChargeCall<'a> {
        UpdateChargeCall {
            client: client,
            charge_id: charge_id,
            args: CallArgs::new()
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn expand(mut self, expand: String) -> Self {
        self.args.add_arg("expand[]", expand);
        self
    }

    pub fn fraud_details(mut self, fraud_details: BTreeMap<String, String>) -> Self {
        self.args.add_object("fraud_details", fraud_details);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn receipt_email(mut self, receipt_email: String) -> Self {
        self.args.add_arg("receipt_email", receipt_email);
        self
    }

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_object("shipping", shipping);
        self
    }
}

impl<'a> ApiCall<Charge> for UpdateChargeCall<'a> {
    fn call(self) -> Result<Charge> {
        self.client.post(format!("/charges/{}", self.charge_id), &self.args)
    }
}

#[derive(Debug)]
pub struct CaptureChargeCall<'a> {
    client: &'a StripeClient,
    charge_id: String,
    args: CallArgs,
}

impl<'a> CaptureChargeCall<'a> {
    pub fn new(client: &'a StripeClient, charge_id: String) -> CaptureChargeCall<'a> {
        CaptureChargeCall {
            client: client,
            charge_id: charge_id,
            args: CallArgs::new()
        }
    }

    pub fn amount(mut self, amount: i64) -> Self {
        self.args.add_arg("amount", amount);
        self
    }

    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.args.add_arg("application_fee", application_fee);
        self
    }

    pub fn expand(mut self, expand: String) -> Self {
        self.args.add_arg("expand[]", expand);
        self
    }

    pub fn receipt_email(mut self, receipt_email: String) -> Self {
        self.args.add_arg("receipt_email", receipt_email);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }
}

impl<'a> ApiCall<Charge> for CaptureChargeCall<'a> {
    fn call(self) -> Result<Charge> {
        self.client.post(format!("/charges/{}/capture", self.charge_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListChargesCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListChargesCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListChargesCall<'a> {
        ListChargesCall {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
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

    pub fn expand(mut self, expand: String) -> Self {
        self.args.add_arg("expand[]", expand);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn source(mut self, source: SourceType) -> Self {
        self.args.add_object("source", ("object", source.to_string()));
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }
}

impl<'a> ApiCall<ApiList<Charge>> for ListChargesCall<'a> {
    fn call(self) -> Result<ApiList<Charge>> {
        self.client.get("/charges", &self.args)
    }
}

use errors::error::Error;
use model::currency::Currency;
use model::Shipping;
use serde_json;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct CreateCharge {
    pub amount: i64,
    pub currency: Currency,
    pub application_fee: Option<i64>,
    pub capture: Option<bool>,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub receipt_email: Option<String>,
    pub shipping: Option<Shipping>,
    pub customer: Option<String>,
    pub source: Option<String>,
    pub statement_descriptor: Option<String>,
}

impl CreateCharge {
    pub fn from_customer(customer_id: &str, amount: i64, currency: &Currency) -> CreateCharge {
        CreateCharge {
            amount: amount,
            currency: currency.clone(),
            application_fee: None,
            capture: None,
            description: None,
            destination: None,
            metadata: None,
            receipt_email: None,
            shipping: None,
            customer: Some(String::from(customer_id)),
            source: None,
            statement_descriptor: None,
        }
    }

    pub fn from_source(source_id: &str, amount: i64, currency: &Currency) -> CreateCharge {
        CreateCharge {
            amount: amount,
            currency: currency.clone(),
            application_fee: None,
            capture: None,
            description: None,
            destination: None,
            metadata: None,
            receipt_email: None,
            shipping: None,
            customer: None,
            source: Some(String::from(source_id)),
            statement_descriptor: None,
        }
    }

    pub fn as_map(self) -> Result<BTreeMap<String, String>, Error> {
        let mut map: BTreeMap<&'static str, String> = BTreeMap::new();
        map.insert("amount", self.amount.to_string());
        map.insert("currency", self.currency.to_string());
        if let Some(application_fee) = self.application_fee {
            map.insert("application_fee", application_fee.to_string());
        }
        if let Some(capture) = self.capture {
            map.insert("capture", capture.to_string());
        }
        if let Some(description) = self.description {
            map.insert("description", description);
        }
        if let Some(destination) = self.destination {
            map.insert("destination", destination);
        }
        if let Some(receipt_email) = self.receipt_email {
            map.insert("receipt_email", receipt_email);
        }
        if let Some(customer) = self.customer {
            map.insert("customer", customer);
        }
        if let Some(source) = self.source {
            map.insert("source", source);
        }
        if let Some(statement_descriptor) = self.statement_descriptor {
            map.insert("statement_descriptor", statement_descriptor);
        }
        if let Some(shipping) = self.shipping {
            map.insert("shipping", serde_json::to_string(&shipping)?);
        }
        if let Some(metadata) = self.metadata {
            map.insert("metadata", serde_json::to_string(&metadata)?);
        }

        let map: BTreeMap<String, String> = map.into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect();

        Ok(map)
    }
}

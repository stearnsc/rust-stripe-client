use errors::error::Error;
use serde_json;
use std::collections::BTreeMap;
use super::Shipping;

pub struct UpdateCharge {
    description: Option<String>,
    fraud_details: Option<BTreeMap<String, String>>,
    metadata: Option<BTreeMap<String, String>>,
    receipt_email: Option<String>,
    shipping: Option<Shipping>,
}

impl UpdateCharge {
    pub fn new() -> UpdateCharge {
        UpdateCharge {
            description: None,
            fraud_details: None,
            metadata: None,
            receipt_email: None,
            shipping: None,
        }
    }

    pub fn as_map(self) -> Result<BTreeMap<String, String>, Error> {
        let mut map = BTreeMap::new();
        if let Some(description) = self.description {
            map.insert("description", description);
        }
        if let Some(fraud_details) = self.fraud_details {
            map.insert("fraud_details", serde_json::to_string(&fraud_details)?);
        }
        if let Some(metadata) = self.metadata {
            map.insert("metadata", serde_json::to_string(&metadata)?);
        }
        if let Some(receipt_email) = self.receipt_email {
            map.insert("receipt_email", receipt_email);
        }
        if let Some(shipping) = self.shipping {
            map.insert("shipping", serde_json::to_string(&shipping)?);
        }
        let map = map.into_iter().map(|(k, v)| (String::from(k), v)).collect();
        Ok(map)
    }
}

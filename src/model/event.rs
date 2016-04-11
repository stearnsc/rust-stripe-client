use serde::de::Error;
use serde;
use serde_json::value::Value;
use serde_json;
use std::collections::BTreeMap;
use super::StripeObject;

/// https://stripe.com/docs/api#event_object
#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    pub id: String,
    pub api_version: Option<String>,
    pub created: i64,
    pub data: EventData,
    pub livemode: bool,
    pub pending_webhooks: i64,
    pub request: Option<String>,
    #[serde(rename="type")]
    pub event_type: String,
}

impl StripeObject for Event {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventDataRaw {
    pub object: Value,
    pub previous_attributes: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct EventData {
    pub object: Value,
    pub id: String,
    pub object_name: String,
    pub previous_attributes: Option<BTreeMap<String, String>>,
}

impl EventData {
    pub fn parse<T: StripeObject>(self) -> Result<T, serde_json::error::Error> {
        serde_json::from_value::<T>(self.object)
    }
}

impl StripeObject for EventData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl serde::Deserialize for EventData {
    fn deserialize<D>(deserializer: &mut D) -> Result<EventData, D::Error>
        where D: serde::Deserializer
    {
        let EventDataRaw { object, previous_attributes } = EventDataRaw::deserialize(deserializer)?;
        let id = try!(match object.find("id") {
            Some(&Value::String(ref id)) => Ok(id.clone()),
            Some(_)                      => Err(D::Error::invalid_type(serde::de::Type::String)),
            None                         => Err(D::Error::missing_field("id"))
        });
        let object_name = try!(match object.find("object") {
            Some(&Value::String(ref object)) => Ok(object.clone()),
            Some(_)                          => Err(D::Error::invalid_type(serde::de::Type::String)),
            None                             => Err(D::Error::missing_field("object"))
        });

        Ok(EventData {
            object: object,
            id: id,
            object_name: object_name,
            previous_attributes: previous_attributes
        })
    }
}


use serde;
use super::address::Address;
use super::currency::Currency;
use super::StripeObject;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shipping {
    pub address: Address,
    pub carrier: Option<String>,
    pub name: String,
    pub phone: Option<String>,
    pub tracking_number: Option<String>
}

impl UrlEncodable for Shipping {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let mut vec = vec![];
        match *self { Shipping {
            ref address,
            ref carrier,
            ref name,
            ref phone,
            ref tracking_number
        } => {
            vec.extend(UrlEncodable::named("address", address));
            if let &Some(ref carrier) = carrier {
                vec.push(("carrier".to_string(), carrier.to_string()));
            }
            vec.push(("name".to_string(), name.to_string()));
            if let &Some(ref phone) = phone {
                vec.push(("phone".to_string(), phone.to_string()));
            }
            if let &Some(ref tracking_number) = tracking_number {
                vec.push(("tracking_number".to_string(), tracking_number.to_string()));
            }
        }}
        vec
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShippingMethod {
    pub id: String,
    pub amount: i64,
    pub currency: Currency,
    pub delivery_estimate: Option<DeliveryEstimate>,
    pub description: Option<String>,
}

impl StripeObject for ShippingMethod {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeliveryEstimate {
    pub date: Option<String>,
    pub earliest: Option<String>,
    pub latest: Option<String>,
    #[serde(rename="type")]
    pub estimate_type: DeliveryEstimateType,
}

#[derive(Clone, Debug)]
pub enum DeliveryEstimateType {
    Range,
    Exact,
    Unknown(String)
}

impl serde::Deserialize for DeliveryEstimateType {
    fn deserialize<D>(deserializer: &mut D) -> Result<DeliveryEstimateType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "range" => DeliveryEstimateType::Range,
            "exact" => DeliveryEstimateType::Exact,
            unknown => DeliveryEstimateType::Unknown(String::from(unknown)),
        })
    }
}

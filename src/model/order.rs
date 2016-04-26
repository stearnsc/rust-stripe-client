use serde;
use std::collections::BTreeMap;
use std::fmt;
use super::currency::Currency;
use super::shipping::{Shipping, ShippingMethod};
use super::StripeObject;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct Order {
    pub id: String,
    pub amount: i64,
    pub application: Option<String>,
    pub application_fee: Option<i64>,
    pub charge: Option<String>,
    pub created: i64,
    pub currency: Currency,
    pub customer: String,
    pub email: Option<String>,
    pub external_coupon_code: Option<String>,
    pub items: Vec<OrderItem>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub selected_shipping_method: Option<String>,
    pub shipping: Option<Shipping>,
    pub shipping_methods: Vec<ShippingMethod>,
    pub status: OrderStatus,
    pub status_transitions: Option<BTreeMap<OrderStatus, i64>>,
    pub updated: i64,
}

impl StripeObject for Order {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderItem {
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub quantity: Option<i64>,
    #[serde(rename="type")]
    pub item_type: Option<ItemType>
}

impl UrlEncodable for OrderItem {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let mut vec = Vec::new();
        let OrderItem {
            ref amount, ref currency, ref description, ref parent, ref quantity, ref item_type
        } = *self;
        if let Some(ref amount) = *amount {
            vec.push(("amount".to_string(), amount.to_string()));
        }
        if let Some(ref currency) = *currency {
            vec.push(("currency".to_string(), currency.to_string()));
        }
        if let Some(ref description) = *description {
            vec.push(("description".to_string(), description.to_string()));
        }
        if let Some(ref parent) = *parent {
            vec.push(("parent".to_string(), parent.to_string()));
        }
        if let Some(ref quantity) = *quantity {
            vec.push(("quantity".to_string(), quantity.to_string()));
        }
        if let Some(ref item_type) = *item_type {
            vec.push(("type".to_string(), item_type.to_string()));
        }
        vec
    }
}

#[derive(Clone, Debug)]
pub enum ItemType {
    Sku,
    Tax,
    Shipping,
    Discount,
    Unknown(String),
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ItemType::Sku                  => write!(f, "sku"),
            ItemType::Tax                  => write!(f, "tax"),
            ItemType::Shipping             => write!(f, "shipping"),
            ItemType::Discount             => write!(f, "discount"),
            ItemType::Unknown(ref unknown) => write!(f, "{}", unknown),
        }
    }
}

impl serde::Deserialize for ItemType {
    fn deserialize<D>(deserializer: &mut D) -> Result<ItemType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "sku"      => ItemType::Sku,
            "tax"      => ItemType::Tax,
            "shipping" => ItemType::Shipping,
            "discount" => ItemType::Discount,
            unknown    => ItemType::Unknown(String::from(unknown)),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderStatus {
    Created,
    Paid,
    Canceled,
    Fulfilled,
    Returned,
    Unknown(String),
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderStatus::Created        => write!(f, "created"),
            OrderStatus::Paid           => write!(f, "paid"),
            OrderStatus::Canceled       => write!(f, "canceled"),
            OrderStatus::Fulfilled      => write!(f, "fulfilled"),
            OrderStatus::Returned       => write!(f, "returned"),
            OrderStatus::Unknown(ref s) => write!(f, "{}", s),
        }
    }
}

impl serde::Deserialize for OrderStatus {
    fn deserialize<D>(deserializer: &mut D) -> Result<OrderStatus, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "created"   => OrderStatus::Created,
            "paid"      => OrderStatus::Paid,
            "canceled"  => OrderStatus::Canceled,
            "fulfilled" => OrderStatus::Fulfilled,
            "returned"  => OrderStatus::Returned,
            unknown     => OrderStatus::Unknown(String::from(unknown)),
        })
    }
}

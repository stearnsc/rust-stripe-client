use either::Either;
use serde;
use std::collections::BTreeMap;
use std::fmt;
use super::currency::Currency;
use super::product::Product;
use super::dimensions::Dimensions;
use super::StripeObject;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct Sku {
    pub id: String,
    pub active: bool,
    pub attributes: BTreeMap<String, String>,
    pub created: i64,
    pub currency: Currency,
    pub image: Option<String>,
    pub inventory: Inventory,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub package_dimensions: Option<Dimensions>,
    pub price: i64,
    pub product: Either<String, Product>,
    pub updated: i64
}

impl StripeObject for Sku {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Inventory {
    pub quantity: Option<i64>,
    #[serde(rename="type")]
    pub inventory_type: InventoryType,
    pub value: Option<InventoryState>
}

impl UrlEncodable for Inventory {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let mut v = vec![];
        if let Some(ref quantity) = self.quantity {
            v.push(("quantity".to_string(), quantity.to_string()));
        }
        v.push(("type".to_string(), self.inventory_type.to_string()));
        if let Some(ref value) = self.quantity {
            v.push(("value".to_string(), value.to_string()));
        }
        v
    }
}

#[derive(Clone, Debug)]
pub enum InventoryType {
    Finite,
    Bucket,
    Infinite,
    Unknown(String),
}

impl fmt::Display for InventoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InventoryType::Finite         => write!(f, "finite"),
            InventoryType::Bucket         => write!(f, "bucket"),
            InventoryType::Infinite       => write!(f, "infinite"),
            InventoryType::Unknown(ref s) => write!(f, "{}", s),
        }
    }
}

impl serde::Deserialize for InventoryType {
    fn deserialize<D>(deserializer: &mut D) -> Result<InventoryType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "finite"   => InventoryType::Finite,
            "bucket"   => InventoryType::Bucket,
            "infinite" => InventoryType::Infinite,
            unknown    => InventoryType::Unknown(String::from(unknown)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum InventoryState {
    InStock,
    Limited,
    OutOfStock,
    Unknown(String)
}

impl fmt::Display for InventoryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InventoryState::InStock        => write!(f, "in_stock"),
            InventoryState::Limited        => write!(f, "limited"),
            InventoryState::OutOfStock     => write!(f, "out_of_stock"),
            InventoryState::Unknown(ref s) => write!(f, "{}", s),
        }
    }
}

impl serde::Deserialize for InventoryState {
    fn deserialize<D>(deserializer: &mut D) -> Result<InventoryState, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "in_stock"     => InventoryState::InStock,
            "limited"      => InventoryState::Limited,
            "out_of_stock" => InventoryState::OutOfStock,
            unknown        => InventoryState::Unknown(String::from(unknown)),
        })
    }
}

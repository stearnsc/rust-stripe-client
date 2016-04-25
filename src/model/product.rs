use std::collections::BTreeMap;
use super::api_list::ApiList;
use super::dimensions::Dimensions;
use super::StripeObject;
use super::sku::Sku;

#[derive(Clone, Debug, Deserialize)]
pub struct Product {
    pub id: String,
    pub active: bool,
    pub attributes: Vec<String>,
    pub caption: Option<String>,
    pub created: i64,
    pub deactivate_on: Vec<String>,
    pub description: Option<String>,
    pub images: Option<Vec<String>>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub name: String,
    pub package_dimensions: Option<Dimensions>,
    pub shippable: bool,
    pub skus: ApiList<Sku>,
    pub updated: i64,
    pub url: Option<String>
}

impl StripeObject for Product {
    fn id(&self) -> &str {
        &self.id
    }
}

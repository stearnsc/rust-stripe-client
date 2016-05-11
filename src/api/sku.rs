use call_args::CallArgs;
use model::{ApiList, Currency, Delete, Dimensions, Inventory, Sku};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateSkuCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateSkuCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        currency: Currency,
        inventory: Inventory,
        price: i64,
        product: String
    ) -> CreateSkuCall<'a> {
        let mut args = CallArgs::new();
        args.add_arg("currency", currency);
        args.add_object("inventory", inventory);
        args.add_arg("price", price);
        args.add_arg("product", product);
        CreateSkuCall {
            client: client,
            args: args
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.args.add_arg("id", id);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.args.add_arg("active", active);
        self
    }

    pub fn attributes(mut self, attributes: BTreeMap<String, String>) -> Self {
        self.args.add_object("attributes", attributes);
        self
    }

    pub fn image(mut self, image: String) -> Self {
        self.args.add_arg("image", image);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn package_dimensions(mut self, package_dimensions: Dimensions) -> Self {
        self.args.add_object("package_dimensions", package_dimensions);
        self
    }
}

impl<'a> ApiCall<Sku> for CreateSkuCall<'a> {
    fn call(self) -> Result<Sku> {
        self.client.post("/skus", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveSkuCall<'a> {
    client: &'a StripeClient,
    sku_id: String
}

impl<'a> RetrieveSkuCall<'a> {
    pub fn new(client: &'a StripeClient, sku_id: String) -> RetrieveSkuCall<'a> {
        RetrieveSkuCall {
            client: client,
            sku_id: sku_id
        }
    }
}

impl<'a> ApiCall<Sku> for RetrieveSkuCall<'a> {
    fn call(self) -> Result<Sku> {
        self.client.get(format!("/sku_id/{}", self.sku_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateSkuCall<'a> {
    client: &'a StripeClient,
    sku_id: String,
    args: CallArgs
}

impl<'a> UpdateSkuCall<'a> {
    pub fn new(client: &'a StripeClient, sku_id: String) -> UpdateSkuCall<'a> {
        UpdateSkuCall {
            client: client,
            sku_id: sku_id,
            args: CallArgs::new()
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.args.add_arg("active", active);
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.args.add_arg("currency", currency);
        self
    }

    pub fn image(mut self, image: String) -> Self {
        self.args.add_arg("image", image);
        self
    }

    pub fn inventory(mut self, inventory: Inventory) -> Self {
        self.args.add_object("inventory", inventory);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn package_dimensions(mut self, package_dimensions: Dimensions) -> Self {
        self.args.add_object("package_dimensions", package_dimensions);
        self
    }

    pub fn price(mut self, price: i64) -> Self {
        self.args.add_arg("price", price);
        self
    }

    pub fn product(mut self, product: String) -> Self {
        self.args.add_arg("product", product);
        self
    }
}

impl<'a> ApiCall<Sku> for UpdateSkuCall<'a> {
    fn call(self) -> Result<Sku> {
        self.client.post(format!("/skus/{}", self.sku_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListSkusCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListSkusCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListSkusCall<'a> {
        ListSkusCall {
            client: client,
            args: CallArgs::from(("include[]", "total_count"))
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.args.add_arg("active", active);
        self
    }

    pub fn attributes(mut self, attributes: BTreeMap<String, String>) -> Self {
        self.args.add_object("attributes", attributes);
        self
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
        self
    }

    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.args.add_list("ids", ids);
        self
    }

    pub fn in_stock(mut self, in_stock: bool) -> Self {
        self.args.add_arg("in_stock", in_stock);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn product(mut self, product: String) -> Self {
        self.args.add_arg("product", product);
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }
}

impl<'a> ApiCall<ApiList<Sku>> for ListSkusCall<'a> {
    fn call(self) -> Result<ApiList<Sku>> {
        self.client.get("/skus", &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteSkuCall<'a> {
    client: &'a StripeClient,
    sku_id: String,
}

impl<'a> DeleteSkuCall<'a> {
    pub fn new(client: &'a StripeClient, sku_id: String) -> DeleteSkuCall<'a> {
        DeleteSkuCall {
            client: client,
            sku_id: sku_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteSkuCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/skus/{}", self.sku_id))
    }
}

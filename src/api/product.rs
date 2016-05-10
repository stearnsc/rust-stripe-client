use call_args::CallArgs;
use model::{ApiList, Delete, Dimensions, Product};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateProductRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateProductRequest<'a> {
    pub fn new(client: &'a StripeClient, name: String) -> CreateProductRequest<'a> {
        CreateProductRequest {
            client: client,
            args: CallArgs(vec![("name".to_string(), name)])
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

    pub fn attributes(mut self, attributes: Vec<String>) -> Self {
        self.args.add_list("attributes", attributes);
        self
    }

    pub fn caption(mut self, caption: String) -> Self {
        self.args.add_arg("caption", caption);
        self
    }

    pub fn deactivate_on(mut self, deactivate_on: Vec<String>) -> Self {
        self.args.add_list("deactivate_on", deactivate_on);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.args.add_list("images", images);
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

    pub fn shippable(mut self, shippable: bool) -> Self {
        self.args.add_arg("shippable", shippable);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.args.add_arg("url", url);
        self
    }
}

impl<'a> ApiCall<Product> for CreateProductRequest<'a> {
    fn call(self) -> Result<Product> {
        self.client.post("/products", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveProductRequest<'a> {
    client: &'a StripeClient,
    product_id: String,
    args: CallArgs
}

impl<'a> RetrieveProductRequest<'a> {
    pub fn new(client: &'a StripeClient, product_id: String) -> RetrieveProductRequest<'a> {
        RetrieveProductRequest {
            client: client,
            product_id: product_id,
            args: CallArgs::new()
        }
    }
}

impl<'a> ApiCall<Product> for RetrieveProductRequest<'a> {
    fn call(self) -> Result<Product> {
        self.client.get(format!("/products/{}", self.product_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateProductRequest<'a> {
    client: &'a StripeClient,
    product_id: String,
    args: CallArgs
}

impl<'a> UpdateProductRequest<'a> {
    pub fn new(client: &'a StripeClient, product_id: String) -> UpdateProductRequest<'a> {
        UpdateProductRequest {
            client: client,
            product_id: product_id,
            args: CallArgs::new()
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.args.add_arg("active", active);
        self
    }

    pub fn caption(mut self, caption: String) -> Self {
        self.args.add_arg("caption", caption);
        self
    }

    pub fn deactivate_on(mut self, deactivate_on: Vec<String>) -> Self {
        self.args.add_list("deactivate_on", deactivate_on);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.args.add_list("images", images);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.args.add_arg("name", name);
        self
    }

    pub fn package_dimensions(mut self, package_dimensions: Dimensions) -> Self {
        self.args.add_object("package_dimensions", package_dimensions);
        self
    }

    pub fn shippable(mut self, shippable: bool) -> Self {
        self.args.add_arg("shippable", shippable);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.args.add_arg("url", url);
        self
    }
}

impl<'a> ApiCall<Product> for UpdateProductRequest<'a> {
    fn call(self) -> Result<Product> {
        self.client.post(format!("/products/{}", self.product_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListProductsRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListProductsRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListProductsRequest<'a> {
        ListProductsRequest {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.args.add_arg("active", active);
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

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn shippable(mut self, shippable: bool) -> Self {
        self.args.add_arg("shippable", shippable);
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.args.add_arg("url", url);
        self
    }
}

impl<'a> ApiCall<ApiList<Product>> for ListProductsRequest<'a> {
    fn call(self) -> Result<ApiList<Product>> {
        self.client.get("/products", &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteProductRequest<'a> {
    client: &'a StripeClient,
    product_id: String
}

impl<'a> DeleteProductRequest<'a> {
    pub fn new(client: &'a StripeClient, product_id: String) -> DeleteProductRequest<'a> {
        DeleteProductRequest {
            client: client,
            product_id: product_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteProductRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/products/{}", self.product_id))
    }
}

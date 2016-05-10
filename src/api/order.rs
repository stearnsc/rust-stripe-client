use call_args::CallArgs;
use model::{ApiList, Currency, NewCard, Order, OrderItem, OrderStatus, Shipping};
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient, TimeConstraint};

#[derive(Debug)]
pub struct CreateOrderRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateOrderRequest<'a> {
    pub fn new(client: &'a StripeClient, currency: Currency) -> CreateOrderRequest<'a> {
        CreateOrderRequest {
            client: client,
            args: CallArgs(vec![("currency".to_string(), currency.to_string())])
        }
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.args.add_arg("email", email);
        self
    }

    pub fn items(mut self, items: Vec<OrderItem>) -> Self {
        self.args.add_object("items", items);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_object("shipping", shipping);
        self
    }
}

impl<'a> ApiCall<Order> for CreateOrderRequest<'a> {
    fn call(self) -> Result<Order> {
        self.client.post("/orders", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveOrderRequest<'a> {
    client: &'a StripeClient,
    order_id: String
}

impl<'a> RetrieveOrderRequest<'a> {
    pub fn new(client: &'a StripeClient, order_id: String) -> RetrieveOrderRequest<'a> {
        RetrieveOrderRequest {
            client: client,
            order_id: order_id
        }
    }
}

impl<'a> ApiCall<Order> for RetrieveOrderRequest<'a> {
    fn call(self) -> Result<Order> {
        self.client.get(format!("/orders/{}", self.order_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateOrderRequest<'a> {
    client: &'a StripeClient,
    order_id: String,
    args: CallArgs
}

impl<'a> UpdateOrderRequest<'a> {
    pub fn new(client: &'a StripeClient, order_id: String) -> UpdateOrderRequest<'a> {
        UpdateOrderRequest {
            client: client,
            order_id: order_id,
            args: CallArgs::new()
        }
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn selected_shipping_method(mut self, selected_shipping_method: String) -> Self {
        self.args.add_arg("selected_shipping_method", selected_shipping_method);
        self
    }

    pub fn status(mut self, status: OrderStatus) -> Self {
        self.args.add_arg("status", status);
        self
    }
}

impl<'a> ApiCall<Order> for UpdateOrderRequest<'a> {
    fn call(self) -> Result<Order> {
        self.client.post(format!("/orders/{}", self.order_id), &self.args)
    }
}

#[derive(Debug)]
pub struct PayOrderRequest<'a> {
    client: &'a StripeClient,
    order_id: String,
    args: CallArgs
}

impl<'a> PayOrderRequest<'a> {
    pub fn new(client: &'a StripeClient, order_id: String) -> PayOrderRequest<'a> {
        PayOrderRequest {
            client: client,
            order_id: order_id,
            args: CallArgs::new()
        }
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
        self
    }

    pub fn source_id(mut self, source_id: String) -> Self {
        self.args.add_arg("source", source_id);
        self
    }

    pub fn source(mut self, source: NewCard) -> Self {
        self.args.add_object("source", source);
        self
    }

    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.args.add_arg("application_fee", application_fee);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.args.add_arg("email", email);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<Order> for PayOrderRequest<'a> {
    fn call(self) -> Result<Order> {
        self.client.post(format!("/orders/{}/pay", self.order_id), &self.args)
    }
}

#[derive(Debug)]
pub struct ListOrdersRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListOrdersRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListOrdersRequest<'a> {
        ListOrdersRequest {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
        self
    }

    pub fn created_exact(mut self, created: i64) -> Self {
        self.args.add_arg("created", created);
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

    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.args.add_list("ids", ids);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.args.add_arg("limit", limit);
        self
    }

    pub fn starting_after(mut self, starting_after: String) -> Self {
        self.args.add_arg("starting_after", starting_after);
        self
    }

    pub fn status(mut self, status: OrderStatus) -> Self {
        self.args.add_arg("status", status);
        self
    }

    pub fn canceled(mut self, canceled: TimeConstraint) -> Self {
        self.args.add_object("status_transitions[canceled]", canceled);
        self
    }

    pub fn canceled_exact(mut self, canceled: i64) -> Self {
        self.args.add_arg("status_transitions[canceled]", canceled);
        self
    }

    pub fn fulfilled(mut self, fulfilled: TimeConstraint) -> Self {
        self.args.add_object("status_transitions[fulfilled]", fulfilled);
        self
    }

    pub fn fulfilled_exact(mut self, fulfilled: i64) -> Self {
        self.args.add_arg("status_transitions[fulfilled]", fulfilled);
        self
    }

    pub fn paid(mut self, paid: TimeConstraint) -> Self {
        self.args.add_object("status_transitions[paid]", paid);
        self
    }

    pub fn paid_exact(mut self, paid: i64) -> Self {
        self.args.add_arg("status_transitions[paid]", paid);
        self
    }

    pub fn returned(mut self, returned: TimeConstraint) -> Self {
        self.args.add_object("status_transitions[returned]", returned);
        self
    }

    pub fn returned_exact(mut self, returned: i64) -> Self {
        self.args.add_arg("status_transitions[returned]", returned);
        self
    }

    pub fn upstream_ids(mut self, upstream_ids: Vec<String>) -> Self {
        self.args.add_list("upstream_ids", upstream_ids);
        self
    }
}

impl<'a> ApiCall<ApiList<Order>> for ListOrdersRequest<'a> {
    fn call(self) -> Result<ApiList<Order>> {
        self.client.get("/orders", &self.args)
    }
}

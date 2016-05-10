use model::Delete;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct DeleteCustomerDiscountRequest<'a> {
    client: &'a StripeClient,
    customer_id: String
}

impl<'a> DeleteCustomerDiscountRequest<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> DeleteCustomerDiscountRequest<'a> {
        DeleteCustomerDiscountRequest {
            client: client,
            customer_id: customer_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteCustomerDiscountRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/customers/{}/discount", self.customer_id))
    }
}

#[derive(Debug)]
pub struct DeleteSubscriptionDiscountRequest<'a> {
    client: &'a StripeClient,
    customer_id: String,
    subscription_id: String
}

impl<'a> DeleteSubscriptionDiscountRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        subscription_id: String
    ) -> DeleteSubscriptionDiscountRequest<'a> {
        DeleteSubscriptionDiscountRequest {
            client: client,
            customer_id: customer_id,
            subscription_id: subscription_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteSubscriptionDiscountRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!(
            "/customers/{}/subscriptions/{}/discount",
            self.customer_id,
            self.subscription_id
        ))
    }
}

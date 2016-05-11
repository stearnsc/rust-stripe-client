use model::Delete;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct DeleteCustomerDiscountCall<'a> {
    client: &'a StripeClient,
    customer_id: String
}

impl<'a> DeleteCustomerDiscountCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> DeleteCustomerDiscountCall<'a> {
        DeleteCustomerDiscountCall {
            client: client,
            customer_id: customer_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteCustomerDiscountCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/customers/{}/discount", self.customer_id))
    }
}

#[derive(Debug)]
pub struct DeleteSubscriptionDiscountCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    subscription_id: String
}

impl<'a> DeleteSubscriptionDiscountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        subscription_id: String
    ) -> DeleteSubscriptionDiscountCall<'a> {
        DeleteSubscriptionDiscountCall {
            client: client,
            customer_id: customer_id,
            subscription_id: subscription_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteSubscriptionDiscountCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!(
            "/customers/{}/subscriptions/{}/discount",
            self.customer_id,
            self.subscription_id
        ))
    }
}

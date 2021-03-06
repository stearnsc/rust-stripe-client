use call_args::CallArgs;
use model::{ApiList, BitcoinReceiver, Currency};
use {Result, StripeClient};
use std::collections::BTreeMap;
use super::ApiCall;

#[derive(Debug)]
pub struct CreateBitcoinReceiverCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateBitcoinReceiverCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        amount: i64,
        currency: Currency,
        email: String
    ) -> CreateBitcoinReceiverCall<'a> {
        CreateBitcoinReceiverCall {
            client: client,
            args: CallArgs(vec![
                ("amount".to_string(), amount.to_string()),
                ("currency".to_string(), currency.to_string()),
                ("email".to_string(), email),
            ])
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn refund_mispayments(mut self, refund_mispayments: bool) -> Self {
        self.args.add_arg("refund_mispayments", refund_mispayments);
        self
    }
}

impl<'a> ApiCall<BitcoinReceiver> for CreateBitcoinReceiverCall<'a> {
    fn call(self) -> Result<BitcoinReceiver> {
        self.client.post("/bitcoin/receivers", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveBitcoinReceiverCall<'a> {
    client: &'a StripeClient,
    receiver_id: String
}

impl<'a> RetrieveBitcoinReceiverCall<'a> {
    pub fn new(client: &'a StripeClient, receiver_id: String) -> RetrieveBitcoinReceiverCall<'a> {
        RetrieveBitcoinReceiverCall {
            client: client,
            receiver_id: receiver_id
        }
    }
}

impl<'a> ApiCall<BitcoinReceiver> for RetrieveBitcoinReceiverCall<'a> {
    fn call(self) -> Result<BitcoinReceiver> {
        self.client.get(format!("/bitcoin/receivers/{}", self.receiver_id), &())
    }
}

#[derive(Debug)]
pub struct ListBitcoinReceiversCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListBitcoinReceiversCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListBitcoinReceiversCall<'a> {
        ListBitcoinReceiversCall {
            client: client,
            args: CallArgs(vec![
                ("include[]".to_string(), "total_count".to_string())
            ])
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

    pub fn filled(mut self, filled: bool) -> Self {
        self.args.add_arg("filled", filled);
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

    pub fn uncaptured_funds(mut self, uncaptured_funds: bool) -> Self {
        self.args.add_arg("uncaptured_funds", uncaptured_funds);
        self
    }
}

impl<'a> ApiCall<ApiList<BitcoinReceiver>> for ListBitcoinReceiversCall<'a> {
    fn call(self) -> Result<ApiList<BitcoinReceiver>> {
        self.client.get("/bitcoin/receivers", &self.args)
    }
}

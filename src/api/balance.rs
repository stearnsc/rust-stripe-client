use call_args::CallArgs;
use model::{ApiList, Balance, BalanceTransaction, Currency, TransactionType};
use {Result, StripeClient};
use super::ApiCall;
use time_constraint::TimeConstraint;

#[derive(Debug)]
pub struct RetrieveBalanceRequest<'a> {
    client: &'a StripeClient,
}

impl<'a> RetrieveBalanceRequest<'a> {
    pub fn new(client: &'a StripeClient) -> RetrieveBalanceRequest<'a> {
        RetrieveBalanceRequest {
            client: client
        }
    }
}

impl<'a> ApiCall<Balance> for RetrieveBalanceRequest<'a> {
    fn call(self) -> Result<Balance> {
        self.client.get("/balance", &())
    }
}

#[derive(Debug)]
pub struct RetrieveBalanceTransactionRequest<'a> {
    client: &'a StripeClient,
    balance_transaction_id: String
}

impl<'a> RetrieveBalanceTransactionRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        balance_transaction_id: &str
    ) -> RetrieveBalanceTransactionRequest<'a> {
        RetrieveBalanceTransactionRequest {
            client: client,
            balance_transaction_id: String::from(balance_transaction_id)
        }
    }
}

impl<'a> ApiCall<BalanceTransaction> for RetrieveBalanceTransactionRequest<'a> {
    fn call(self) -> Result<BalanceTransaction> {
        self.client.get(&format!("/balance/history/{}", self.balance_transaction_id), &())
    }
}

#[derive(Debug)]
pub struct ListBalanceHistoryRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListBalanceHistoryRequest<'a> {
    pub fn new(client: &'a StripeClient) -> Self {
        ListBalanceHistoryRequest {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn available_on(mut self, available_on: TimeConstraint) -> Self {
        self.args.add_named("available_on", available_on);
        self
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_named("created", created);
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.args.add_arg("currency", currency.to_string());
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

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.args.add_arg("source", source);
        self
    }

    pub fn transfer(mut self, transfer: String) -> Self {
        self.args.add_arg("transfer", transfer);
        self
    }

    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.args.add_arg("transaction_type", transaction_type);
        self
    }
}

impl<'a> ApiCall<ApiList<BalanceTransaction>> for ListBalanceHistoryRequest<'a> {
    fn call(self) -> Result<ApiList<BalanceTransaction>> {
        self.client.get("/balance/history", &self.args)
    }
}

use call_args::CallArgs;
use model::{ApiList, Balance, BalanceTransaction, Currency, TransactionType};
use {Result, StripeClient};
use super::ApiCall;
use time_constraint::TimeConstraint;

#[derive(Debug)]
pub struct RetrieveBalanceCall<'a> {
    client: &'a StripeClient,
}

impl<'a> RetrieveBalanceCall<'a> {
    pub fn new(client: &'a StripeClient) -> RetrieveBalanceCall<'a> {
        RetrieveBalanceCall {
            client: client
        }
    }
}

impl<'a> ApiCall<Balance> for RetrieveBalanceCall<'a> {
    fn call(self) -> Result<Balance> {
        self.client.get("/balance", &())
    }
}

#[derive(Debug)]
pub struct RetrieveBalanceTransactionCall<'a> {
    client: &'a StripeClient,
    balance_transaction_id: String
}

impl<'a> RetrieveBalanceTransactionCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        balance_transaction_id: String
    ) -> RetrieveBalanceTransactionCall<'a> {
        RetrieveBalanceTransactionCall {
            client: client,
            balance_transaction_id: balance_transaction_id
        }
    }
}

impl<'a> ApiCall<BalanceTransaction> for RetrieveBalanceTransactionCall<'a> {
    fn call(self) -> Result<BalanceTransaction> {
        self.client.get(&format!("/balance/history/{}", self.balance_transaction_id), &())
    }
}

#[derive(Debug)]
pub struct ListBalanceHistoryCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> ListBalanceHistoryCall<'a> {
    pub fn new(client: &'a StripeClient) -> Self {
        ListBalanceHistoryCall {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn available_on(mut self, available_on: TimeConstraint) -> Self {
        self.args.add_object("available_on", available_on);
        self
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
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

impl<'a> ApiCall<ApiList<BalanceTransaction>> for ListBalanceHistoryCall<'a> {
    fn call(self) -> Result<ApiList<BalanceTransaction>> {
        self.client.get("/balance/history", &self.args)
    }
}

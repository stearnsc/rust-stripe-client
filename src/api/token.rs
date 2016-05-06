use call_args::CallArgs;
use model::{ApiList, NewBankAccount, NewCard, Token};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateCardTokenRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateCardTokenRequest<'a> {
    pub fn new(client: &'a StripeClient) -> CreateCardTokenRequest<'a> {
        CreateCardTokenRequest {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn card(mut self, card: NewCard) -> Self {
        self.args.add_named("card", card);
        self
    }

    pub fn customer(mut self, customer: String, card_id: String) -> Self {
        self.args.add_arg("customer", customer);
        self.args.add_arg("card", card_id);
        self
    }
}

impl<'a> ApiCall<Token> for CreateCardTokenRequest<'a> {
    fn call(self) -> Result<Token> {
        self.client.post("/tokens", &self.args)
    }
}

#[derive(Debug)]
pub struct CreateBankAccountTokenRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateBankAccountTokenRequest<'a> {
    pub fn new(client: &'a StripeClient) -> CreateBankAccountTokenRequest<'a> {
        CreateBankAccountTokenRequest {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn bank_account(mut self, bank_account: NewBankAccount) -> Self {
        self.args.add_named("bank_account", bank_account);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
        self
    }
}

impl<'a> ApiCall<Token> for CreateBankAccountTokenRequest<'a> {
    fn call(self) -> Result<Token> {
        self.client.post("/tokens", &args)
    }
}

#[derive(Debug)]
pub struct CreatePiiTokenRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreatePiiTokenRequest<'a> {
    pub fn new(client: &'a StripeClient, personal_id_number: String) -> CreatePiiTokenRequest<'a> {
        CreatePiiTokenRequest {
            client: client,
            args: CallArgs(vec![("pii[personal_id_number]".to_string(), personal_id_number)]),
        }
    }
}

impl<'a> ApiCall<Token> for CreatePiiTokenRequest<'a> {
    fn call(self) -> Result<Token> {
        self.client.post("/tokens", &args)
    }
}

#[derive(Debug)]
pub struct RetrieveTokenRequest<'a> {
    client: &'a StripeClient,
    token_id: String
}

impl<'a> RetrieveTokenRequest<'a> {
    pub fn new(client: &'a StripeClient, token_id: String) -> RetrieveTokenRequest<'a> {
        RetrieveTokenRequest {
            client: client,
            token_id: token_id
        }
    }
}

impl<'a> ApiCall<OBJECT> for RetrieveTokenRequest<'a> {
    fn call(self) -> Result<OBJECT> {
        self.client.get(format!("/tokens/{}", self.token_id), &())
    }
}

use call_args::CallArgs;
use model::{NewBankAccount, NewCard, Token};
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateCardTokenCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateCardTokenCall<'a> {
    pub fn new(client: &'a StripeClient) -> CreateCardTokenCall<'a> {
        CreateCardTokenCall {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn card(mut self, card: NewCard) -> Self {
        self.args.add_object("card", card);
        self
    }

    pub fn customer(mut self, customer: String, card_id: String) -> Self {
        self.args.add_arg("customer", customer);
        self.args.add_arg("card", card_id);
        self
    }
}

impl<'a> ApiCall<Token> for CreateCardTokenCall<'a> {
    fn call(self) -> Result<Token> {
        self.client.post("/tokens", &self.args)
    }
}

#[derive(Debug)]
pub struct CreateBankAccountTokenCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateBankAccountTokenCall<'a> {
    pub fn new(client: &'a StripeClient) -> CreateBankAccountTokenCall<'a> {
        CreateBankAccountTokenCall {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn bank_account(mut self, bank_account: NewBankAccount) -> Self {
        self.args.add_object("bank_account", bank_account);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.args.add_arg("customer", customer);
        self
    }
}

impl<'a> ApiCall<Token> for CreateBankAccountTokenCall<'a> {
    fn call(self) -> Result<Token> {
        self.client.post("/tokens", &self.args)
    }
}

#[derive(Debug)]
pub struct CreatePiiTokenCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreatePiiTokenCall<'a> {
    pub fn new(client: &'a StripeClient, personal_id_number: String) -> CreatePiiTokenCall<'a> {
        CreatePiiTokenCall {
            client: client,
            args: CallArgs::from(("pii[personal_id_number]", personal_id_number)),
        }
    }
}

impl<'a> ApiCall<Token> for CreatePiiTokenCall<'a> {
    fn call(self) -> Result<Token> {
        self.client.post("/tokens", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveTokenCall<'a> {
    client: &'a StripeClient,
    token_id: String
}

impl<'a> RetrieveTokenCall<'a> {
    pub fn new(client: &'a StripeClient, token_id: String) -> RetrieveTokenCall<'a> {
        RetrieveTokenCall {
            client: client,
            token_id: token_id
        }
    }
}

impl<'a> ApiCall<Token> for RetrieveTokenCall<'a> {
    fn call(self) -> Result<Token> {
        self.client.get(format!("/tokens/{}", self.token_id), &())
    }
}

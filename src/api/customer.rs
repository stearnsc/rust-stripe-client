use call_args::CallArgs;
use model::{
    AccountHolderType, ApiList, BankAccount, Card, Customer, Delete, NewBankAccount, NewCard,
    Shipping
};
use std::collections::BTreeMap;
use super::ApiCall;
use time_constraint::TimeConstraint;
use {Result, StripeClient};

#[derive(Debug)]
pub struct CreateCustomerCall<'a> {
    client: &'a StripeClient,
    args: CallArgs,
}

impl<'a> CreateCustomerCall<'a> {
    pub fn new(client: &'a StripeClient) -> CreateCustomerCall<'a> {
        CreateCustomerCall {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn account_balance(mut self, account_balance: i64) -> Self {
        self.args.add_arg("account_balance", account_balance);
        self
    }

    pub fn business_vat_id(mut self, business_vat_id: String) -> Self {
        self.args.add_arg("business_vat_id", business_vat_id);
        self
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
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

    pub fn plan(mut self, plan: String) -> Self {
        self.args.add_arg("plan", plan);
        self
    }

    pub fn quantity(mut self, quantity: i64) -> Self {
        self.args.add_arg("quantity", quantity);
        self
    }

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_object("shipping", shipping);
        self
    }

    pub fn source_token(mut self, source: String) -> Self {
        self.args.add_arg("source", source);
        self
    }

    pub fn source_card(mut self, source: NewCard) -> Self {
        self.args.add_object("source", source);
        self
    }

    pub fn tax_percent(mut self, tax_percent: f64) -> Self {
        self.args.add_arg("tax_percent", format!("{:.*}", 2, tax_percent));
        self
    }

    pub fn trial_end(mut self, trial_end: i64) -> Self {
        self.args.add_arg("trial_end", trial_end);
        self
    }
}

impl<'a> ApiCall<Customer> for CreateCustomerCall<'a> {
    fn call(self) -> Result<Customer> {
        self.client.post("/customers", &self.args)
    }
}

#[derive(Debug)]
pub struct RetrieveCustomerCall<'a> {
    client: &'a StripeClient,
    customer_id: String
}

impl<'a> RetrieveCustomerCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> RetrieveCustomerCall<'a> {
        RetrieveCustomerCall {
            client: client,
            customer_id: customer_id
        }
    }
}

impl<'a> ApiCall<Customer> for RetrieveCustomerCall<'a> {
    fn call(self) -> Result<Customer> {
        self.client.get(format!("/customers/{}", self.customer_id), &())
    }
}

#[derive(Debug)]
pub struct UpdateCustomerCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> UpdateCustomerCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> UpdateCustomerCall<'a> {
        UpdateCustomerCall {
            client: client,
            customer_id: customer_id,
            args: CallArgs::new()
        }
    }

    pub fn account_balance(mut self, account_balance: String) -> Self {
        self.args.add_arg("account_balance", account_balance);
        self
    }

    pub fn business_vat_id(mut self, business_vat_id: String) -> Self {
        self.args.add_arg("business_vat_id", business_vat_id);
        self
    }

    pub fn coupon(mut self, coupon: String) -> Self {
        self.args.add_arg("coupon", coupon);
        self
    }

    pub fn default_source(mut self, default_source: String) -> Self {
        self.args.add_arg("default_source", default_source);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.args.add_arg("description", description);
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

    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.args.add_object("shipping", shipping);
        self
    }

    pub fn source_token(mut self, source_token: String) -> Self {
        self.args.add_arg("source", source_token);
        self
    }

    pub fn source_card(mut self, source_card: NewCard) -> Self {
        self.args.add_object("source", source_card);
        self
    }
}

impl<'a> ApiCall<Customer> for UpdateCustomerCall<'a> {
    fn call(self) -> Result<Customer> {
        self.client.post(format!("/customers/{}", self.customer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteCustomerCall<'a> {
    client: &'a StripeClient,
    customer_id: String
}

impl<'a> DeleteCustomerCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> DeleteCustomerCall<'a> {
        DeleteCustomerCall {
            client: client,
            customer_id: customer_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteCustomerCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/customers/{}", self.customer_id))
    }
}

#[derive(Debug)]
pub struct ListCustomersCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListCustomersCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListCustomersCall<'a> {
        ListCustomersCall {
            client: client,
            args: CallArgs(vec![("include[]".to_string(), "total_count".to_string())])
        }
    }

    pub fn created(mut self, created: TimeConstraint) -> Self {
        self.args.add_object("created", created);
        self
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
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
}

impl<'a> ApiCall<ApiList<Customer>> for ListCustomersCall<'a> {
    fn call(self) -> Result<ApiList<Customer>> {
        self.client.get("/customers", &self.args)
    }
}

#[derive(Debug)]
pub struct CustomerCreateBankAccountCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> CustomerCreateBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String
    ) -> CustomerCreateBankAccountCall<'a> {
        CustomerCreateBankAccountCall {
            client: client,
            customer_id: customer_id,
            args: CallArgs::new()
        }
    }

    pub fn bank_account_token(mut self, bank_account_token: String) -> Self {
        self.args.add_arg("source", bank_account_token);
        self
    }

    pub fn bank_account(mut self, bank_account: NewBankAccount) -> Self {
        self.args.add_object("source", bank_account);
        self
    }

    pub fn default_for_currency(mut self, default_for_currency: bool) -> Self {
        self.args.add_arg("default_for_currency", default_for_currency);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<BankAccount> for CustomerCreateBankAccountCall<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.post(format!("/customers/{}/sources", self.customer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct CustomerRetrieveBankAccountCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    bank_account_id: String
}

impl<'a> CustomerRetrieveBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        bank_account_id: String
    ) -> CustomerRetrieveBankAccountCall<'a> {
        CustomerRetrieveBankAccountCall {
            client: client,
            customer_id: customer_id,
            bank_account_id: bank_account_id
        }
    }
}

impl<'a> ApiCall<BankAccount> for CustomerRetrieveBankAccountCall<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.get(
            format!("/customers/{}/sources/{}", self.customer_id, self.bank_account_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct CustomerUpdateBankAccountCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    bank_account_id: String,
    args: CallArgs
}

impl<'a> CustomerUpdateBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        bank_account_id: String
    ) -> CustomerUpdateBankAccountCall<'a> {
        CustomerUpdateBankAccountCall {
            client: client,
            customer_id: customer_id,
            bank_account_id: bank_account_id,
            args: CallArgs::new()
        }
    }

    pub fn account_holder_name(mut self, account_holder_name: String) -> Self {
        self.args.add_arg("account_holder_name", account_holder_name);
        self
    }

    pub fn account_holder_type(mut self, account_holder_type: AccountHolderType) -> Self {
        self.args.add_arg("account_holder_type", account_holder_type);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }
}

impl<'a> ApiCall<BankAccount> for CustomerUpdateBankAccountCall<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.post(
            format!("/customers/{}/sources/{}", self.customer_id, self.bank_account_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct CustomerDeleteBankAccountCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    bank_account_id: String
}

impl<'a> CustomerDeleteBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        bank_account_id: String
    ) -> CustomerDeleteBankAccountCall<'a> {
        CustomerDeleteBankAccountCall {
            client: client,
            customer_id: customer_id,
            bank_account_id: bank_account_id
        }
    }
}

impl<'a> ApiCall<Delete> for CustomerDeleteBankAccountCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!(
            "/customers/{}/sources/{}",
            self.customer_id,
            self.bank_account_id
        ))
    }
}

#[derive(Debug)]
pub struct CustomerListBankAccountsCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> CustomerListBankAccountsCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> CustomerListBankAccountsCall<'a> {
        CustomerListBankAccountsCall {
            client: client,
            customer_id: customer_id,
            args: CallArgs(vec![
                ("include[]".to_string(), "total_count".to_string()),
                ("object".to_string(), "bank_account".to_string())
            ])
        }
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
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
}

impl<'a> ApiCall<ApiList<BankAccount>> for CustomerListBankAccountsCall<'a> {
    fn call(self) -> Result<ApiList<BankAccount>> {
        self.client.get(format!("/customers/{}/sources", self.customer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct CustomerCreateCardCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> CustomerCreateCardCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> CustomerCreateCardCall<'a> {
        CustomerCreateCardCall {
            client: client,
            customer_id: customer_id,
            args: CallArgs::new()
        }
    }

    pub fn card_token(mut self, card_token: String) -> Self {
        self.args.add_arg("source", card_token);
        self
    }

    pub fn card(mut self, card: NewCard) -> Self {
        self.args.add_object("source", card);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn default_for_currency(mut self, default_for_currency: bool) -> Self {
        self.args.add_arg("default_for_currency", default_for_currency);
        self
    }
}

impl<'a> ApiCall<Card> for CustomerCreateCardCall<'a> {
    fn call(self) -> Result<Card> {
        self.client.post(format!("/customers/{}/sources", self.customer_id), &self.args)
    }
}

#[derive(Debug)]
pub struct CustomerRetrieveCardCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    card_id: String
}

impl<'a> CustomerRetrieveCardCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        card_id: String
    ) -> CustomerRetrieveCardCall<'a> {
        CustomerRetrieveCardCall {
            client: client,
            customer_id: customer_id,
            card_id: card_id
        }
    }
}

impl<'a> ApiCall<Card> for CustomerRetrieveCardCall<'a> {
    fn call(self) -> Result<Card> {
        self.client.get(format!("/customers/{}/sources/{}", self.customer_id, self.card_id), &())
    }
}

#[derive(Debug)]
pub struct CustomerUpdateCardCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    card_id: String,
    args: CallArgs
}

impl<'a> CustomerUpdateCardCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        card_id: String
    ) -> CustomerUpdateCardCall<'a> {
        CustomerUpdateCardCall {
            client: client,
            customer_id: customer_id,
            card_id: card_id,
            args: CallArgs::new()
        }
    }

    pub fn address_city(mut self, address_city: String) -> Self {
        self.args.add_arg("address_city", address_city);
        self
    }

    pub fn address_country(mut self, address_country: String) -> Self {
        self.args.add_arg("address_country", address_country);
        self
    }

    pub fn address_line1(mut self, address_line1: String) -> Self {
        self.args.add_arg("address_line1", address_line1);
        self
    }

    pub fn address_line2(mut self, address_line2: String) -> Self {
        self.args.add_arg("address_line2", address_line2);
        self
    }

    pub fn address_state(mut self, address_state: String) -> Self {
        self.args.add_arg("address_state", address_state);
        self
    }

    pub fn address_zip(mut self, address_zip: String) -> Self {
        self.args.add_arg("address_zip", address_zip);
        self
    }

    pub fn default_for_currency(mut self, default_for_currency: bool) -> Self {
        self.args.add_arg("default_for_currency", default_for_currency);
        self
    }

    pub fn exp_month(mut self, exp_month: String) -> Self {
        self.args.add_arg("exp_month", exp_month);
        self
    }

    pub fn exp_year(mut self, exp_year: String) -> Self {
        self.args.add_arg("exp_year", exp_year);
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
}

impl<'a> ApiCall<Card> for CustomerUpdateCardCall<'a> {
    fn call(self) -> Result<Card> {
        self.client.post(
            format!("/customers/{}/sources/{}", self.customer_id, self.card_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct CustomerDeleteCardCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    card_id: String
}

impl<'a> CustomerDeleteCardCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        customer_id: String,
        card_id: String
    ) -> CustomerDeleteCardCall<'a> {
        CustomerDeleteCardCall {
            client: client,
            customer_id: customer_id,
            card_id: card_id
        }
    }
}

impl<'a> ApiCall<Delete> for CustomerDeleteCardCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/customers/{}/sources/{}", self.customer_id, self.card_id))
    }
}

#[derive(Debug)]
pub struct CustomerListCardsCall<'a> {
    client: &'a StripeClient,
    customer_id: String,
    args: CallArgs
}

impl<'a> CustomerListCardsCall<'a> {
    pub fn new(client: &'a StripeClient, customer_id: String) -> CustomerListCardsCall<'a> {
        CustomerListCardsCall {
            client: client,
            customer_id: customer_id,
            args: CallArgs(vec![
                ("include[]".to_string(), "total_count".to_string()),
                ("object".to_string(), "card".to_string())
            ])
        }
    }

    pub fn ending_before(mut self, ending_before: String) -> Self {
        self.args.add_arg("ending_before", ending_before);
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
}

impl<'a> ApiCall<ApiList<Card>> for CustomerListCardsCall<'a> {
    fn call(self) -> Result<ApiList<Card>> {
        self.client.get(format!("/customers/{}/sources", self.customer_id), &self.args)
    }
}

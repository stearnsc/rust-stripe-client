use call_args::CallArgs;
use model::{
    ApiList, Account, AccountRejectReason, BankAccount, Card, Currency, Delete, NewLegalEntity,
    NewBankAccount, NewCard, TosAcceptance, TransferSchedule
 };
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct RetrieveAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: Option<String>
}

impl<'a> RetrieveAccountRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: Option<String>) -> RetrieveAccountRequest<'a> {
        RetrieveAccountRequest {
            client: client,
            account_id: account_id
        }
    }
}

impl<'a> ApiCall<Account> for RetrieveAccountRequest<'a> {
    fn call(self) -> Result<Account> {
        let endpoint = self.account_id
            .map(|id| format!("/accounts/{}", id))
            .unwrap_or("/accounts".to_string());
        self.client.get(endpoint, &())
    }
}

#[derive(Debug)]
pub struct CreateAccountRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateAccountRequest<'a> {
    pub fn new(client: &'a StripeClient) -> CreateAccountRequest<'a> {
        CreateAccountRequest {
            client: client,
            args: CallArgs::new()
        }
    }

    pub fn business_logo(mut self, business_logo: String) -> Self {
        self.args.add_arg("business_logo", business_logo);
        self
    }

    pub fn business_primary_color(mut self, business_primary_color: String) -> Self {
        self.args.add_arg("business_primary_color", business_primary_color);
        self
    }

    pub fn business_url(mut self, business_url: String) -> Self {
        self.args.add_arg("business_url", business_url);
        self
    }

    pub fn country(mut self, country: String) -> Self {
        self.args.add_arg("country", country);
        self
    }

    pub fn debit_negative_balances(mut self, debit_negative_balances: bool) -> Self {
        self.args.add_arg("debit_negative_balances", debit_negative_balances);
        self
    }

    pub fn decline_charge_on(mut self, avs_failure: bool, cvc_failure: bool) -> Self {
        self.args.add_object("decline_charge_on", (
            ("avs_failure", avs_failure.to_string()),
            ("cvc_failure", cvc_failure.to_string())
        ));
        self
    }

    pub fn default_currency(mut self, default_currency: Currency) -> Self {
        self.args.add_arg("default_currency", default_currency);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.args.add_arg("email", email);
        self
    }

    pub fn external_account_id(mut self, external_account_id: String) -> Self {
        self.args.add_arg("external_account", external_account_id);
        self
    }

    pub fn external_card(mut self, external_card: NewCard) -> Self {
        self.args.add_object("external_account", external_card);
        self
    }

    pub fn external_bank_account(mut self, external_bank_account: NewBankAccount) -> Self {
        self.args.add_object("external_account", external_bank_account);
        self
    }

    pub fn legal_entity(mut self, legal_entity: NewLegalEntity) -> Self {
        self.args.add_object("legal_entity", legal_entity);
        self
    }

    pub fn managed(mut self, managed: bool) -> Self {
        self.args.add_arg("managed", managed);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn product_description(mut self, product_description: String) -> Self {
        self.args.add_arg("product_description", product_description);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }

    pub fn support_email(mut self, support_email: String) -> Self {
        self.args.add_arg("support_email", support_email);
        self
    }

    pub fn support_phone(mut self, support_phone: String) -> Self {
        self.args.add_arg("support_phone", support_phone);
        self
    }

    pub fn support_url(mut self, support_url: String) -> Self {
        self.args.add_arg("support_url", support_url);
        self
    }

    pub fn tos_acceptance(mut self, tos_acceptance: TosAcceptance) -> Self {
        self.args.add_object("tos_acceptance", tos_acceptance);
        self
    }

    pub fn transfer_schedule(mut self, transfer_schedule: TransferSchedule) -> Self {
        self.args.add_object("transfer_schedule", transfer_schedule);
        self
    }
}

impl<'a> ApiCall<Account> for CreateAccountRequest<'a> {
    fn call(self) -> Result<Account> {
        self.client.post("/accounts", &self.args)
    }
}

#[derive(Debug)]
pub struct UpdateAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> UpdateAccountRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> UpdateAccountRequest<'a> {
        UpdateAccountRequest {
            client: client,
            account_id: account_id,
            args: CallArgs::new()
        }
    }

    pub fn business_logo(mut self, business_logo: String) -> Self {
        self.args.add_arg("business_logo", business_logo);
        self
    }

    pub fn business_primary_color(mut self, business_primary_color: String) -> Self {
        self.args.add_arg("business_primary_color", business_primary_color);
        self
    }

    pub fn business_url(mut self, business_url: String) -> Self {
        self.args.add_arg("business_url", business_url);
        self
    }

    pub fn debit_negative_balances(mut self, debit_negative_balances: bool) -> Self {
        self.args.add_arg("debit_negative_balances", debit_negative_balances);
        self
    }

    pub fn decline_charge_on(mut self, avs_failure: bool, cvc_failure: bool) -> Self {
        self.args.add_object("decline_charge_on", (
            ("avs_failure", avs_failure.to_string()),
            ("cvc_failure", cvc_failure.to_string())
        ));
        self
    }

    pub fn default_currency(mut self, default_currency: Currency) -> Self {
        self.args.add_arg("default_currency", default_currency);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.args.add_arg("email", email);
        self
    }

    pub fn external_account_id(mut self, external_account_id: String) -> Self {
        self.args.add_arg("external_account", external_account_id);
        self
    }

    pub fn external_card(mut self, external_card: NewCard) -> Self {
        self.args.add_object("external_account", external_card);
        self
    }

    pub fn external_bank_account(mut self, external_bank_account: NewBankAccount) -> Self {
        self.args.add_object("external_account", external_bank_account);
        self
    }

    pub fn legal_entity(mut self, legal_entity: NewLegalEntity) -> Self {
        self.args.add_object("legal_entity", legal_entity);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.args.add_object("metadata", metadata);
        self
    }

    pub fn product_description(mut self, product_description: String) -> Self {
        self.args.add_arg("product_description", product_description);
        self
    }

    pub fn statement_descriptor(mut self, statement_descriptor: String) -> Self {
        self.args.add_arg("statement_descriptor", statement_descriptor);
        self
    }

    pub fn support_email(mut self, support_email: String) -> Self {
        self.args.add_arg("support_email", support_email);
        self
    }

    pub fn support_phone(mut self, support_phone: String) -> Self {
        self.args.add_arg("support_phone", support_phone);
        self
    }

    pub fn support_url(mut self, support_url: String) -> Self {
        self.args.add_arg("support_url", support_url);
        self
    }

    pub fn tos_acceptance(mut self, tos_acceptance: TosAcceptance) -> Self {
        self.args.add_object("tos_acceptance", tos_acceptance);
        self
    }

    pub fn transfer_schedule(mut self, transfer_schedule: TransferSchedule) -> Self {
        self.args.add_object("transfer_schedule", transfer_schedule);
        self
    }
}

impl<'a> ApiCall<Account> for UpdateAccountRequest<'a> {
    fn call(self) -> Result<Account> {
        self.client.post(format!("/accounts/{}", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String
}

impl<'a> DeleteAccountRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> DeleteAccountRequest<'a> {
        DeleteAccountRequest {
            client: client,
            account_id: account_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteAccountRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/accounts/{}", self.account_id))
    }
}

#[derive(Debug)]
pub struct RejectAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    reason: AccountRejectReason
}

impl<'a> RejectAccountRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        reason: AccountRejectReason
    ) -> RejectAccountRequest<'a> {
        RejectAccountRequest {
            client: client,
            account_id: account_id,
            reason: reason
        }
    }
}

impl<'a> ApiCall<Account> for RejectAccountRequest<'a> {
    fn call(self) -> Result<Account> {
        self.client.post(
            format!("/accounts/{}/reject", self.account_id),
            &("reason", self.reason.to_string())
        )
    }
}

#[derive(Debug)]
pub struct ListAccountsRequest<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListAccountsRequest<'a> {
    pub fn new(client: &'a StripeClient) -> ListAccountsRequest<'a> {
        ListAccountsRequest {
            client: client,
            args: CallArgs::new()
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

impl<'a> ApiCall<ApiList<Account>> for ListAccountsRequest<'a> {
    fn call(self) -> Result<ApiList<Account>> {
        self.client.get("/accounts", &self.args)
    }
}

#[derive(Debug)]
pub struct AccountCreateBankAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountCreateBankAccountRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountCreateBankAccountRequest<'a> {
        AccountCreateBankAccountRequest {
            client: client,
            account_id: account_id,
            args: CallArgs::new()
        }
    }

    pub fn bank_account_token(mut self, bank_account_token: String) -> Self {
        self.args.add_arg("external_account", bank_account_token);
        self
    }

    pub fn bank_account(mut self, bank_account: NewBankAccount) -> Self {
        self.args.add_object("external_account", bank_account);
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

impl<'a> ApiCall<BankAccount> for AccountCreateBankAccountRequest<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.post(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct AccountRetrieveBankAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    external_account_id: String
}

impl<'a> AccountRetrieveBankAccountRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        external_account_id: String
    ) -> AccountRetrieveBankAccountRequest<'a> {
        AccountRetrieveBankAccountRequest {
            client: client,
            account_id: account_id,
            external_account_id: external_account_id
        }
    }
}

impl<'a> ApiCall<BankAccount> for AccountRetrieveBankAccountRequest<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.get(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.external_account_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct AccountUpdateBankAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    external_account_id: String,
    args: CallArgs
}

impl<'a> AccountUpdateBankAccountRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        external_account_id: String
    ) -> AccountUpdateBankAccountRequest<'a> {
        AccountUpdateBankAccountRequest {
            client: client,
            account_id: account_id,
            external_account_id: external_account_id,
            args: CallArgs::new()
        }
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

impl<'a> ApiCall<BankAccount> for AccountUpdateBankAccountRequest<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.post(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.external_account_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct AccountDeleteBankAccountRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    external_account_id: String
}

impl<'a> AccountDeleteBankAccountRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        external_account_id: String
    ) -> AccountDeleteBankAccountRequest<'a> {
        AccountDeleteBankAccountRequest {
            client: client,
            account_id: account_id,
            external_account_id: external_account_id
        }
    }
}

impl<'a> ApiCall<Delete> for AccountDeleteBankAccountRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(
            format!("/accounts/{}/external_accounts/{}",
            self.account_id,
            self.external_account_id
        ))
    }
}

#[derive(Debug)]
pub struct AccountListBankAccountsRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountListBankAccountsRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountListBankAccountsRequest<'a> {
        AccountListBankAccountsRequest {
            client: client,
            account_id: account_id,
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

impl<'a> ApiCall<ApiList<BankAccount>> for AccountListBankAccountsRequest<'a> {
    fn call(self) -> Result<ApiList<BankAccount>> {
        self.client.get(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct AccountCreateCardRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountCreateCardRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountCreateCardRequest<'a> {
        AccountCreateCardRequest {
            client: client,
            account_id: account_id,
            args: CallArgs::new()
        }
    }

    pub fn card_token(mut self, card_token: String) -> Self {
        self.args.add_arg("external_account", card_token);
        self
    }

    pub fn card(mut self, card: NewCard) -> Self {
        self.args.add_object("external_account", card);
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

impl<'a> ApiCall<Card> for AccountCreateCardRequest<'a> {
    fn call(self) -> Result<Card> {
        self.client.post(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct AccountRetrieveCardRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    card_id: String
}

impl<'a> AccountRetrieveCardRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        card_id: String
    ) -> AccountRetrieveCardRequest<'a> {
        AccountRetrieveCardRequest {
            client: client,
            account_id: account_id,
            card_id: card_id
        }
    }
}

impl<'a> ApiCall<Card> for AccountRetrieveCardRequest<'a> {
    fn call(self) -> Result<Card> {
        self.client.get(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.card_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct AccountUpdateCardRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    card_id: String,
    args: CallArgs
}

impl<'a> AccountUpdateCardRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        card_id: String
    ) -> AccountUpdateCardRequest<'a> {
        AccountUpdateCardRequest {
            client: client,
            account_id: account_id,
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

impl<'a> ApiCall<Card> for AccountUpdateCardRequest<'a> {
    fn call(self) -> Result<Card> {
        self.client.post(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.card_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct AccountDeleteCardRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    card_id: String
}

impl<'a> AccountDeleteCardRequest<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        card_id: String
    ) -> AccountDeleteCardRequest<'a> {
        AccountDeleteCardRequest {
            client: client,
            account_id: account_id,
            card_id: card_id
        }
    }
}

impl<'a> ApiCall<Delete> for AccountDeleteCardRequest<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!(
            "/accounts/{}/external_accounts/{}",
            self.account_id,
            self.card_id
        ))
    }
}

#[derive(Debug)]
pub struct AccountListCardsRequest<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountListCardsRequest<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountListCardsRequest<'a> {
        AccountListCardsRequest {
            client: client,
            account_id: account_id,
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

impl<'a> ApiCall<ApiList<Card>> for AccountListCardsRequest<'a> {
    fn call(self) -> Result<ApiList<Card>> {
        self.client.get(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

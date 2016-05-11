use call_args::CallArgs;
use model::{
    ApiList, Account, AccountRejectReason, BankAccount, Card, Currency, Delete, NewLegalEntity,
    NewBankAccount, NewCard, TosAcceptance, TransferSchedule
 };
use std::collections::BTreeMap;
use super::ApiCall;
use {Result, StripeClient};

#[derive(Debug)]
pub struct RetrieveAccountCall<'a> {
    client: &'a StripeClient,
    account_id: Option<String>
}

impl<'a> RetrieveAccountCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: Option<String>) -> RetrieveAccountCall<'a> {
        RetrieveAccountCall {
            client: client,
            account_id: account_id
        }
    }
}

impl<'a> ApiCall<Account> for RetrieveAccountCall<'a> {
    fn call(self) -> Result<Account> {
        let endpoint = self.account_id
            .map(|id| format!("/accounts/{}", id))
            .unwrap_or("/accounts".to_string());
        self.client.get(endpoint, &())
    }
}

#[derive(Debug)]
pub struct CreateAccountCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> CreateAccountCall<'a> {
    pub fn new(client: &'a StripeClient) -> CreateAccountCall<'a> {
        CreateAccountCall {
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

impl<'a> ApiCall<Account> for CreateAccountCall<'a> {
    fn call(self) -> Result<Account> {
        self.client.post("/accounts", &self.args)
    }
}

#[derive(Debug)]
pub struct UpdateAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> UpdateAccountCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> UpdateAccountCall<'a> {
        UpdateAccountCall {
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

impl<'a> ApiCall<Account> for UpdateAccountCall<'a> {
    fn call(self) -> Result<Account> {
        self.client.post(format!("/accounts/{}", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct DeleteAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String
}

impl<'a> DeleteAccountCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> DeleteAccountCall<'a> {
        DeleteAccountCall {
            client: client,
            account_id: account_id
        }
    }
}

impl<'a> ApiCall<Delete> for DeleteAccountCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!("/accounts/{}", self.account_id))
    }
}

#[derive(Debug)]
pub struct RejectAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    reason: AccountRejectReason
}

impl<'a> RejectAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        reason: AccountRejectReason
    ) -> RejectAccountCall<'a> {
        RejectAccountCall {
            client: client,
            account_id: account_id,
            reason: reason
        }
    }
}

impl<'a> ApiCall<Account> for RejectAccountCall<'a> {
    fn call(self) -> Result<Account> {
        self.client.post(
            format!("/accounts/{}/reject", self.account_id),
            &("reason", self.reason.to_string())
        )
    }
}

#[derive(Debug)]
pub struct ListAccountsCall<'a> {
    client: &'a StripeClient,
    args: CallArgs
}

impl<'a> ListAccountsCall<'a> {
    pub fn new(client: &'a StripeClient) -> ListAccountsCall<'a> {
        ListAccountsCall {
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

impl<'a> ApiCall<ApiList<Account>> for ListAccountsCall<'a> {
    fn call(self) -> Result<ApiList<Account>> {
        self.client.get("/accounts", &self.args)
    }
}

#[derive(Debug)]
pub struct AccountCreateBankAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountCreateBankAccountCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountCreateBankAccountCall<'a> {
        AccountCreateBankAccountCall {
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

impl<'a> ApiCall<BankAccount> for AccountCreateBankAccountCall<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.post(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct AccountRetrieveBankAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    external_account_id: String
}

impl<'a> AccountRetrieveBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        external_account_id: String
    ) -> AccountRetrieveBankAccountCall<'a> {
        AccountRetrieveBankAccountCall {
            client: client,
            account_id: account_id,
            external_account_id: external_account_id
        }
    }
}

impl<'a> ApiCall<BankAccount> for AccountRetrieveBankAccountCall<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.get(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.external_account_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct AccountUpdateBankAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    external_account_id: String,
    args: CallArgs
}

impl<'a> AccountUpdateBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        external_account_id: String
    ) -> AccountUpdateBankAccountCall<'a> {
        AccountUpdateBankAccountCall {
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

impl<'a> ApiCall<BankAccount> for AccountUpdateBankAccountCall<'a> {
    fn call(self) -> Result<BankAccount> {
        self.client.post(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.external_account_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct AccountDeleteBankAccountCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    external_account_id: String
}

impl<'a> AccountDeleteBankAccountCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        external_account_id: String
    ) -> AccountDeleteBankAccountCall<'a> {
        AccountDeleteBankAccountCall {
            client: client,
            account_id: account_id,
            external_account_id: external_account_id
        }
    }
}

impl<'a> ApiCall<Delete> for AccountDeleteBankAccountCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(
            format!("/accounts/{}/external_accounts/{}",
            self.account_id,
            self.external_account_id
        ))
    }
}

#[derive(Debug)]
pub struct AccountListBankAccountsCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountListBankAccountsCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountListBankAccountsCall<'a> {
        AccountListBankAccountsCall {
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

impl<'a> ApiCall<ApiList<BankAccount>> for AccountListBankAccountsCall<'a> {
    fn call(self) -> Result<ApiList<BankAccount>> {
        self.client.get(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct AccountCreateCardCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountCreateCardCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountCreateCardCall<'a> {
        AccountCreateCardCall {
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

impl<'a> ApiCall<Card> for AccountCreateCardCall<'a> {
    fn call(self) -> Result<Card> {
        self.client.post(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

#[derive(Debug)]
pub struct AccountRetrieveCardCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    card_id: String
}

impl<'a> AccountRetrieveCardCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        card_id: String
    ) -> AccountRetrieveCardCall<'a> {
        AccountRetrieveCardCall {
            client: client,
            account_id: account_id,
            card_id: card_id
        }
    }
}

impl<'a> ApiCall<Card> for AccountRetrieveCardCall<'a> {
    fn call(self) -> Result<Card> {
        self.client.get(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.card_id),
            &()
        )
    }
}

#[derive(Debug)]
pub struct AccountUpdateCardCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    card_id: String,
    args: CallArgs
}

impl<'a> AccountUpdateCardCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        card_id: String
    ) -> AccountUpdateCardCall<'a> {
        AccountUpdateCardCall {
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

impl<'a> ApiCall<Card> for AccountUpdateCardCall<'a> {
    fn call(self) -> Result<Card> {
        self.client.post(
            format!("/accounts/{}/external_accounts/{}", self.account_id, self.card_id),
            &self.args
        )
    }
}

#[derive(Debug)]
pub struct AccountDeleteCardCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    card_id: String
}

impl<'a> AccountDeleteCardCall<'a> {
    pub fn new(
        client: &'a StripeClient,
        account_id: String,
        card_id: String
    ) -> AccountDeleteCardCall<'a> {
        AccountDeleteCardCall {
            client: client,
            account_id: account_id,
            card_id: card_id
        }
    }
}

impl<'a> ApiCall<Delete> for AccountDeleteCardCall<'a> {
    fn call(self) -> Result<Delete> {
        self.client.delete(format!(
            "/accounts/{}/external_accounts/{}",
            self.account_id,
            self.card_id
        ))
    }
}

#[derive(Debug)]
pub struct AccountListCardsCall<'a> {
    client: &'a StripeClient,
    account_id: String,
    args: CallArgs
}

impl<'a> AccountListCardsCall<'a> {
    pub fn new(client: &'a StripeClient, account_id: String) -> AccountListCardsCall<'a> {
        AccountListCardsCall {
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

impl<'a> ApiCall<ApiList<Card>> for AccountListCardsCall<'a> {
    fn call(self) -> Result<ApiList<Card>> {
        self.client.get(format!("/accounts/{}/external_accounts", self.account_id), &self.args)
    }
}

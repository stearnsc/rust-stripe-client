#![feature(custom_derive, plugin)]
#![feature(question_mark)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate rand;
extern crate serde;
extern crate serde_json;

use hyper::client::response::Response;
use hyper::Client;
use hyper::header::{Authorization, Basic, Headers};
use hyper::status::StatusCode;
use serde::de::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::Read;

pub mod api;
pub mod either;
pub mod errors;
pub mod model;

mod call_args;
mod idempotency_header;
mod stripe_version_header;
mod url_encodable;
mod time_constraint;

pub use either::Either;
pub use time_constraint::TimeConstraint;

use api::*;
use errors::error::Error;
use errors::stripe_error;
use model::*;
use stripe_version_header::StripeVersion;
use url_encodable::UrlEncodable;

const BASE_URL: &'static str = "https://api.stripe.com/v1";
const API_VERSION: &'static str = "2016-03-07";

pub type Result<T> = std::result::Result<T, Error>;

// TODO add file upload stuff

#[derive(Debug)]
pub struct StripeClient {
    pub key: String,
    client: Client
}

impl StripeClient {

    pub fn new(key: &str) -> StripeClient {
        StripeClient {
            key: String::from(key),
            client: Client::new()
        }
    }

    /// https://stripe.com/docs/api#retrieve_balance
    pub fn retrieve_balance<'a>(&'a self) -> RetrieveBalanceCall<'a> {
        RetrieveBalanceCall::new(self)
    }

    /// https://stripe.com/docs/api#retrieve_balance_transaction
    pub fn retrieve_balance_transaction<'a>(
        &'a self,
        transaction_id: String
    ) -> RetrieveBalanceTransactionCall<'a> {
        RetrieveBalanceTransactionCall::new(self, transaction_id)
    }

    /// https://stripe.com/docs/api#balance_history
    pub fn list_balance_history<'a>(&'a self) -> ListBalanceHistoryCall<'a> {
        ListBalanceHistoryCall::new(self)
    }

    /// https://stripe.com/docs/api#create_charge
    pub fn create_charge<'a>(
        &'a self,
        amount: i64,
        currency: Currency,
    ) -> CreateChargeCall<'a> {
        CreateChargeCall::new(self, amount, currency)
    }

    /// https://stripe.com/docs/api#retrieve_charge
    pub fn retrieve_charge<'a>(&'a self, charge_id: String) -> RetrieveChargeCall<'a> {
        RetrieveChargeCall::new(self, charge_id)
    }

    /// https://stripe.com/docs/api#update_charge
    pub fn update_charge<'a>(&'a self, charge_id: String) -> UpdateChargeCall<'a> {
        UpdateChargeCall::new(self, charge_id)
    }

    /// https://stripe.com/docs/api#capture_charge
    pub fn capture_charge<'a>(&'a self, charge_id: String) -> CaptureChargeCall<'a> {
        CaptureChargeCall::new(self, charge_id)
    }

    /// https://stripe.com/docs/api#list_charges
    pub fn list_charges<'a>(&'a self) -> ListChargesCall<'a> {
        ListChargesCall::new(self)
    }

    /// https://stripe.com/docs/api#create_customer
    pub fn create_customer<'a>(&'a self) -> CreateCustomerCall<'a> {
        CreateCustomerCall::new(self)
    }

    /// https://stripe.com/docs/api#retrieve_customer
    pub fn retrieve_customer<'a>(&'a self, customer_id: String) -> RetrieveCustomerCall<'a> {
        RetrieveCustomerCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#update_customer
    pub fn update_customer<'a>(&'a self, customer_id: String) -> UpdateCustomerCall<'a> {
        UpdateCustomerCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#delete_customer
    pub fn delete_customer<'a>(&'a self, customer_id: String) -> DeleteCustomerCall<'a> {
        DeleteCustomerCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#list_customers
    pub fn list_customers<'a>(&'a self) -> ListCustomersCall<'a> {
        ListCustomersCall::new(self)
    }

    /// https://stripe.com/docs/api#retrieve_dispute
    pub fn retrieve_dispute<'a>(&'a self, dispute_id: String) -> RetrieveDisputeCall<'a> {
        RetrieveDisputeCall::new(self, dispute_id)
    }

    /// https://stripe.com/docs/api#update_dispute
    pub fn update_dispute<'a>(&'a self, dispute_id: String) -> UpdateDisputeCall<'a> {
        UpdateDisputeCall::new(self, dispute_id)
    }

    /// https://stripe.com/docs/api#close_dispute
    pub fn close_dispute<'a>(&'a self, dispute_id: String) -> CloseDisputeCall<'a> {
        CloseDisputeCall::new(self, dispute_id)
    }

    /// https://stripe.com/docs/api#list_disputes
    pub fn list_disputes<'a>(&'a self) -> ListDisputesCall<'a> {
        ListDisputesCall::new(self)
    }

    /// https://stripe.com/docs/api#retrieve_event
    pub fn retrieve_event<'a>(&'a self, event_id: String) -> RetrieveEventCall<'a> {
        RetrieveEventCall::new(self, event_id)
    }

    /// https://stripe.com/docs/api#list_events
    pub fn list_events<'a>(&'a self) -> ListEventCall<'a> {
        ListEventCall::new(self)
    }

    /// https://stripe.com/docs/api#create_refund
    pub fn create_refund<'a>(&'a self, charge_id: String) -> CreateRefundCall<'a> {
        CreateRefundCall::new(self, charge_id)
    }

    /// https://stripe.com/docs/api#retrieve_refund
    pub fn retrieve_refund<'a>(&'a self, refund_id: String) -> RetrieveRefundCall<'a> {
        RetrieveRefundCall::new(self, refund_id)
    }

    /// https://stripe.com/docs/api#update_refund
    pub fn update_refund<'a>(&'a self, refund_id: String) -> UpdateRefundCall<'a> {
        UpdateRefundCall::new(self, refund_id)
    }

    /// https://stripe.com/docs/api#list_refunds
    pub fn list_refunds<'a>(&'a self) -> ListRefundCall<'a> {
        ListRefundCall::new(self)
    }

    /// https://stripe.com/docs/api#create_card_token
    pub fn create_card_token<'a>(&'a self) -> CreateCardTokenCall<'a> {
        CreateCardTokenCall::new(self)
    }

    /// https://stripe.com/docs/api#create_bank_account_token
    pub fn create_bank_account_token<'a>(&'a self) -> CreateBankAccountTokenCall<'a> {
        CreateBankAccountTokenCall::new(self)
    }

    /// https://stripe.com/docs/api#create_pii_token
    pub fn create_pii_token<'a>(&'a self, personal_id_number: String) -> CreatePiiTokenCall<'a> {
        CreatePiiTokenCall::new(self, personal_id_number)
    }

    /// https://stripe.com/docs/api#retrieve_token
    pub fn retrieve_token<'a>(&'a self, token_id: String) -> RetrieveTokenCall<'a> {
        RetrieveTokenCall::new(self, token_id)
    }

    /// https://stripe.com/docs/api#create_transfer
    pub fn create_transfer<'a>(
        &'a self,
        amount: i64,
        currency: Currency,
        destination: String
    ) -> CreateTransferCall<'a> {
        CreateTransferCall::new(self, amount, currency, destination)
    }

    /// https://stripe.com/docs/api#retrieve_transfer
    pub fn retrieve_transfer<'a>(&'a self, transfer_id: String) -> RetrieveTransferCall<'a> {
        RetrieveTransferCall::new(self, transfer_id)
    }

    /// https://stripe.com/docs/api#update_transfer
    pub fn update_transfer<'a>(&'a self, transfer_id: String) -> UpdateTransferCall<'a> {
        UpdateTransferCall::new(self, transfer_id)
    }

    /// https://stripe.com/docs/api#list_transfers
    pub fn list_transfers<'a>(&'a self) -> ListTransfersCall<'a> {
        ListTransfersCall::new(self)
    }

    /// https://stripe.com/docs/api#create_transfer_reversal
    pub fn create_transfer_reversal<'a>(&'a self, transfer_id: String) -> CreateTransferReversalCall<'a> {
        CreateTransferReversalCall::new(self, transfer_id)
    }

    /// https://stripe.com/docs/api#retrieve_transfer_reversal
    pub fn retrieve_transfer_reversal<'a>(
        &'a self,
        transfer_id: String,
        reversal_id: String
    ) -> RetrieveTransferReversalCall<'a> {
        RetrieveTransferReversalCall::new(self, transfer_id, reversal_id)
    }

    /// https://stripe.com/docs/api#update_transfer_reversal
    pub fn update_transfer_reversal<'a>(
        &'a self,
        transfer_id: String,
        reversal_id: String
    ) -> UpdateTransferReversalCall<'a> {
        UpdateTransferReversalCall::new(self, transfer_id, reversal_id)
    }

    /// https://stripe.com/docs/api#list_transfer_reversals
    pub fn list_transfer_reversals<'a>(
        &'a self,
        transfer_id: String
    ) -> ListTransferReversalsCall<'a> {
        ListTransferReversalsCall::new(self, transfer_id)
    }

    /// https://stripe.com/docs/api#retrieve_account
    /// Fetch account associated with self.key
    pub fn retrieve_current_account<'a>(&'a self) -> RetrieveAccountCall<'a> {
        RetrieveAccountCall::new(self, None)
    }

    /// https://stripe.com/docs/api#retrieve_account
    pub fn retrieve_account<'a>(&'a self, account_id: String) -> RetrieveAccountCall<'a> {
        RetrieveAccountCall::new(self, Some(account_id))
    }

    /// https://stripe.com/docs/api#create_account
    pub fn create_account<'a>(&'a self) -> CreateAccountCall<'a> {
        CreateAccountCall::new(self)
    }

    /// https://stripe.com/docs/api#update_account
    pub fn update_account<'a>(&'a self, account_id: String) -> UpdateAccountCall<'a> {
        UpdateAccountCall::new(self, account_id)
    }

    /// https://stripe.com/docs/api#delete_account
    pub fn delete_account<'a>(&'a self, account_id: String) -> DeleteAccountCall<'a> {
        DeleteAccountCall::new(self, account_id)
    }

    /// https://stripe.com/docs/api#reject_account
    pub fn reject_account<'a>(
        &'a self,
        account_id: String,
        reason: AccountRejectReason
    ) -> RejectAccountCall<'a> {
        RejectAccountCall::new(self, account_id, reason)
    }

    /// https://stripe.com/docs/api#list_accounts
    pub fn list_accounts<'a>(&'a self) -> ListAccountsCall<'a> {
        ListAccountsCall::new(self)
    }

    /// https://stripe.com/docs/api#create_fee_refund
    pub fn create_fee_refund<'a>(&'a self, application_fee_id: String) -> CreateFeeRefundCall<'a> {
        CreateFeeRefundCall::new(self, application_fee_id)
    }

    /// https://stripe.com/docs/api#retrieve_fee_refund
    pub fn retrieve_fee_refund<'a>(
        &'a self,
        application_fee_id: String,
        refund_id: String
    ) -> RetrieveFeeRefundCall<'a> {
        RetrieveFeeRefundCall::new(self, application_fee_id, refund_id)
    }

    /// https://stripe.com/docs/api#update_fee_refund
    pub fn update_fee_refund<'a>(
        &'a self,
        application_fee_id: String,
        refund_id: String
    ) -> UpdateFeeRefundCall<'a> {
        UpdateFeeRefundCall::new(self, application_fee_id, refund_id)
    }

    /// https://stripe.com/docs/api#list_fee_refunds
    pub fn list_fee_refunds<'a>(&'a self, application_fee_id: String) -> ListFeeRefundsCall<'a> {
        ListFeeRefundsCall::new(self, application_fee_id)
    }

    /// https://stripe.com/docs/api#retrieve_application_fee
    pub fn retrieve_application_fee<'a>(&'a self, fee_id: String) -> RetrieveApplicationFeeCall<'a> {
        RetrieveApplicationFeeCall::new(self, fee_id)
    }

    /// https://stripe.com/docs/api#list_application_fees
    pub fn list_application_fees<'a>(&'a self) -> ListApplicationFeesCall<'a> {
        ListApplicationFeesCall::new(self)
    }

    /// https://stripe.com/docs/api#list_country_specs
    pub fn list_country_specs<'a>(&'a self) -> ListCountrySpecCall<'a> {
        ListCountrySpecCall::new(self)
    }

    /// https://stripe.com/docs/api#retrieve_country_spec
    pub fn retrieve_country_spec<'a>(&'a self, iso_code: String) -> RetrieveCountrySpecCall<'a> {
        RetrieveCountrySpecCall::new(self, iso_code)
    }

    /// https://stripe.com/docs/api#account_create_bank_account
    pub fn account_create_bank_account<'a>(
        &'a self,
        account_id: String
    ) -> AccountCreateBankAccountCall<'a> {
        AccountCreateBankAccountCall::new(self, account_id)
    }

    /// https://stripe.com/docs/api#account_retrieve_bank_account
    pub fn account_retrieve_bank_account<'a>(
        &'a self,
        account_id: String,
        bank_account_id: String
    ) -> AccountRetrieveBankAccountCall<'a> {
        AccountRetrieveBankAccountCall::new(self, account_id, bank_account_id)
    }

    /// https://stripe.com/docs/api#account_update_bank_account
    pub fn account_update_bank_account<'a>(
        &'a self,
        account_id: String,
        bank_account_id: String
    ) -> AccountUpdateBankAccountCall<'a> {
        AccountUpdateBankAccountCall::new(self, account_id, bank_account_id)
    }

    /// https://stripe.com/docs/api#account_delete_bank_account
    pub fn account_delete_bank_account<'a>(
        &'a self,
        account_id: String,
        bank_account_id: String
    ) -> AccountDeleteBankAccountCall<'a> {
        AccountDeleteBankAccountCall::new(self, account_id, bank_account_id)
    }

    /// https://stripe.com/docs/api#account_list_bank_accounts
    pub fn account_list_bank_accounts<'a>(
        &'a self,
        account_id: String
    ) -> AccountListBankAccountsCall<'a> {
        AccountListBankAccountsCall::new(self, account_id)
    }

    /// https://stripe.com/docs/api#account_create_card
    pub fn account_create_card<'a>(&'a self, account_id: String) -> AccountCreateCardCall<'a> {
        AccountCreateCardCall::new(self, account_id)
    }

    /// https://stripe.com/docs/api#account_retrieve_card
    pub fn account_retrieve_card<'a>(
        &'a self,
        account_id: String,
        card_id: String
    ) -> AccountRetrieveCardCall<'a> {
        AccountRetrieveCardCall::new(self, account_id, card_id)
    }

    /// https://stripe.com/docs/api#account_update_card
    pub fn account_update_card<'a>(
        &'a self,
        account_id: String,
        card_id: String
    ) -> AccountUpdateCardCall<'a> {
        AccountUpdateCardCall::new(self, account_id, card_id)
    }

    /// https://stripe.com/docs/api#account_delete_card
    pub fn account_delete_card<'a>(
        &'a self,
        account_id: String,
        card_id: String
    ) -> AccountDeleteCardCall<'a> {
        AccountDeleteCardCall::new(self, account_id, card_id)
    }

    /// https://stripe.com/docs/api#account_list_cards
    pub fn account_list_cards<'a>(&'a self, account_id: String) -> AccountListCardsCall<'a> {
        AccountListCardsCall::new(self, account_id)
    }

    /// https://stripe.com/docs/api#customer_create_bank_account
    pub fn customer_create_bank_account<'a>(
        &'a self,
        customer_id: String
    ) -> CustomerCreateBankAccountCall<'a> {
        CustomerCreateBankAccountCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#customer_retrieve_bank_account
    pub fn customer_retrieve_bank_account<'a>(
        &'a self,
        customer_id: String,
        bank_account_id: String
    ) -> CustomerRetrieveBankAccountCall<'a> {
        CustomerRetrieveBankAccountCall::new(self, customer_id, bank_account_id)
    }

    /// https://stripe.com/docs/api#customer_update_bank_account
    pub fn customer_update_bank_account<'a>(
        &'a self,
        customer_id: String,
        bank_account_id: String
    ) -> CustomerUpdateBankAccountCall<'a> {
        CustomerUpdateBankAccountCall::new(self, customer_id, bank_account_id)
    }

    /// https://stripe.com/docs/api#customer_delete_bank_account
    pub fn customer_delete_bank_account<'a>(
        &'a self,
        customer_id: String,
        bank_account_id: String
    ) -> CustomerDeleteBankAccountCall<'a> {
        CustomerDeleteBankAccountCall::new(self, customer_id, bank_account_id)
    }

    /// https://stripe.com/docs/api#customer_list_bank_accounts
    pub fn customer_list_bank_accounts<'a>(
        &'a self,
        customer_id: String
    ) -> CustomerListBankAccountsCall<'a> {
        CustomerListBankAccountsCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#create_bitcoin_receiver
    pub fn create_bitcoin_receiver<'a>(
        &'a self,
        amount: i64,
        currency: Currency,
        email: String
    ) -> CreateBitcoinReceiverCall<'a> {
        CreateBitcoinReceiverCall::new(self, amount, currency, email)
    }

    /// https://stripe.com/docs/api#retrieve_bitcoin_receiver
    pub fn retrieve_bitcoin_receiver<'a>(&'a self, receiver_id: String) -> RetrieveBitcoinReceiverCall<'a> {
        RetrieveBitcoinReceiverCall::new(self, receiver_id)
    }

    /// https://stripe.com/docs/api#list_bitcoin_receivers
    pub fn list_bitcoin_receivers<'a>(&'a self) -> ListBitcoinReceiversCall<'a> {
        ListBitcoinReceiversCall::new(self)
    }

    /// https://stripe.com/docs/api#create_card
    pub fn create_card<'a>(
        &'a self,
        customer_id: String
    ) -> CustomerCreateCardCall<'a> {
        CustomerCreateCardCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#retrieve_card
    pub fn retrieve_card<'a>(
        &'a self,
        customer_id: String,
        card_id: String
    ) -> CustomerRetrieveCardCall<'a> {
        CustomerRetrieveCardCall::new(self, customer_id, card_id)
    }

    /// https://stripe.com/docs/api#update_card
    pub fn update_card<'a>(
        &'a self,
        customer_id: String,
        card_id: String
    ) -> CustomerUpdateCardCall<'a> {
        CustomerUpdateCardCall::new(self, customer_id, card_id)
    }

    /// https://stripe.com/docs/api#delete_card
    pub fn delete_card<'a>(
        &'a self,
        customer_id: String,
        card_id: String
    ) -> CustomerDeleteCardCall<'a> {
        CustomerDeleteCardCall::new(self, customer_id, card_id)
    }

    /// https://stripe.com/docs/api#list_cards
    pub fn list_cards<'a>(&'a self, customer_id: String) -> CustomerListCardsCall<'a> {
        CustomerListCardsCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#create_order
    pub fn create_order<'a>(&'a self, currency: Currency) -> CreateOrderCall<'a> {
        CreateOrderCall::new(self, currency)
    }

    /// https://stripe.com/docs/api#retrieve_order
    pub fn retrieve_order<'a>(&'a self, order_id: String) -> RetrieveOrderCall<'a> {
        RetrieveOrderCall::new(self, order_id)
    }

    /// https://stripe.com/docs/api#update_order
    pub fn update_order<'a>(&'a self, order_id: String) -> UpdateOrderCall<'a> {
        UpdateOrderCall::new(self, order_id)
    }

    /// https://stripe.com/docs/api#pay_order
    pub fn pay_order<'a>(&'a self, order_id: String) -> PayOrderCall<'a> {
        PayOrderCall::new(self, order_id)
    }

    /// https://stripe.com/docs/api#list_orders
    pub fn list_orders<'a>(&'a self) -> ListOrdersCall<'a> {
        ListOrdersCall::new(self)
    }

    /// https://stripe.com/docs/api#create_product
    pub fn create_product<'a>(&'a self, name: String) -> CreateProductCall<'a> {
        CreateProductCall::new(self, name)
    }

    /// https://stripe.com/docs/api#retrieve_product
    pub fn retrieve_product<'a>(&'a self, product_id: String) -> RetrieveProductCall<'a> {
        RetrieveProductCall::new(self, product_id)
    }

    /// https://stripe.com/docs/api#update_product
    pub fn update_product<'a>(&'a self, product_id: String) -> UpdateProductCall<'a> {
        UpdateProductCall::new(self, product_id)
    }

    /// https://stripe.com/docs/api#delete_product
    pub fn delete_product<'a>(&'a self, product_id: String) -> DeleteProductCall<'a> {
        DeleteProductCall::new(self, product_id)
    }

    /// https://stripe.com/docs/api#list_products
    pub fn list_products<'a>(&'a self) -> ListProductsCall<'a> {
        ListProductsCall::new(self)
    }

    /// https://stripe.com/docs/api#create_sku
    pub fn create_sku<'a>(
        &'a self,
        currency: Currency,
        inventory: Inventory,
        price: i64,
        product: String
    ) -> CreateSkuCall<'a> {
        CreateSkuCall::new(self, currency, inventory, price, product)
    }

    /// https://stripe.com/docs/api#retrieve_sku
    pub fn retrieve_sku<'a>(&'a self, sku_id: String) -> RetrieveSkuCall<'a> {
        RetrieveSkuCall::new(self, sku_id)
    }

    /// https://stripe.com/docs/api#update_sku
    pub fn update_sku<'a>(&'a self, sku_id: String) -> UpdateSkuCall<'a> {
        UpdateSkuCall::new(self, sku_id)
    }

    /// https://stripe.com/docs/api#list_skus
    pub fn list_skus<'a>(&'a self) -> ListSkusCall<'a> {
        ListSkusCall::new(self)
    }

    /// https://stripe.com/docs/api#delete_sku
    pub fn delete_sku<'a>(&'a self, sku_id: String) -> DeleteSkuCall<'a> {
        DeleteSkuCall::new(self, sku_id)
    }

    /// https://stripe.com/docs/api#create_coupon
    pub fn create_coupon<'a>(&'a self, duration: CouponDuration) -> CreateCouponCall<'a> {
        CreateCouponCall::new(self, duration)
    }

    /// https://stripe.com/docs/api#retrieve_coupon
    pub fn retrieve_coupon<'a>(&'a self, coupon_id: String) -> RetrieveCouponCall<'a> {
        RetrieveCouponCall::new(self, coupon_id)
    }

    /// https://stripe.com/docs/api#update_coupon
    pub fn update_coupon<'a>(&'a self, coupon_id: String) -> UpdateCouponCall<'a> {
        UpdateCouponCall::new(self, coupon_id)
    }

    /// https://stripe.com/docs/api#delete_coupon
    pub fn delete_coupon<'a>(&'a self, coupon_id: String) -> DeleteCouponCall<'a> {
        DeleteCouponCall::new(self, coupon_id)
    }

    /// https://stripe.com/docs/api#list_coupons
    pub fn list_coupons<'a>(&'a self) -> ListCouponsCall<'a> {
        ListCouponsCall::new(self)
    }

    /// https://stripe.com/docs/api#delete_discount
    pub fn delete_discount<'a>(&'a self, customer_id: String) -> DeleteCustomerDiscountCall<'a> {
        DeleteCustomerDiscountCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#delete_subscription_discount
    pub fn delete_subscription_discount<'a>(
        &'a self,
        customer_id: String,
        subscription_id: String
    ) -> DeleteSubscriptionDiscountCall<'a> {
        DeleteSubscriptionDiscountCall::new(self, customer_id, subscription_id)
    }

    /// https://stripe.com/docs/api#create_invoice
    pub fn create_invoice<'a>(&'a self, customer_id: String) -> CreateInvoiceCall<'a> {
        CreateInvoiceCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#retrieve_invoice
    pub fn retrieve_invoice<'a>(&'a self, invoice_id: String) -> RetrieveInvoiceCall<'a> {
        RetrieveInvoiceCall::new(self, invoice_id)
    }

    /// https://stripe.com/docs/api#invoice_lines
    pub fn retrieve_invoice_lines<'a>(&'a self, invoice_id: String) -> RetrieveInvoiceLinesCall<'a> {
        RetrieveInvoiceLinesCall::new(self, invoice_id)
    }

    /// https://stripe.com/docs/api#upcoming_invoice
    pub fn retrieve_upcoming_invoice<'a>(
        &'a self,
        customer_id: String
    ) -> RetrieveUpcomingInvoiceCall<'a> {
        RetrieveUpcomingInvoiceCall::new(self, customer_id)
    }

    /// https://stripe.com/docs/api#update_invoice
    pub fn update_invoice<'a>(&'a self, invoice_id: String) -> UpdateInvoiceCall<'a> {
        UpdateInvoiceCall::new(self, invoice_id)
    }

    /// https://stripe.com/docs/api#pay_invoice
    pub fn pay_invoice<'a>(&'a self, invoice_id: String) -> PayInvoiceCall<'a> {
        PayInvoiceCall::new(self, invoice_id)
    }

    /// https://stripe.com/docs/api#list_invoices
    pub fn list_invoices<'a>(&'a self) -> ListInvoicesCall<'a> {
        ListInvoicesCall::new(self)
    }

    /// https://stripe.com/docs/api#create_invoiceitem
    pub fn create_invoiceitem<'a>(
        &'a self,
        amount: i64,
        currency: Currency,
        customer_id: String
    ) -> CreateInvoiceitemCall<'a> {
        CreateInvoiceitemCall::new(self, amount, currency, customer_id)
    }

    /// https://stripe.com/docs/api#retrieve_invoiceitem
    pub fn retrieve_invoiceitem<'a>(
        &'a self,
        invoiceitem_id: String
    ) -> RetrieveInvoiceitemCall<'a> {
        RetrieveInvoiceitemCall::new(self, invoiceitem_id)
    }

    /// https://stripe.com/docs/api#update_invoiceitem
    pub fn update_invoiceitem<'a>(
        &'a self,
        invoiceitem_id: String
    ) -> UpdateInvoiceitemCall<'a> {
        UpdateInvoiceitemCall::new(self, invoiceitem_id)
    }

    /// https://stripe.com/docs/api#delete_invoiceitem
    pub fn delete_invoiceitem<'a>(
        &'a self,
        invoiceitem_id: String
    ) -> DeleteInvoiceitemCall<'a> {
        DeleteInvoiceitemCall::new(self, invoiceitem_id)
    }

    /// https://stripe.com/docs/api#list_invoiceitems
    pub fn list_invoiceitems<'a>(&'a self) -> ListInvoiceitemsCall<'a> {
        ListInvoiceitemsCall::new(self)
    }

    /// https://stripe.com/docs/api#create_plan
    pub fn create_plan<'a>(
        &'a self,
        plan_id: String,
        amount: i64,
        currency: Currency,
        interval: Interval,
        name: String
    ) -> CreatePlanCall<'a> {
        CreatePlanCall::new(self, plan_id, amount, currency, interval, name)
    }

    /// https://stripe.com/docs/api#retrieve_plan
    pub fn retrieve_plan<'a>(&'a self, plan_id: String) -> RetrievePlanCall<'a> {
        RetrievePlanCall::new(self, plan_id)
    }

    /// https://stripe.com/docs/api#update_plan
    pub fn update_plan<'a>(&'a self, plan_id: String) -> UpdatePlanCall<'a> {
        UpdatePlanCall::new(self, plan_id)
    }

    /// https://stripe.com/docs/api#delete_plan
    pub fn delete_plan<'a>(&'a self, plan_id: String) -> DeletePlanCall<'a> {
        DeletePlanCall::new(self, plan_id)
    }

    /// https://stripe.com/docs/api#list_plans
    pub fn list_plans<'a>(&'a self) -> ListPlansCall<'a> {
        ListPlansCall::new(self)
    }

    /// https://stripe.com/docs/api#create_subscription
    pub fn create_subscription<'a>(
        &'a self,
        customer_id: String,
        plan_id: String
    ) -> CreateSubscriptionCall<'a> {
        CreateSubscriptionCall::new(self, customer_id, plan_id)
    }

    /// https://stripe.com/docs/api#retrieve_subscription
    pub fn retrieve_subscription<'a>(
        &'a self,
        customer_id: String,
        subscription_id: String
    ) -> RetrieveSubscriptionCall<'a> {
        RetrieveSubscriptionCall::new(self, customer_id, subscription_id)
    }

    /// https://stripe.com/docs/api#update_subscription
    pub fn update_subscription<'a>(
        &'a self,
        customer_id: String,
        subscription_id: String
    ) -> UpdateSubscriptionCall<'a> {
        UpdateSubscriptionCall::new(self, customer_id, subscription_id)
    }

    /// https://stripe.com/docs/api#cancel_subscription
    pub fn cancel_subscription<'a>(
        &'a self,
        customer_id: String,
        subscription_id: String
    ) -> CancelSubscriptionCall<'a> {
        CancelSubscriptionCall::new(self, customer_id, subscription_id)
    }

    /// https://stripe.com/docs/api#list_subscriptions
    pub fn list_subscriptions<'a>(
        &'a self,
        customer_id: String
    ) -> ListActiveSubscriptionsCall<'a> {
        ListActiveSubscriptionsCall::new(self, customer_id)
    }

    pub fn get<T: Deserialize, E: Display>(
        &self,
        endpoint: E,
        args: &UrlEncodable
    ) -> Result<T> {
        let params = args.encoded_string();
        let url = if params.is_empty() {
            StripeClient::endpoint(endpoint)
        } else {
            StripeClient::endpoint(&format!("{}?{}", endpoint, params))
        };
        let res = self.client.get(&url)
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response::<T>(res)
    }

    pub fn post<T: Deserialize, E: Display>(
        &self,
        endpoint: E,
        args: &UrlEncodable,
    ) -> Result<T> {
        let body = args.encoded_string();
        let mut req_builder = self.client.post(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers());

        if !body.is_empty() {
            req_builder = req_builder.body(body.as_bytes());
        }
        StripeClient::parse_response::<T>(req_builder.send()?)
    }

    pub fn post_with_custom_headers<T: Deserialize, E: Display>(
        &self,
        endpoint: E,
        args: &UrlEncodable,
        custom_headers: Headers
    ) -> Result<T> {
        let body = args.encoded_string();
        let mut req_builder = self.client.post(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers())
            .headers(custom_headers);

        if !body.is_empty() {
            req_builder = req_builder.body(body.as_bytes());
        }
        StripeClient::parse_response::<T>(req_builder.send()?)
    }

    pub fn delete<T: Deserialize, E: Display>(
        &self,
        endpoint: E
    ) -> Result<T> {
        let res = self.client.delete(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers())
            .send()?;
        StripeClient::parse_response(res)
    }

    pub fn delete_with_args<T: Deserialize, E: Display>(
        &self,
        endpoint: E,
        args: &UrlEncodable
    ) -> Result<T> {
        let body = args.encoded_string();
        let mut req = self.client.delete(&StripeClient::endpoint(endpoint))
            .headers(self.default_headers());

        if !body.is_empty() {
            req = req.body(body.as_bytes());
        }
        let res = req.send()?;
        StripeClient::parse_response(res)
    }

    pub fn retrieve_all<T: StripeObject>(
        &self,
        list: ApiList<T>,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<Vec<T>> {
        let mut data = vec![];
        let mut page = list;
        loop {
            data.append(&mut page.data);

            if page.has_more {
                page = self.fetch_next_page(&page, args)?;
            } else {
                break;
            }
        }
        Ok(data)
    }

    pub fn fetch_next_page<T: StripeObject>(
        &self,
        list: &ApiList<T>,
        args: Option<&BTreeMap<String, String>>
    ) -> Result<ApiList<T>> {
        let mut args = args.map(|a| a.clone()).unwrap_or(BTreeMap::new());
        if list.has_more {
            if let Some(stripe_object) = list.data.last() {
                args.insert(String::from("starting_after"), String::from(stripe_object.id()));
            }
            self.get(&list.url, &args)
        } else {
            Ok(ApiList {
                data: vec![],
                has_more: false,
                total_count: list.total_count,
                url: (&list.url).clone()
            })
        }
    }

    fn endpoint<E: Display>(endpoint: E) -> String {
        let endpoint = endpoint.to_string();
        if endpoint.starts_with("/") {
            format!("{}{}", BASE_URL, endpoint)
        } else {
            format!("{}/{}", BASE_URL, endpoint)
        }
    }

    fn parse_response<T: Deserialize>(mut res: Response) -> Result<T> {
        match res.status {
            StatusCode::Ok => {
                let mut body = String::new();
                res.read_to_string(&mut body)?;
                let t = serde_json::from_str(&body)?;
                Ok(t)
            },
            _ => {
                let mut body = String::new();
                res.read_to_string(&mut body)?;
                let err = serde_json::from_str::<stripe_error::StripeErrorWrapper>(&body)?.error;
                Err(Error::StripeError(err))
            }
        }
    }

    fn default_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
                username: self.key.clone(),
                password: None
        }));
        headers.set(StripeVersion::new(API_VERSION));
        headers
    }
}

#[cfg(test)]
mod test {
    use hyper::Client;
    use super::StripeClient;
    use std::error::Error;

    #[test]
    fn it_works() {
        let key = "sk_test_GBE7dKKNklsixtSM4VG876qt";
        // let key = "not_a_key";
        let client = StripeClient::new(&key);
        match client.fetch_account() {
            Ok(account) => println!("{:?}", account),
            Err(err) => {
                println!("{}", &err);
                println!("{}", err.description());
            }
        }
        panic!("boom");
    }
}

mod account;
mod account_reject_reason;
mod account_update;
mod address;
mod api_list;
mod application_fee;
mod balance;
mod balance_transaction;
mod bank_account;
mod bitcoin_receiver;
mod card;
mod charge;
mod country_spec;
mod coupon;
mod create_charge;
mod customer;
mod date_of_birth;
mod delete;
mod discount;
mod dispute;
mod event;
mod fee_refund;
mod interval;
mod legal_entity;
mod money;
mod owner;
mod recipient;
mod refund;
mod shipping;
mod source;
mod source_type;
mod subscription;
mod token;
mod transfer;
mod transfer_reversal;
mod update_charge;
mod verification;

pub use model::account::*;
pub use model::account_reject_reason::*;
pub use model::account_update::*;
pub use model::address::*;
pub use model::api_list::*;
pub use model::application_fee::*;
pub use model::balance::*;
pub use model::balance_transaction::*;
pub use model::bank_account::*;
pub use model::bitcoin_receiver::*;
pub use model::card::*;
pub use model::charge::*;
pub use model::country_spec::*;
pub use model::coupon::*;
pub use model::create_charge::*;
pub use model::customer::*;
pub use model::date_of_birth::*;
pub use model::delete::*;
pub use model::discount::*;
pub use model::dispute::*;
pub use model::event::*;
pub use model::fee_refund::*;
pub use model::interval::*;
pub use model::legal_entity::*;
pub use model::money::*;
pub use model::owner::*;
pub use model::recipient::*;
pub use model::refund::*;
pub use model::shipping::*;
pub use model::source::*;
pub use model::source_type::*;
pub use model::subscription::*;
pub use model::token::*;
pub use model::transfer::*;
pub use model::transfer_reversal::*;
pub use model::update_charge::*;
pub use model::verification::*;

use serde::de::Deserialize;
use std::fmt::Debug;

pub trait StripeObject : Clone + Debug + Deserialize {
    fn id(&self) -> &str;
}

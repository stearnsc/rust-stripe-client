pub mod account;
pub mod application_fee;
pub mod balance;
pub mod bitcoin;
pub mod charge;
pub mod country_spec;
pub mod coupon;
pub mod customer;
pub mod discount;
pub mod dispute;
pub mod event;
pub mod fee_refund;
pub mod invoice;
pub mod invoiceitem;
pub mod order;
pub mod plan;
pub mod product;
pub mod refund;
pub mod sku;
pub mod subscription;
pub mod token;
pub mod transfer;
pub mod transfer_reversal;

pub use self::account::*;
pub use self::application_fee::*;
pub use self::balance::*;
pub use self::bitcoin::*;
pub use self::charge::*;
pub use self::country_spec::*;
pub use self::coupon::*;
pub use self::customer::*;
pub use self::discount::*;
pub use self::dispute::*;
pub use self::event::*;
pub use self::fee_refund::*;
pub use self::invoice::*;
pub use self::invoiceitem::*;
pub use self::order::*;
pub use self::plan::*;
pub use self::product::*;
pub use self::refund::*;
pub use self::sku::*;
pub use self::subscription::*;
pub use self::token::*;
pub use self::transfer::*;
pub use self::transfer_reversal::*;

use Result;

pub trait ApiCall<T> {
    fn call(self) -> Result<T>;
}

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
pub mod invoice;
pub mod invoiceitem;
pub mod order;
pub mod plan;
pub mod product;
pub mod sku;
pub mod subscription;
pub mod token;
pub mod transfer;
pub mod transfer_reversal;

use Result;

trait ApiCall<T> {
    fn call(self) -> Result<T>;
}

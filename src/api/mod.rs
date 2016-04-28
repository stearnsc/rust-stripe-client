pub mod balance;
pub mod charge;
pub mod customer;

use Result;

trait ApiCall<T> {
    fn call(self) -> Result<T>;
}

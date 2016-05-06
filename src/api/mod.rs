pub mod balance;
pub mod charge;
pub mod customer;
pub mod dispute;

use Result;

trait ApiCall<T> {
    fn call(self) -> Result<T>;
}

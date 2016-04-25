use super::address::Address;
use super::date_of_birth::DateOfBirth;
use super::verification::Verification;

#[derive(Clone, Debug, Deserialize)]
pub struct Owner {
    address: Address,
    dob: DateOfBirth,
    first_name: String,
    last_name: String,
    verification: Verification
}

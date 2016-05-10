use super::address::Address;
use super::date_of_birth::DateOfBirth;
use super::verification::Verification;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct Owner {
    address: Option<Address>,
    dob: Option<DateOfBirth>,
    first_name: Option<String>,
    last_name: Option<String>,
    verification: Option<Verification>
}

pub struct NewOwner(Vec<(String, String)>);

impl NewOwner {
    pub fn address(mut self, address: Address) -> Self {
        self.0.extend(UrlEncodable::named("address", &address));
        self
    }

    pub fn dob(mut self, dob: DateOfBirth) -> Self {
        self.0.extend(UrlEncodable::named("dob", &dob));
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.0.push(("first_name".to_string(), first_name));
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.0.push(("last_name".to_string(), last_name));
        self
    }

    pub fn verification(mut self, verification_file_token: String) -> Self {
        self.0.push(("verification".to_string(), verification_file_token));
        self
    }
}

impl UrlEncodable for NewOwner {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        self.0.key_value_pairs()
    }
}



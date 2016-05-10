use serde;
use super::address::Address;
use super::date_of_birth::DateOfBirth;
use super::gender::Gender;
use super::owner::{Owner, NewOwner};
use super::verification::Verification;
use std::collections::BTreeMap;
use std::fmt;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct LegalEntity {
    pub additional_owners: Option<Vec<Owner>>,
    pub address: Option<Address>,
    pub business_name: Option<String>,
    pub business_tax_id_provided: bool,
    pub dob: Option<DateOfBirth>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_address: Option<Address>,
    pub personal_id_number_provided: Option<bool>,
    pub ssn_last_4_provided: Option<bool>,
    #[serde(rename="type")]
    pub entity_type: LegalEntityType,
    pub verification: Verification,
}

#[derive(Clone, Debug)]
pub enum LegalEntityType {
    Individual,
    Company,
    Unknown(String)
}

impl fmt::Display for LegalEntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LegalEntityType::Individual           => write!(f, "individual"),
            LegalEntityType::Company              => write!(f, "company"),
            LegalEntityType::Unknown(ref unknown) => write!(f, "{}", unknown),
        }
    }
}

impl serde::Deserialize for LegalEntityType {
    fn deserialize<D>(deserializer: &mut D) -> Result<LegalEntityType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "individual" => LegalEntityType::Individual,
            "company"    => LegalEntityType::Company,
            unknown      => LegalEntityType::Unknown(String::from(unknown)),
        })
    }
}

pub struct NewLegalEntity(BTreeMap<String, String>);

impl NewLegalEntity {
    pub fn additional_owners(mut self, additional_owners: Vec<NewOwner>) -> Self {
        self.0.extend(UrlEncodable::structured_list("additional_owners", &additional_owners));
        self
    }

    pub fn address(mut self, address: Address) -> Self {
        self.0.extend(UrlEncodable::named("address", &address));
        self
    }

    pub fn business_name(mut self, business_name: String) -> Self {
        self.0.insert("business_name".to_string(), business_name);
        self
    }

    pub fn business_tax_id(mut self, business_tax_id: String) -> Self {
        self.0.insert("business_tax_id".to_string(), business_tax_id);
        self
    }

    pub fn business_vat_id(mut self, business_vat_id: String) -> Self {
        self.0.insert("business_vat_id".to_string(), business_vat_id);
        self
    }

    pub fn dob(mut self, dob: DateOfBirth) -> Self {
        self.0.extend(UrlEncodable::named("dob", &dob));
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.0.insert("first_name".to_string(), first_name);
        self
    }

    pub fn gender(mut self, gender: Gender) -> Self {
        self.0.insert("gender".to_string(), gender.to_string());
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.0.insert("last_name".to_string(), last_name);
        self
    }

    pub fn maiden_name(mut self, maiden_name: String) -> Self {
        self.0.insert("maiden_name".to_string(), maiden_name);
        self
    }

    pub fn personal_address(mut self, personal_address: Address) -> Self {
        self.0.extend(UrlEncodable::named("personal_address", &personal_address));
        self
    }

    pub fn ssn_last_4(mut self, ssn_last_4: String) -> Self {
        self.0.insert("ssn_last_4".to_string(), ssn_last_4);
        self
    }

    pub fn entity_type(mut self, entity_type: LegalEntityType) -> Self {
        self.0.insert("type".to_string(), entity_type.to_string());
        self
    }

    pub fn verification(mut self, verification_file_id: String) -> Self {
        self.0.insert("verification".to_string(), verification_file_id);
        self
    }
}

impl UrlEncodable for NewLegalEntity {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let NewLegalEntity(ref args) = *self;
        args.key_value_pairs()
    }
}

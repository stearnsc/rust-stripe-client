use custom_ser::*;
use serde;
use super::address::Address;
use super::date_of_birth::DateOfBirth;
use super::owner::Owner;
use super::verification::Verification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEntity {
    #[serde(rename="type")]
    pub entity_type: String,
    pub address: Address,
    pub business_name: String,
    pub business_tax_id_provided: bool,
    pub dob: DateOfBirth,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_address: Option<Address>,
    pub personal_id_number_provided: Option<bool>,
    pub ssn_last_4_provided: Option<bool>,
    pub verification: Verification,
    pub additional_owners: Vec<Owner>
}

#[derive(Debug, Clone)]
pub enum LegalEntityType {
    Individual,
    Company,
    Unknown(String)
}

impl LegalEntityType {
    fn from_str(s: &str) -> LegalEntityType {
        match s {
            "individual" => LegalEntityType::Individual,
            "company"    => LegalEntityType::Company,
            unknown      => LegalEntityType::Unknown(String::from(unknown))
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            LegalEntityType::Individual     => "individual",
            LegalEntityType::Company        => "company",
            LegalEntityType::Unknown(ref s) => s
        })
    }
}

simple_serde!(LegalEntityType, LegalEntityType::to_string, LegalEntityType::from_str);

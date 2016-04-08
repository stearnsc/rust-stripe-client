use serde;
use std::collections::BTreeMap;
use super::customer::Customer;

#[derive(Debug, Clone, Deserialize)]
pub struct Card {
    pub id: String,
    pub account: Option<String>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<Check>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<Check>,
    pub brand: CardBrand,
    pub country: String,
    pub currency: Option<String>,
    pub customer: Option<Customer>,
    pub cvc_check: Option<Check>,
    pub default_for_currency: Option<bool>,
    pub dynamic_last4: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: CardType,
    pub last4: String,
    pub metadata: Option<BTreeMap<String, String>>,
    pub name: Option<String>,
    pub recipient: Option<String>,
    pub tokenization_method: Option<TokenizationMethod>,
}

#[derive(Clone, Debug)]
pub enum Check {
    Pass,
    Fail,
    Unavailable,
    Unchecked,
    Unknown(String),
}

impl serde::Deserialize for Check {
    fn deserialize<D>(deserializer: &mut D) -> Result<Check, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "pass"        => Check::Pass,
            "fail"        => Check::Fail,
            "unavailable" => Check::Unavailable,
            "unchecked"   => Check::Unchecked,
            unknown       => Check::Unknown(String::from(unknown)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
    Unknown(String)
}

impl serde::Deserialize for CardType {
    fn deserialize<D>(deserializer: &mut D) -> Result<CardType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "credit"  => CardType::Credit,
            "debit"   => CardType::Debit,
            "prepaid" => CardType::Prepaid,
            unknown   => CardType::Unknown(String::from(unknown)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum TokenizationMethod {
    AndroidPay,
    ApplePay,
    Unknown(String),
}

impl serde::Deserialize for TokenizationMethod {
    fn deserialize<D>(deserializer: &mut D) -> Result<TokenizationMethod, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "android_pay" => TokenizationMethod::AndroidPay,
            "apple_pay"   => TokenizationMethod::ApplePay,
            unknown       => TokenizationMethod::Unknown(String::from(unknown)),
        })
    }
}

#[derive(Clone, Debug)]
pub enum CardBrand {
    Visa,
    AmericanExpress,
    MasterCard,
    Discover,
    Jcb,
    DinersClub,
    Unknown(String)
}

impl serde::Deserialize for CardBrand {
    fn deserialize<D>(deserializer: &mut D) -> Result<CardBrand, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "Visa"             => CardBrand::Visa,
            "American Express" => CardBrand::AmericanExpress,
            "MasterCard"       => CardBrand::MasterCard,
            "Discover"         => CardBrand::Discover,
            "JCB"              => CardBrand::Jcb,
            "Diners Club"      => CardBrand::DinersClub,
            unknown            => CardBrand::Unknown(String::from(unknown)),
        })
    }
}

use custom_ser::*;
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

impl Check {
    fn from_str(s: &str) -> Check {
        match s {
            "pass"        => Check::Pass,
            "fail"        => Check::Fail,
            "unavailable" => Check::Unavailable,
            "unchecked"   => Check::Unchecked,
            unknown       => Check::Unknown(String::from(unknown))
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            Check::Pass           => "pass",
            Check::Fail           => "fail",
            Check::Unavailable    => "unavailable",
            Check::Unchecked      => "unchecked",
            Check::Unknown(ref s) => s,
        })
    }
}

simple_serde!(Check, Check::to_string, Check::from_str);

#[derive(Clone, Debug)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
    Unknown(String)
}

impl CardType {
    fn from_str(s: &str) -> CardType {
        match s {
            "credit"  => CardType::Credit,
            "debit"   => CardType::Debit,
            "prepaid" => CardType::Prepaid,
            unknown   => CardType::Unknown(String::from(unknown)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            CardType::Credit         => "credit",
            CardType::Debit          => "debit",
            CardType::Prepaid        => "prepaid",
            CardType::Unknown(ref s) => s,
        })
    }
}

simple_serde!(CardType, CardType::to_string, CardType::from_str);

#[derive(Clone, Debug)]
pub enum TokenizationMethod {
    AndroidPay,
    ApplePay,
    Unknown(String),
}

impl TokenizationMethod {
    fn from_str(s: &str) -> TokenizationMethod {
        match s {
            "android_pay" => TokenizationMethod::AndroidPay,
            "apple_pay"   => TokenizationMethod::ApplePay,
            unknown       => TokenizationMethod::Unknown(String::from(unknown)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            TokenizationMethod::AndroidPay     => "android_pay",
            TokenizationMethod::ApplePay       => "apple_pay",
            TokenizationMethod::Unknown(ref s) => s,
        })
    }
}

simple_serde!(TokenizationMethod, TokenizationMethod::to_string, TokenizationMethod::from_str);

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

impl CardBrand {
    fn from_str(s: &str) -> CardBrand {
        match s {
            "Visa"             => CardBrand::Visa,
            "American Express" => CardBrand::AmericanExpress,
            "MasterCard"       => CardBrand::MasterCard,
            "Discover"         => CardBrand::Discover,
            "JCB"              => CardBrand::Jcb,
            "Diners Club"      => CardBrand::DinersClub,
            unknown            => CardBrand::Unknown(String::from(unknown)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            CardBrand::Visa            => "Visa",
            CardBrand::AmericanExpress => "American Express",
            CardBrand::MasterCard      => "MasterCard",
            CardBrand::Discover        => "Discover",
            CardBrand::Jcb             => "JCB",
            CardBrand::DinersClub      => "Diners Club",
            CardBrand::Unknown(ref s)  => s,
        })
    }
}

simple_serde!(CardBrand, CardBrand::to_string, CardBrand::from_str);

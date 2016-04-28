use serde;
use std::collections::BTreeMap;
use super::currency::Currency;
use super::customer::Customer;
use super::StripeObject;
use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
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

impl StripeObject for Card {
    fn id(&self) -> &str {
        &self.id
    }
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

#[derive(Clone, Debug)]
pub struct NewCard {
    pub exp_month: i64,
    pub exp_year: i64,
    pub number: String,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub currency: Option<Currency>,
    pub cvc: Option<i64>,
    pub default_for_currency: Option<bool>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub name: Option<String>
}

impl NewCard {
    pub fn new(number: String, exp_month: i64, exp_year: i64) -> Self {
        NewCard {
            exp_month: exp_month,
            exp_year: exp_year,
            number: number,
            address_city: None,
            address_country: None,
            address_line1: None,
            address_line2: None,
            address_state: None,
            address_zip: None,
            currency: None,
            cvc: None,
            default_for_currency: None,
            metadata: None,
            name: None,
        }
    }

    pub fn address_city(mut self, address_city: String) -> Self {
        self.address_city = Some(address_city);
        self
    }

    pub fn address_country(mut self, address_country: String) -> Self {
        self.address_country = Some(address_country);
        self
    }

    pub fn address_line1(mut self, address_line1: String) -> Self {
        self.address_line1 = Some(address_line1);
        self
    }

    pub fn address_line2(mut self, address_line2: String) -> Self {
        self.address_line2 = Some(address_line2);
        self
    }

    pub fn address_state(mut self, address_state: String) -> Self {
        self.address_state = Some(address_state);
        self
    }

    pub fn address_zip(mut self, address_zip: String) -> Self {
        self.address_zip = Some(address_zip);
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn cvc(mut self, cvc: i64) -> Self {
        self.cvc = Some(cvc);
        self
    }

    pub fn default_for_currency(mut self, default_for_currency: bool) -> Self {
        self.default_for_currency = Some(default_for_currency);
        self
    }

    pub fn metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

impl UrlEncodable for NewCard {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let NewCard  {
            ref exp_month, ref exp_year, ref number, ref address_city, ref address_country,
            ref address_line1, ref address_line2, ref address_state, ref address_zip,
            ref currency, ref cvc, ref default_for_currency, ref metadata, ref name
        } = *self;

        let mut v = vec![
            ("exp_month".to_string(), exp_month.to_string()),
            ("exp_year".to_string(), exp_year.to_string()),
            ("number".to_string(), number.to_string()),
        ];
        if let Some(ref address_city) = *address_city {
            v.push(("address_city".to_string(), address_city.to_string()))
        }
        if let Some(ref address_country) = *address_country {
            v.push(("address_country".to_string(), address_country.to_string()))
        }
        if let Some(ref address_line1) = *address_line1 {
            v.push(("address_line1".to_string(), address_line1.to_string()))
        }
        if let Some(ref address_line2) = *address_line2 {
            v.push(("address_line2".to_string(), address_line2.to_string()))
        }
        if let Some(ref address_state) = *address_state {
            v.push(("address_state".to_string(), address_state.to_string()))
        }
        if let Some(ref address_zip) = *address_zip {
            v.push(("address_zip".to_string(), address_zip.to_string()))
        }
        if let Some(ref currency) = *currency {
            v.push(("currency".to_string(), currency.to_string()))
        }
        if let Some(ref cvc) = *cvc {
            v.push(("cvc".to_string(), cvc.to_string()))
        }
        if let Some(ref default_for_currency) = *default_for_currency {
            v.push(("default_for_currency".to_string(), default_for_currency.to_string()))
        }
        if let Some(ref metadata) = *metadata {
            v.extend(UrlEncodable::named("metadata", metadata).key_value_pairs())
        }
        if let Some(ref name) = *name {
            v.push(("name".to_string(), name.to_string()))
        }
        v
    }
}

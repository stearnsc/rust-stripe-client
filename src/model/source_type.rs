use serde;
use std::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum SourceType {
    Card,
    BankAccount,
    BitcoinReceiver,
    AlipayAccount,
    Other(String)
}

impl serde::Deserialize for SourceType {
    fn deserialize<D>(deserializer: &mut D) -> Result<SourceType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "card"             => SourceType::Card,
            "bank_account"     => SourceType::BankAccount,
            "bitcoin_receiver" => SourceType::BitcoinReceiver,
            "alipay_account"   => SourceType::AlipayAccount,
            other              => SourceType::Other(String::from(other)),
        })
    }
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SourceType::Card            => write!(f, "card"),
            SourceType::BankAccount     => write!(f, "bank_account"),
            SourceType::BitcoinReceiver => write!(f, "bitcoin_receiver"),
            SourceType::AlipayAccount   => write!(f, "alipay_account"),
            SourceType::Other(ref s)    => write!(f, "{}", s),
        }
    }
}

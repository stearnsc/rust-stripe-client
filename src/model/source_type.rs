use custom_ser::*;
use serde;

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub enum SourceType {
    Card,
    BankAccount,
    BitcoinReceiver,
    AlipayAccount,
    Other(String)
}

impl SourceType {
    fn from_str(s: &str) -> SourceType {
        match s {
            "card"             => SourceType::Card,
            "bank_account"     => SourceType::BankAccount,
            "bitcoin_receiver" => SourceType::BitcoinReceiver,
            "alipay_account"   => SourceType::AlipayAccount,
            other              => SourceType::Other(String::from(other))
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            SourceType::Card => "card",
            SourceType::BankAccount => "bank_account",
            SourceType::BitcoinReceiver => "bitcoin_receiver",
            SourceType::AlipayAccount => "alipay_account",
            SourceType::Other(ref other) => other
        })
    }
}

simple_serde!(SourceType, SourceType::to_string, SourceType::from_str);

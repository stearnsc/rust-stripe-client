use serde;

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
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

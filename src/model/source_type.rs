use serde;

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

impl serde::Serialize for SourceType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            SourceType::Card            => str::serialize("card", serializer),
            SourceType::BankAccount     => str::serialize("bank_account", serializer),
            SourceType::BitcoinReceiver => str::serialize("bitcoin_receiver", serializer),
            SourceType::AlipayAccount   => str::serialize("alipay_account", serializer),
            SourceType::Other(ref s)    => str::serialize(s, serializer),
        }
    }
}

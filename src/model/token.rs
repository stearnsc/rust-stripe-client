use serde;
use super::bank_account::BankAccount;
use super::card::Card;
use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct Token {
    pub id: String,
    pub bank_account: Option<BankAccount>,
    pub card: Option<Card>,
    pub client_ip: Option<String>,
    pub created: i64,
    pub livemode: bool,
    #[serde(rename="type")]
    pub token_type: TokenType,
    pub used: bool
}

impl StripeObject for Token {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Card,
    BankAccount,
    Pii,
    Unknown(String),
}

impl serde::Deserialize for TokenType {
    fn deserialize<D>(deserializer: &mut D) -> Result<TokenType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "bank_account" => TokenType::BankAccount,
            "card"         => TokenType::Card,
            "pii"          => TokenType::Pii,
            unknown        => TokenType::Unknown(String::from(unknown)),
        })
    }
}

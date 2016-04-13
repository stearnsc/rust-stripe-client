use serde;
use std::collections::BTreeMap;
use super::account::Account;
use super::api_list::ApiList;
use super::card::Card;
use super::StripeObject;

#[derive(Debug, Clone, Deserialize)]
pub struct Recipient {
    pub id: String,
    pub active_account: Option<Account>,
    pub cards: ApiList<Card>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub livemode: bool,
    pub metadata: Option<BTreeMap<String, String>>,
    pub migrated_to: Option<String>,
    pub name: String,
    #[serde(rename="type")]
    pub recipient_type: RecipientType,
}

impl StripeObject for Recipient {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub enum RecipientType {
    Individual,
    Corporation,
    Unknown(String)
}

impl serde::Deserialize for RecipientType {
    fn deserialize<D>(deserializer: &mut D) -> Result<RecipientType, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "individual"  => RecipientType::Individual,
            "corporation" => RecipientType::Corporation,
            unknown       => RecipientType::Unknown(String::from(unknown)),
        })
    }
}

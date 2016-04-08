use serde::Deserialize;
use serde;
use serde::de::Error;
use serde_json::value::Value;
use super::bitcoin_receiver::BitcoinReceiver;
use super::card::Card;
use super::StripeObject;

type UnknownId = String;

#[derive(Clone, Debug)]
pub enum Source {
    Card(Card),
    BitcoinReceiver(BitcoinReceiver),
    Unknown(Value, String),
}

impl StripeObject for Source {
    fn id(&self) -> &str {
        match *self {
            Source::Card(ref card)                => &card.id,
            Source::BitcoinReceiver(ref receiver) => &receiver.id,
            Source::Unknown(_, ref id)            => id
        }
    }
}

impl Deserialize for Source {
    fn deserialize<D>(deserializer: &mut D) -> Result<Source, D::Error>
        where D: serde::Deserializer
    {
        if let Ok(card) = Card::deserialize(deserializer) {
            return Ok(Source::Card(card));
        }
        if let Ok(receiver) = BitcoinReceiver::deserialize(deserializer) {
            return Ok(Source::BitcoinReceiver(receiver));
        }
        if let Ok(json) = Value::deserialize(deserializer) {
            if let Some(id) = {
                match json.find("id") {
                    Some(&Value::String(ref id)) => Some(id.clone()),
                    _                            => None
                }
            } {
                return Ok(Source::Unknown(json, id))
            }
        }
        Err(D::Error::unknown_variant("Expected card, receiver, or object with id"))
    }
}

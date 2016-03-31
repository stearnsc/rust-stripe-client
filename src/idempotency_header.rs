use hyper::header::{Header, HeaderFormat};
use hyper;
use std::fmt;

#[derive(Clone, Debug)]
pub struct IdempotencyKey {
    pub key: String
}

impl IdempotencyKey {
    pub fn new(key: &str) -> IdempotencyKey {
        IdempotencyKey {
            key: String::from(key)
        }
    }
}

impl Header for IdempotencyKey {
    fn header_name() -> &'static str {
        "Idempotency-Key"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<Self> {
        if raw.len() != 1 {
            Err(hyper::error::Error::Header)
        } else {
            let key = try!(String::from_utf8(raw[0].clone()));
            Ok(IdempotencyKey::new(&key))
        }
    }
}

impl HeaderFormat for IdempotencyKey {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.key)
    }
}

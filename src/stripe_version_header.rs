use hyper::header::{Header, HeaderFormat};
use hyper;
use std::fmt;

#[derive(Clone, Debug)]
pub struct StripeVersion {
    pub key: String
}

impl StripeVersion {
    pub fn new(key: &str) -> StripeVersion {
        StripeVersion {
            key: String::from(key)
        }
    }
}

impl Header for StripeVersion {
    fn header_name() -> &'static str {
        "Stripe-Version"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::error::Result<Self> {
        if raw.len() != 1 {
            Err(hyper::error::Error::Header)
        } else {
            let key = try!(String::from_utf8(raw[0].clone()));
            Ok(StripeVersion::new(&key))
        }
    }
}

impl HeaderFormat for StripeVersion {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.key)
    }
}

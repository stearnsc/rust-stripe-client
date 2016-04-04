use hyper;
use serde_json;
use std;
use super::stripe_error;

#[derive(Debug)]
pub enum Error {
    HttpError(hyper::error::Error),
    SerializationError(serde_json::error::Error),
    IoError(std::io::Error),
    StripeError(stripe_error::StripeError)
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::HttpError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::SerializationError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<stripe_error::StripeError> for Error {
    fn from(err: stripe_error::StripeError) -> Self {
        Error::StripeError(err)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HttpError(ref err) => err.description(),
            Error::SerializationError(ref err) => err.description(),
            Error::IoError(ref err) => err.description(),
            Error::StripeError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::HttpError(ref err) => Some(err),
            Error::SerializationError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
            Error::StripeError(ref err) => Some(err)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::HttpError(ref err)  => write!(f, "Error::HyperError({})", err),
            Error::SerializationError(ref err)  => write!(f, "Error::SerdeError({})", err),
            Error::IoError(ref err)     => write!(f, "Error::IoError({})", err),
            Error::StripeError(ref err) => write!(f, "Error::StripeError({})", err)
        }
    }
}

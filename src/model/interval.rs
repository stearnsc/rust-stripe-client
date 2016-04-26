use serde;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Interval {
    Day,
    Week,
    Month,
    Year,
    Unknown(String),
}

use self::Interval::*;
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Day => write!(f, "day"),
            Week => write!(f, "week"),
            Month => write!(f, "month"),
            Year => write!(f, "year"),
            Unknown(ref s) => write!(f, "{}", s),
        }
    }
}

impl serde::Deserialize for Interval {
    fn deserialize<D>(deserializer: &mut D) -> Result<Interval, D::Error>
        where D: serde::Deserializer
    {
        Ok(match String::deserialize(deserializer)?.as_ref() {
            "day"   => Interval::Day,
            "week"  => Interval::Week,
            "month" => Interval::Month,
            "year"  => Interval::Year,
            unknown => Interval::Unknown(String::from(unknown)),
        })
    }
}

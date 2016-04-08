use custom_ser::*;
use serde;

#[derive(Clone, Debug)]
pub enum Interval {
    Day,
    Week,
    Month,
    Year,
    Unknown(String),
}

impl Interval {
    fn from_str(s: &str) -> Interval {
        match s {
            "day"   => Interval::Day,
            "week"  => Interval::Week,
            "month" => Interval::Month,
            "year"  => Interval::Year,
            unknown => Interval::Unknown(String::from(unknown)),
        }
    }

    fn to_string(&self) -> String {
        String::from(match *self {
            Interval::Day            => "day",
            Interval::Week           => "week",
            Interval::Month          => "month",
            Interval::Year           => "year",
            Interval::Unknown(ref s) => s
        })
    }
}

simple_serde!(Interval, Interval::to_string, Interval::from_str);

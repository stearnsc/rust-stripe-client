use serde;

#[derive(Clone, Debug)]
pub enum Interval {
    Day,
    Week,
    Month,
    Year,
    Unknown(String),
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

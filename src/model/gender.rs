use std::fmt;

pub enum Gender {
    Male,
    Female,
    Other(String)
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Gender::Male             => write!(f, "male"),
            Gender::Female           => write!(f, "female"),
            Gender::Other(ref other) => write!(f, "{}", other),
        }
    }
}

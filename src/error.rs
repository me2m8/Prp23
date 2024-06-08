use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum PersonError {
    FutureBirthday,
    LongDead,
    InvalidPin(String),
    InvalidPinLength(usize),
}

impl Display for PersonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &PersonError::LongDead => write!(f, "Error: Person is long dead"),
            &PersonError::FutureBirthday => write!(f, "Error: Birthday is in the future"),
            &PersonError::InvalidPinLength(n) => write!(f, "Error: InvalidPinLength({})", n),
            PersonError::InvalidPin(s) => write!(f, "Error: InvalidPin({})", s),
        }
    }
}

impl Error for PersonError {}

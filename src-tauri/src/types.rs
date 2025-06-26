use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

impl From<String> for Gender {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "male" | "m" => Gender::Male,
            "female" | "f" => Gender::Female,
            _ => Gender::Other,
        }
    }
}

impl From<&str> for Gender {
    fn from(s: &str) -> Self {
        Gender::from(s.to_string())
    }
}

impl From<Gender> for String {
    fn from(gender: Gender) -> Self {
        gender.to_string()
    }
}

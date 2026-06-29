use std::{fmt, str::FromStr};

use thiserror::Error;

pub const RUSSIAN: &str = "ru";
pub const ENGLISH: &str = "en";

#[derive(Debug, Clone, Copy, Error)]
#[error("unknown language; expected either `{RUSSIAN}` or `{ENGLISH}`")]
pub struct Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Language {
    #[default]
    Russian,
    English,
}

impl fmt::Display for Language {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse_str(string)
    }
}

impl Language {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Russian => RUSSIAN,
            Self::English => ENGLISH,
        }
    }

    pub fn parse_str(string: &str) -> Result<Self, Error> {
        match string {
            RUSSIAN => Ok(Self::Russian),
            ENGLISH => Ok(Self::English),
            _ => Err(Error),
        }
    }
}

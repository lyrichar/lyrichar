use std::{fmt, str::FromStr};

use thiserror::Error;

use unic_langid::{LanguageIdentifier, langid};

pub type StaticStr = &'static str;

pub trait LanguageString {
    fn string(&self) -> StaticStr;
}

impl LanguageString for Language {
    fn string(&self) -> StaticStr {
        self.as_str()
    }
}

impl<T: LanguageString> LanguageString for Option<T> {
    fn string(&self) -> StaticStr {
        match self {
            Some(language) => language.string(),
            None => UNKNOWN,
        }
    }
}

pub trait LanguageCycle {
    fn cycle(&self) -> Language;
}

impl LanguageCycle for Language {
    fn cycle(&self) -> Language {
        match self {
            Self::English => Self::Russian,
            Self::Russian => Self::English,
        }
    }
}

impl<T: LanguageCycle> LanguageCycle for Option<T> {
    fn cycle(&self) -> Language {
        match self {
            Some(language) => language.cycle(),
            None => Language::DEFAULT,
        }
    }
}

pub const ENGLISH: &str = "en";
pub const RUSSIAN: &str = "ru";

pub const ENGLISH_ID: LanguageIdentifier = langid!("en-US");
pub const RUSSIAN_ID: LanguageIdentifier = langid!("ru-RU");

pub const UNKNOWN: &str = "unknown";

#[derive(Debug, Clone, Copy, Error)]
#[error("unknown language; expected either `{ENGLISH}` or `{RUSSIAN}`")]
pub struct Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    English,
    Russian,
}

impl Default for Language {
    fn default() -> Self {
        Self::DEFAULT
    }
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

impl TryFrom<LanguageIdentifier> for Language {
    type Error = Error;

    fn try_from(id: LanguageIdentifier) -> Result<Self, Self::Error> {
        Self::match_id(id)
    }
}

impl Language {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Russian => RUSSIAN,
            Self::English => ENGLISH,
        }
    }

    pub const fn id(&self) -> LanguageIdentifier {
        match self {
            Self::Russian => RUSSIAN_ID,
            Self::English => ENGLISH_ID,
        }
    }

    pub fn match_id(id: LanguageIdentifier) -> Result<Self, Error> {
        match id {
            RUSSIAN_ID => Ok(Self::Russian),
            ENGLISH_ID => Ok(Self::English),
            _ => Err(Error),
        }
    }

    pub fn parse_str(string: &str) -> Result<Self, Error> {
        match string {
            RUSSIAN => Ok(Self::Russian),
            ENGLISH => Ok(Self::English),
            _ => Err(Error),
        }
    }

    pub const DEFAULT: Self = Self::English;
}

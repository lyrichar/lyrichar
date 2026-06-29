use dioxus_i18n::prelude::*;

use unic_langid::{LanguageIdentifier, langid};

pub const ENGLISH_ID: LanguageIdentifier = langid!("en-US");
pub const RUSSIAN_ID: LanguageIdentifier = langid!("ru-RU");

pub const ENGLISH: &str = include_str!("translations/en-US.ftl");
pub const RUSSIAN: &str = include_str!("translations/ru-RU.ftl");

pub fn i18n_config() -> I18nConfig {
    let english = Locale::new_static(ENGLISH_ID, ENGLISH);
    let russian = Locale::new_static(RUSSIAN_ID, RUSSIAN);

    I18nConfig::new(ENGLISH_ID)
        .with_locale(english)
        .with_locale(russian)
        .with_fallback(ENGLISH_ID)
}

use dioxus_i18n::prelude::*;

use lyrichar_core::languages::Language;

pub const DEFAULT: Language = Language::DEFAULT;

pub const ENGLISH: Language = Language::English;
pub const RUSSIAN: Language = Language::Russian;

pub const ENGLISH_STRING: &str = include_str!("translations/en-US.ftl");
pub const RUSSIAN_STRING: &str = include_str!("translations/ru-RU.ftl");

pub fn i18n_config() -> I18nConfig {
    let english = Locale::new_static(ENGLISH.id(), ENGLISH_STRING);
    let russian = Locale::new_static(RUSSIAN.id(), RUSSIAN_STRING);

    I18nConfig::new(DEFAULT.id())
        .with_locale(english)
        .with_locale(russian)
        .with_fallback(DEFAULT.id())
}

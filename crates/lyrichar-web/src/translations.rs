use dioxus::signals::Signal;
use dioxus_i18n::prelude::*;
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};

use lyrichar_core::languages::Language;

pub const LANGUAGE: &str = "language";

pub const ENGLISH: Language = Language::English;
pub const RUSSIAN: Language = Language::Russian;

pub const ENGLISH_STRING: &str = include_str!("translations/en-US.ftl");
pub const RUSSIAN_STRING: &str = include_str!("translations/ru-RU.ftl");

pub fn use_language() -> Signal<Language> {
    use_synced_storage::<LocalStorage, Language>(LANGUAGE.to_owned(), Language::default)
}

pub fn i18n_config(initial: Language) -> I18nConfig {
    let english = Locale::new_static(ENGLISH.id(), ENGLISH_STRING);
    let russian = Locale::new_static(RUSSIAN.id(), RUSSIAN_STRING);

    I18nConfig::new(initial.id())
        .with_locale(english)
        .with_locale(russian)
        .with_fallback(initial.id())
}

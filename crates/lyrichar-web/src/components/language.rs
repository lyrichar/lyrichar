use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

use lyrichar_core::languages::{LanguageCycle, LanguageString};

use crate::translations::use_language;

#[component]
pub fn LanguageButton() -> Element {
    let mut i18n = i18n();

    let mut language = use_language();

    let cycle = move |_| {
        let cycled = language.read().cycle();

        language.set(cycled);
    };

    use_effect(move || {
        i18n.set_language(language.read().id());
    });

    rsx! {
        button {
            r#type: "button",

            aria_label: "Change language",

            class: "
                fixed bottom-4 right-4
                h-10 px-4 rounded-lg
                bg-linear-to-b from-lyrics-magenta to-lyrics-yellow
                z-50
                select-none hover:scale-110 transition-transform
            ",

            onclick: cycle,

            span { class: "text-xl text-neutral-50 dark:text-neutral-900",
                {language.read().string()}
            }
        }
    }
}

use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, tid};

use lyrichar_core::languages::{LanguageCycle, LanguageString};

use crate::{chrono, components::head::Head, routes::Route, translations::use_language, urls};

pub const NAME: Asset = asset!(
    "/assets/name.svg",
    ImageAssetOptions::new().with_hash_suffix(false)
);

#[component]
pub fn Home() -> Element {
    let year = chrono::year();
    let url = urls::root();

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
        Head {
            title: tid!("home"),
            description: tid!("home.description"),
            url,
        }

        nav { aria_label: "Navigation", class: "absolute flex w-full",
            div { class: "
                    mx-auto
                    max-w-md sm:max-w-3xl lg:max-w-7xl
                    px-4 sm:px-6 lg:px-8 py-4
                    flex items-center
                    w-full
                ",

                Link { to: Route::Home {}, class: "mr-auto",
                    img { class: "w-auto h-16", src: NAME, alt: "Lyrichar" }
                }

                div { class: "relative ml-auto flex space-x-8",

                    div { class: "hidden md:flex space-x-8",

                    }

                    button {
                        r#type: "button",

                        class: "
                            flex items-center justify-center
                            bg-neutral-900 dark:bg-neutral-50
                            h-10 px-4
                            w-full sm:w-auto
                            rounded-lg
                        ",

                        onclick: cycle,

                        label { class: "
                                text-xl text-transparent
                                bg-clip-text bg-linear-to-b
                                from-lyrics-magenta to-lyrics-yellow
                            ",
                            {language.read().string()}
                        }
                    }

                    a {
                        href: "/open",
                        class: "
                            flex items-center justify-center
                            text-xl
                            bg-gradient-to-b from-lyrics-magenta to-lyrics-yellow
                            text-neutral-50 dark:text-neutral-900
                            h-10 px-4
                            w-full sm:w-auto
                            rounded-lg
                        ",
                        "Open"
                    }
                }
            }
        }

        div { class: "
                mx-auto
                max-w-md sm:max-w-3xl lg:max-w-7xl
                px-4 sm:px-6 lg:px-8
                flex flex-col lg:flex-row
                justify-between
                gap-5
                pt-16 sm:pt-20 lg:pt-24
            ",

            section { class: "my-12 w-full lg:w-1/2",
                h1 { class: "text-5xl lg:text-7xl",
                    {tid!("slogan")}
                    span { class: "
                            text-transparent
                            bg-clip-text bg-linear-to-b
                            from-lyrics-magenta to-lyrics-yellow
                            animate-blink
                        ",
                        "▏"
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    chrono,
    components::{head::Head, language::LanguageButton},
    routes::Route,
    urls,
};

pub const NAME: Asset = asset!(
    "/assets/images/name.svg",
    ImageAssetOptions::new().with_hash_suffix(false)
);

pub const SPACE: &str = "\u{A0}";

#[component]
pub fn Home() -> Element {
    let year = chrono::year();
    let age = chrono::age();

    let url = urls::root();

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

                    div { class: "hidden md:flex space-x-8" }

                    a {
                        href: "/open",
                        class: "
                            flex items-center justify-center
                            text-xl
                            bg-linear-to-b from-lyrics-magenta to-lyrics-yellow
                            text-neutral-50 dark:text-neutral-900
                            h-10 px-4
                            w-full sm:w-auto
                            hover:scale-110 transition-transform
                            rounded-lg
                        ",
                        {tid!("open")}
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

                    span { class: "cursor", {SPACE} }
                }
            }
        }

        LanguageButton {}
    }
}

use dioxus::prelude::*;

use crate::{chrono, components::head::Head, routes::Route, urls};

pub const TITLE: &str = "Home";
pub const DESCRIPTION: &str = "Lyrics at the speed of thought.";

pub const NAME: Asset = asset!(
    "/assets/name.svg",
    ImageAssetOptions::new().with_hash_suffix(false)
);

#[component]
pub fn Home() -> Element {
    let year = chrono::year();
    let url = urls::root();

    rsx! {
        Head {
            title: TITLE,
            description: DESCRIPTION,
            url: url,
        }

        nav {
            aria_label: "Navigation",
            class: "absolute flex w-full",
            div {
                class: "
                    mx-auto
                    max-w-md sm:max-w-3xl lg:max-w-7xl
                    px-4 sm:px-6 lg:px-8 py-4
                    flex items-center
                    w-full
                ",
                Link {
                    to: Route::Home {},
                    class: "mr-auto",
                    img {
                        class: "w-auto h-16",
                        src: NAME,
                        alt: "Lyrichar",
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
                    "Open",
                }
            }
        }

        div {
            class: "
                mx-auto
                max-w-md sm:max-w-3xl lg:max-w-7xl
                px-4 sm:px-6 lg:px-8
                flex flex-col lg:flex-row
                justify-between
                gap-5
                pt-16 sm:pt-20 lg:pt-24
            ",

            section {
                class: "my-12 w-full lg:w-1/2",
                h1 {
                    class: "text-5xl lg:text-7xl",
                    span { class: "hover-linear-text", "Lyrics" }
                    " "
                    span { class: "hover-linear-text", "at" }
                    " "
                    span { class: "hover-linear-text", "the" }
                    " "
                    span { class: "hover-linear-text", "speed" }
                    " "
                    span { class: "hover-linear-text", "of" }
                    " "
                    span { class: "hover-linear-text", "thought" }
                    span { class: "hover-linear-text", "." }
                    span { class: "hover-linear-text animate-blink", "▏" }
                }
            }
        }
    }
}

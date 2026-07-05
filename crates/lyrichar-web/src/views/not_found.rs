use dioxus::{fullstack::*, prelude::*};
use dioxus_i18n::tid;

use crate::{components::head::Head, urls};

pub const SLASH: &str = "/";

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route = route.join(SLASH);
    let url = urls::route(route.as_str());

    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);

    rsx! {
        Head {
            title: tid!("not-found"),
            description: tid!("not-found.description"),
            url,
        }

        div { class: "
                mx-auto
                max-w-md sm:max-w-3xl lg:max-w-5xl
                px-4 sm:px-6 lg:px-8
                flex flex-col lg:flex-row
                justify-between
                gap-5
                pt-16 sm:pt-20 lg:pt-24
            ",
            section {
                h1 { class: "text-5xl lg:text-7xl",
                    span { class: "
                        text-transparent
                        bg-clip-text bg-linear-to-b
                        from-lyrics-magenta to-lyrics-yellow
                    ",
                        {tid!("not-found")}
                    }
                }

                p { class: "text-xl",

                    {tid!("not-found.route")}

                    " "

                    span { class: "wrap-break-word text-transparent bg-clip-text bg-linear-to-r from-lyrics-magenta to-lyrics-yellow",
                        "{SLASH}{route}"
                    }

                    " "

                    {tid!("not-found.detail")}
                }
            }
        }
    }
}

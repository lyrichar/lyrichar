use dioxus::{fullstack::*, prelude::*};

use crate::{components::head::Head, urls};

pub const NOT_FOUND: &str = "Not Found";
pub const DESCRIPTION: &str = "This route was not found.";
pub const SLASH: &str = "/";

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route = route.join(SLASH);
    let url = urls::route(route.as_str());

    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);

    rsx! {
        Head {
            title: NOT_FOUND,
            description: DESCRIPTION,
            url: url,
        }

        div {
            section {
                h1 {
                    class: "text-5xl lg:text-7xl",
                    span {
                        class: "text-transparent bg-clip-text bg-linear-to-b from-lyrics-magenta to-lyrics-yellow",
                        { NOT_FOUND }
                    }
                }

                p {
                    class: "text-xl",
                    "Route "

                    span {
                        class: "wrap-break-word text-transparent bg-clip-text bg-linear-to-r from-lyrics-magenta to-lyrics-yellow",
                        "{SLASH}{route}"
                    }

                    " was not found."
                }
            }
        }
    }
}

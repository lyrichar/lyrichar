use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

use crate::{
    translations::i18n_config,
    views::{home::Home, not_found::NotFound},
};

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}

#[component]
pub fn App() -> Element {
    use_init_i18n(i18n_config);

    rsx! {
        main {
            Router::<Route> {}
        }
    }
}

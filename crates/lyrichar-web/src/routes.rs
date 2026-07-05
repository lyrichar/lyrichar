use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

use crate::{
    translations::{i18n_config, use_language},
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
    let language = use_language();

    let initial = language.read().cloned();

    use_init_i18n(move || i18n_config(initial));

    rsx! {
        main { Router::<Route> {} }
    }
}

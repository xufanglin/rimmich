use crate::components::{Home, Settings};
use crate::core::load_config;
use dioxus::prelude::*;
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum AppRoute {
    #[route("/")]
    Home {},
    #[route("/settings")]
    Settings {},
}

#[component]
pub fn App() -> Element {
    let config = load_config().unwrap_or_default();

    use_context_provider(|| Signal::new(config));

    rsx! {
        Router::<AppRoute> {}
    }
}

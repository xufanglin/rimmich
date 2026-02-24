mod concurrency_config;
mod icons;
mod language_config;
mod server_config;
mod user_management;

use concurrency_config::*;
use icons::*;
use language_config::*;
use server_config::*;
use user_management::*;

use crate::components::{AppRoute, get_i18n};
use crate::core::AppConfig;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let nav = use_navigator();
    let config = use_context::<Signal<AppConfig>>();
    let i18n = get_i18n(&config);
    let status = use_signal(|| {
        let i18n = get_i18n(&config);
        i18n.manage_your_settings().to_string()
    });

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/settings.css") }

        div { class: "header-row",
            button {
                onclick: move |_| { nav.push(AppRoute::Home {}); },
                HomeIcon {}
            }
            h2 { "{i18n.settings_header()}" }
        }

        ServerConfig { config, status }
        ConcurrencyConfig { config, status }
        LanguageConfig { config, status }
        UserManagement { config, status }

        div { class: "status-text", "{status}" }
    }
}

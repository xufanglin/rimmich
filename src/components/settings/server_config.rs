use crate::components::get_i18n;
use crate::core::{AppConfig, save_config};
use dioxus::prelude::*;

#[component]
pub fn ServerConfig(config: Signal<AppConfig>, mut status: Signal<String>) -> Element {
    let i18n = get_i18n(&config);
    let mut server_url = use_signal(|| config.read().server_url.clone());

    rsx! {
        div { class: "server-group",
            label { "{i18n.server_url()}" }
            div { class: "server-input-group",
                input {
                    value: "{server_url}",
                    oninput: move |evt| server_url.set(evt.value()),
                    placeholder: "http://your-immich-server:2283",
                }

                button {
                    onclick: move |_| {
                        let mut temp_config = config.read().clone();
                        temp_config.server_url = server_url.read().clone();
                        let i18n = get_i18n(&config);

                        if let Err(e) = save_config(&temp_config) {
                            tracing::error!("Failed to save server URL config: {}", e);
                            status.set(i18n.save_failed(&e.to_string()));
                        } else {
                            tracing::info!("Server URL updated: {}", temp_config.server_url);
                            config.set(temp_config);
                            status.set(i18n.server_url_saved().to_string());
                        }
                    },
                    "{i18n.save()}"
                }
            }
        }
    }
}

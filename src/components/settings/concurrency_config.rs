use crate::components::get_i18n;
use crate::core::{AppConfig, save_config};
use dioxus::prelude::*;

#[component]
pub fn ConcurrencyConfig(config: Signal<AppConfig>, mut status: Signal<String>) -> Element {
    let i18n = get_i18n(&config);
    let mut concurrency = use_signal(|| config.read().concurrency);

    rsx! {
        div { class: "concurrency-group",
            label { "{i18n.concurrency()}" }
            div { class: "concurrency-input-group",
                input {
                    r#type: "number",
                    min: "1",
                    max: "16",
                    value: "{concurrency}",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<u8>() {
                            concurrency.set(val);
                        }
                    },
                }

                button {
                    onclick: move |_| {
                        let mut temp_config = config.read().clone();
                        temp_config.concurrency = *concurrency.read();
                        let i18n = get_i18n(&config);

                        if temp_config.concurrency < 1 || temp_config.concurrency > 16 {
                            tracing::warn!("Invalid concurrency setting: {}", temp_config.concurrency);
                            status.set(i18n.invalid_concurrency().to_string());
                        } else if let Err(e) = save_config(&temp_config) {
                            tracing::error!("Failed to save concurrency config: {}", e);
                            status.set(i18n.save_failed(&e.to_string()));
                        } else {
                            tracing::info!("Concurrency updated: {}", temp_config.concurrency);
                            config.set(temp_config);
                            status.set(i18n.concurrency_saved().to_string());
                        }
                    },
                    "{i18n.save()}"
                }
            }
        }
    }
}

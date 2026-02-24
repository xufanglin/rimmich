use crate::components::get_i18n;
use crate::core::{AppConfig, Language, save_config};
use dioxus::prelude::*;

#[component]
pub fn LanguageConfig(config: Signal<AppConfig>, mut status: Signal<String>) -> Element {
    let i18n = get_i18n(&config);
    let mut selected_language = use_signal(|| config.read().language.clone());

    rsx! {
        div { class: "language-group",
            label { "{i18n.language()}" }
            div { class: "language-input-group",
                select {
                    value: "{selected_language.read().code()}",
                    oninput: move |evt| {
                        match evt.value().as_str() {
                            "en" => selected_language.set(Language::English),
                            _ => selected_language.set(Language::Chinese),
                        }
                    },
                    option { value: "zh", "中文" }
                    option { value: "en", "English" }
                }

                button {
                    onclick: move |_| {
                        let mut temp_config = config.read().clone();
                        temp_config.language = selected_language.read().clone();
                        let i18n = get_i18n(&config);

                        if let Err(e) = save_config(&temp_config) {
                            tracing::error!("Failed to save language config: {}", e);
                            status.set(i18n.save_failed(&e.to_string()));
                        } else {
                            tracing::info!("Language updated: {:?}", temp_config.language);
                            config.set(temp_config);
                            status.set(i18n.language_saved().to_string());
                        }
                    },
                    "{i18n.save()}"
                }
            }
        }
    }
}

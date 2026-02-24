mod file_list;
mod icons;
mod upload;

use file_list::*;
use icons::*;
use upload::*;

use crate::components::{AppRoute, get_i18n};
use crate::core::AppConfig;
use dioxus::prelude::*;
use rfd::FileHandle;

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    let config = use_context::<Signal<AppConfig>>();
    let mut selected_user = use_signal(|| config.read().current_user.clone());
    let server_url = config.read().server_url.clone();
    let mut status = use_signal(|| {
        let i18n = get_i18n(&config);
        i18n.ready().to_string()
    });
    let selected_files = use_signal(|| Vec::<FileHandle>::new());
    let is_uploading = use_signal(|| false);

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/home.css") }

        div { class: "header-row",
            h2 {
                {
                    let i18n = get_i18n(&config);
                    i18n.home_header()
                }
            }
            button {
                onclick: move |_| { nav.push(AppRoute::Settings {}); },
                SettingsIcon {}
            }
        }

        UserSelector {
            selected_user,
            config,
            on_change: move |username: String| {
                let i18n = get_i18n(&config);
                selected_user.set(username.clone());
                status.set(i18n.switched_to_user(&username));
            }
        }

        div { class: "server-info",
            span {
                {
                    let i18n = get_i18n(&config);
                    i18n.server_url()
                }
            }
            p { "{server_url}" }
        }

        FileList { selected_files }

        UploadButtons {
            config,
            selected_user,
            selected_files,
            is_uploading,
            status
        }

        div { class: "status-text", "{status}" }
    }
}

#[component]
fn UserSelector(
    selected_user: Signal<String>,
    config: Signal<AppConfig>,
    on_change: EventHandler<String>,
) -> Element {
    let i18n = get_i18n(&config);

    rsx! {
        div { class: "user-group",
            label { "{i18n.select_upload_account()}" }
            select {
                value: selected_user.read().clone(),
                onchange: move |evt| on_change.call(evt.value()),

                if config.read().users.is_empty() {
                    option { value: "", "{i18n.add_account_in_settings()}" }
                } else {
                    {
                        let mut users: Vec<_> = config.read().users.keys().cloned().collect();
                        users.sort();
                        rsx! {
                            for username in users {
                                option {
                                    value: "{username}",
                                    selected: *selected_user.read() == *username,
                                    "{username}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

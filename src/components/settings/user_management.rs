use super::icons::{DeleteIcon, LockIcon, StarIcon};
use crate::components::get_i18n;
use crate::core::{AppConfig, UserConfig, save_config};
use dioxus::prelude::*;

#[component]
pub fn UserManagement(config: Signal<AppConfig>, mut status: Signal<String>) -> Element {
    let i18n = get_i18n(&config);
    let mut new_username = use_signal(|| String::new());
    let mut new_api_key = use_signal(|| String::new());

    rsx! {
        div { class: "user-group",
            label { "{i18n.user_management()}" }
            div { class: "user-input-group",
                div { class: "adduser-row",
                    input {
                        class: "user-input",
                        placeholder: "{i18n.username()}",
                        value: "{new_username}",
                        oninput: move |evt| new_username.set(evt.value()),
                    }

                    input {
                        class: "apikey-input",
                        placeholder: "API Key",
                        value: "{new_api_key}",
                        oninput: move |evt| new_api_key.set(evt.value()),
                    }

                    button {
                        onclick: move |_| {
                            let name = new_username.read().trim().to_string();
                            let key = new_api_key.read().trim().to_string();
                            let i18n = get_i18n(&config);
                            let mut temp_config = config.read().clone();

                            if name.is_empty() || key.is_empty() {
                                status.set(i18n.please_fill_complete_info().to_string());
                                return;
                            }

                            temp_config.users.insert(name.clone(), UserConfig { api_key: key.clone() });
                            if temp_config.current_user.is_empty() {
                                temp_config.current_user = name.clone();
                            }

                            if let Err(e) = save_config(&temp_config) {
                                tracing::error!("Failed to save user config: {}", e);
                                status.set(i18n.save_failed(&e.to_string()));
                            } else {
                                tracing::info!("New user added: {}", name);
                                config.set(temp_config);
                                new_username.set(String::new());
                                new_api_key.set(String::new());
                                status.set(i18n.user_added().to_string());
                            }
                        },
                        "{i18n.add()}"
                    }
                }
            }
            div { class: "divider" }

            UserList { config, status }
        }
    }
}

#[component]
fn UserList(config: Signal<AppConfig>, mut status: Signal<String>) -> Element {
    let i18n = get_i18n(&config);

    rsx! {
        div { class: "user-management-group",
            if config.read().users.is_empty() {
                div { "{i18n.no_users()}" }
            } else {
                for (username, _) in config.read().users.clone() {
                    UserCard { username, config, status }
                }
            }
        }
    }
}

#[component]
fn UserCard(username: String, config: Signal<AppConfig>, mut status: Signal<String>) -> Element {
    let i18n = get_i18n(&config);
    let is_current = username == config.read().current_user;

    rsx! {
        div { class: "user-card",
            div { class: "user-name",
                if is_current { StarIcon {} }
                "{username}"
            }
            div { class: "user-actions",
                button {
                    class: "action-btn default-btn",
                    title: "{i18n.set_as_default()}",
                    onclick: {
                        let temp_username = username.clone();
                        move |_| {
                            let i18n = get_i18n(&config);
                            let mut temp_config = config.read().clone();
                            temp_config.current_user = temp_username.clone();

                            if let Err(e) = save_config(&temp_config) {
                                tracing::error!("Failed to set default user: {}", e);
                                status.set(i18n.save_failed(&e.to_string()));
                            } else {
                                tracing::info!("Default user changed to: {}", temp_username);
                                config.set(temp_config);
                                status.set(i18n.default_user_changed(&temp_username));
                            }
                        }
                    },
                    LockIcon {}
                    span { "{i18n.set_as_default()}" }
                }

                button {
                    class: "action-btn delete-btn",
                    title: "{i18n.delete()}",
                    onclick: {
                        let temp_username = username.clone();
                        move |_| {
                            let i18n = get_i18n(&config);
                            let mut temp_config = config.read().clone();
                            temp_config.users.remove(&temp_username);

                            if temp_config.current_user == temp_username {
                                temp_config.current_user = temp_config
                                    .users
                                    .keys()
                                    .next()
                                    .cloned()
                                    .unwrap_or_default();
                            }

                            if let Err(e) = save_config(&temp_config) {
                                tracing::error!("Failed to delete user: {}", e);
                                status.set(i18n.save_failed(&e.to_string()));
                            } else {
                                tracing::info!("User deleted: {}", temp_username);
                                config.set(temp_config);
                                status.set(i18n.user_deleted(&temp_username));
                            }
                        }
                    },
                    DeleteIcon {}
                    span { "{i18n.delete()}" }
                }
            }
        }
    }
}

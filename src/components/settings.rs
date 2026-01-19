use crate::components::*;
use crate::services::*;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::{FiArrowLeftCircle, FiLock, FiStar, FiTrash2};

/// 设置页面组件
#[component]
pub fn Settings() -> Element {
    let nav = use_navigator();
    let mut config = use_context::<Signal<AppConfig>>();
    let mut selected_language = use_signal(|| config.read().language.clone());
    let i18n = get_i18n(&config);
    let mut server_url = use_signal(|| config.read().server_url.clone());
    let mut concurrency = use_signal(|| config.read().concurrency.clone());
    let mut new_username = use_signal(|| String::new());
    let mut new_api_key = use_signal(|| String::new());

    let mut status = use_signal(|| i18n.manage_your_settings().to_string());

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/settings.css") }
        div { class: "header-row",
            button {
                onclick: move |_| {
                    nav.push(AppRoute::Home {});
                },
                HomeIcon {}
            }
            h2 { "{i18n.settings_header()}" }
        }

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
                        // 保存服务器地址
                        let mut temp_config = config.read().clone();
                        temp_config.server_url = server_url.read().clone();

                        let i18n = get_i18n(&config);

                        if let Err(e) = config::save_config(&temp_config) {
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
                        // 保存并发数配置
                        let mut temp_config = config.read().clone();
                        temp_config.concurrency = concurrency.read().clone();
                        let i18n = get_i18n(&config);

                        if temp_config.concurrency < 1 || temp_config.concurrency > 16 {
                            tracing::warn!("Invalid concurrency setting: {}", temp_config.concurrency);
                            status.set(i18n.invalid_concurrency().to_string());
                        } else if let Err(e) = config::save_config(&temp_config) {
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
                        // 保存语言设置
                        let mut temp_config = config.read().clone();
                        temp_config.language = selected_language.read().clone();
                        let i18n = get_i18n(&config);
                        if let Err(e) = config::save_config(&temp_config) {
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
                            // 添加新用户
                            let name = new_username.read().trim().to_string();
                            let key = new_api_key.read().trim().to_string();
                            let i18n = get_i18n(&config);
                            let mut temp_config = config.read().clone();

                            if name.is_empty() || key.is_empty() {
                                status.set(i18n.please_fill_complete_info().to_string());
                                return;
                            }

                            temp_config.users.insert(name.clone(), UserConfig { api_key: key.clone() });
                            // 如果是第一个用户，设为默认用户
                            if temp_config.current_user.is_empty() {
                                temp_config.current_user = name.clone();
                            }

                            if let Err(e) = config::save_config(&temp_config) {
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

            div { class: "user-management-group",
                if config.read().users.is_empty() {
                    div { "{i18n.no_users()}" }
                } else {
                    for (username , _) in config.read().users.clone() {
                        div { class: "user-card",
                            div { class: "user-name",
                                if username == config.read().current_user {
                                    StarIcon {}
                                }
                                "{username}"
                            }
                            div { class: "user-actions",
                                button {
                                    class: "action-btn default-btn",
                                    title: "{i18n.set_as_default()}",
                                    onclick: {
                                        let temp_username = username.clone();
                                        move |_| {
                                            // 设置为默认用户
                                            let i18n = get_i18n(&config);
                                            let mut temp_config = config.read().clone();
                                            temp_config.current_user = temp_username.clone();

                                            if let Err(e) = config::save_config(&temp_config) {
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
                                            // 删除用户
                                            let i18n = get_i18n(&config);
                                            let mut temp_config = config.read().clone();
                                            temp_config.users.remove(&temp_username);
                                            // 如果删除的是当前用户，切换到下一个用户
                                            if temp_config.current_user == temp_username {
                                                temp_config.current_user = temp_config
                                                    .users
                                                    .keys()
                                                    .next()
                                                    .cloned()
                                                    .unwrap_or_default();
                                            }
                                            if let Err(e) = config::save_config(&temp_config) {
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
            }
        }

        div { class: "status-text", "{status}" }
    }
}

#[component]
fn HomeIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiArrowLeftCircle
    })
}

#[component]
fn StarIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiStar
    })
}

#[component]
fn LockIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiLock
    })
}
#[component]
fn DeleteIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiTrash2
    })
}

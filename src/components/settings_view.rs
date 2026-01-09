use crate::components::icons::{IconAdd, IconBack, IconDelete, IconPin, IconStar};
use crate::i18n::{I18n, Language};
use crate::services::settings;
use dioxus::prelude::*;

#[component]
pub fn SettingsView(
    settings: Signal<Option<crate::models::AppSettings>>,
    on_back: EventHandler<()>,
) -> Element {
    // 服务器 URL 状态
    let mut server_url = use_signal(|| {
        settings
            .read()
            .as_ref()
            .map(|s| s.server_url.clone())
            .unwrap_or_default()
    });

    // 并发上传数状态，默认值为 5
    let mut concurrency =
        use_signal(|| settings.read().as_ref().map(|s| s.concurrency).unwrap_or(5));

    // 新增用户的输入状态
    let mut new_username = use_signal(|| String::new());
    let mut new_api_key = use_signal(|| String::new());
    // 状态栏消息
    let mut status = use_signal(|| {
        let language = settings
            .read()
            .as_ref()
            .map(|s| s.language)
            .unwrap_or(Language::Chinese);
        let i18n = I18n::new(language);
        i18n.manage_your_settings().to_string()
    });

    // 获取当前语言设置
    let language = settings
        .read()
        .as_ref()
        .map(|s| s.language)
        .unwrap_or(Language::Chinese);
    let i18n = I18n::new(language);

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/main.css") }
        link { rel: "icon", href: asset!("/assets/icon.png") }
        div { class: "app-container",
            div { class: "header-row",
                button {
                    class: "icon-btn",
                    onclick: move |_| on_back.call(()),
                    title: "{i18n.back()}",
                    IconBack {}
                }
                h2 { class: "title-inline", "{i18n.settings()}" }
                div { class: "spacer" }
            }

            div { class: "settings-section",
                label { class: "label", "{i18n.server_url()}" }
                div { class: "input-group",
                    input {
                        class: "text-input",
                        value: "{server_url}",
                        oninput: move |evt| server_url.set(evt.value()),
                        placeholder: "http://your-immich-server:2283",
                    }
                    button {
                        class: "btn-small",
                        onclick: move |_| {
                            let conf_opt = settings.read().clone();
                            if let Some(mut conf) = conf_opt {
                                conf.server_url = server_url.read().clone();
                                let language = conf.language;
                                let i18n = I18n::new(language);
                                if let Err(e) = settings::save_settings(&conf) {
                                    status.set(i18n.save_failed(&e.to_string()));
                                } else {
                                    settings.set(Some(conf));
                                    status.set(i18n.server_url_saved().to_string());
                                }
                            }
                        },
                        "{i18n.save()}"
                    }
                }
            }

            div { class: "settings-section",
                label { class: "label", "{i18n.concurrency()}" }
                div { class: "input-group",
                    input {
                        class: "text-input",
                        r#type: "number",
                        min: "1",
                        max: "20",
                        value: "{concurrency}",
                        oninput: move |evt| {
                            if let Ok(val) = evt.value().parse::<u8>() {
                                concurrency.set(val);
                            }
                        },
                    }
                    button {
                        class: "btn-small",
                        onclick: move |_| {
                            let conf_opt = settings.read().clone();
                            if let Some(mut conf) = conf_opt {
                                conf.concurrency = *concurrency.read();
                                let language = conf.language;
                                let i18n = I18n::new(language);
                                if let Err(e) = settings::save_settings(&conf) {
                                    status.set(i18n.save_failed(&e.to_string()));
                                } else {
                                    settings.set(Some(conf));
                                    status.set(i18n.concurrency_saved().to_string());
                                }
                            }
                        },
                        "{i18n.save()}"
                    }
                }
            }

            div { class: "settings-section",
                label { class: "label", "{i18n.language()}" }
                div { class: "input-group",
                    select {
                        class: "select-input",
                        value: "{language.code()}",
                        onchange: move |evt| {
                            let new_language = match evt.value().as_str() {
                                "en" => Language::English,
                                _ => Language::Chinese,
                            };
                            let conf_opt = settings.read().clone();
                            if let Some(mut conf) = conf_opt {
                                conf.language = new_language;
                                let i18n = I18n::new(new_language);
                                if let Err(e) = settings::save_settings(&conf) {
                                    status.set(i18n.save_failed(&e.to_string()));
                                } else {
                                    settings.set(Some(conf));
                                    status.set(i18n.language_saved().to_string());
                                }
                            }
                        },
                        option { value: "zh", "中文" }
                        option { value: "en", "English" }
                    }
                }
            }

            div { class: "settings-section",
                label { class: "label", "{i18n.user_management()}" }
                div { class: "management-group",
                    // 添加新用户表单区域
                    div { class: "add-user-row",
                        input {
                            class: "text-input input-username",
                            placeholder: "{i18n.username()}",
                            value: "{new_username}",
                            oninput: move |evt| new_username.set(evt.value()),
                        }
                        input {
                            class: "text-input input-api-key",
                            placeholder: "API Key",
                            value: "{new_api_key}",
                            oninput: move |evt| new_api_key.set(evt.value()),
                        }
                        button {
                            class: "btn btn-primary btn-add-user",
                            onclick: move |_| {
                                let name = new_username.read().trim().to_string();
                                let key = new_api_key.read().trim().to_string();
                                let conf_opt = settings.read().clone();
                                if let Some(mut conf) = conf_opt {
                                    let language = conf.language;
                                    let i18n = I18n::new(language);
                                    if name.is_empty() || key.is_empty() {
                                        status.set(i18n.please_fill_complete_info().to_string());
                                        return;
                                    }
                                    conf.users
                                        .insert(
                                            name.clone(),
                                            crate::models::UserSetting {
                                                api_key: key,
                                            },
                                        );
                                    if conf.current_user.is_empty() {
                                        conf.current_user = name;
                                    }
                                    if let Err(e) = settings::save_settings(&conf) {
                                        status.set(i18n.save_failed(&e.to_string()));
                                    } else {
                                        settings.set(Some(conf));
                                        new_username.set(String::new());
                                        new_api_key.set(String::new());
                                        status.set(i18n.user_added().to_string());
                                    }
                                }
                            },
                            IconAdd {}
                            span { "{i18n.add()}" }
                        }
                    }

                    div { class: "divider" }

                    // 已有用户列表展示区域，支持设置默认和删除
                    div { class: "user-list-container",
                        if let Some(conf) = settings.read().as_ref() {
                            if conf.users.is_empty() {
                                div { class: "empty-state-small", "{i18n.no_users()}" }
                            } else {
                                ul { class: "user-list",
                                    for (username , _) in &conf.users {
                                        li { class: "user-item",
                                            span { class: "user-name",
                                                if *username == conf.current_user {
                                                    IconStar {}
                                                }
                                                "{username}"
                                            }
                                            div { class: "user-actions",
                                                button {
                                                    class: "btn-icon-small",
                                                    title: "{i18n.set_as_default()}",
                                                    onclick: {
                                                        let username = username.clone();
                                                        move |_| {
                                                            let conf_opt = settings.read().clone();
                                                            if let Some(mut conf) = conf_opt {
                                                                conf.current_user = username.clone();
                                                                let language = conf.language;
                                                                let i18n = I18n::new(language);
                                                                if let Err(e) = settings::save_settings(&conf) {
                                                                    status.set(i18n.save_failed(&e.to_string()));
                                                                } else {
                                                                    settings.set(Some(conf));
                                                                    status.set(i18n.default_user_changed(&username));
                                                                }
                                                            }
                                                        }
                                                    },
                                                    IconPin {}
                                                    span { "{i18n.set_as_default()}" }
                                                }
                                                button {
                                                    class: "btn-icon-small delete",
                                                    title: "{i18n.delete()}",
                                                    onclick: {
                                                        let username = username.clone();
                                                        move |_| {
                                                            let conf_opt = settings.read().clone();
                                                            if let Some(mut conf) = conf_opt {
                                                                conf.users.remove(&username);
                                                                if conf.current_user == username {
                                                                    conf.current_user = conf
                                                                        .users
                                                                        .keys()
                                                                        .next()
                                                                        .cloned()
                                                                        .unwrap_or_default();
                                                                }
                                                                let language = conf.language;
                                                                let i18n = I18n::new(language);
                                                                if let Err(e) = settings::save_settings(&conf) {
                                                                    status.set(i18n.save_failed(&e.to_string()));
                                                                } else {
                                                                    settings.set(Some(conf));
                                                                    status.set(i18n.user_deleted(&username));
                                                                }
                                                            }
                                                        }
                                                    },
                                                    IconDelete {}
                                                    span { "{i18n.delete()}" }
                                                }
                                            }
                                        }
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
}

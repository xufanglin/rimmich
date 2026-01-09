use crate::components::icons::IconSettings;
use crate::i18n::{I18n, Language};
use crate::services::immich_api;
use dioxus::prelude::*;

#[component]
pub fn Home(
    settings: Signal<Option<crate::models::AppSettings>>,
    on_open_settings: EventHandler<()>,
) -> Element {
    // 当前选中的上传用户
    let mut selected_user = use_signal(|| {
        settings
            .read()
            .as_ref()
            .map(|s| s.current_user.clone())
            .unwrap_or_default()
    });

    // 待上传的文件列表
    let mut selected_files = use_signal(|| Vec::<rfd::FileHandle>::new());
    // 界面底部显示的状态消息
    let mut status = use_signal(|| {
        let language = settings
            .read()
            .as_ref()
            .map(|s| s.language)
            .unwrap_or(Language::Chinese);
        let i18n = I18n::new(language);
        i18n.ready().to_string()
    });
    // 是否正在上传的标志位，用于禁用/启用按钮
    let mut is_uploading = use_signal(|| false);

    // 获取服务器 URL 用于展示
    let server_url = settings
        .read()
        .as_ref()
        .map(|c| c.server_url.clone())
        .unwrap_or_else(|| "unknown server".to_string());

    rsx!(
        // 渲染 CSS 样式
        link { rel: "stylesheet", href: asset!("/assets/main.css") }
        link { rel: "icon", href: asset!("/assets/icon.png") }
        div { class: "app-container",
            div { class: "header-row",
                div { class: "spacer" }
                {
                    let language = settings
                        .read()
                        .as_ref()
                        .map(|s| s.language)
                        .unwrap_or(Language::Chinese);
                    let i18n = I18n::new(language);
                    rsx! {
                        h2 { class: "title-inline", "{i18n.title()}" }
                        button {
                            class: "icon-btn",
                            onclick: move |_| on_open_settings.call(()),
                            title: "{i18n.settings()}",
                            IconSettings {}
                        }
                    }
                }
            }

            // 账号选择区域
            div { class: "form-group",
                {
                    let language = settings
                        .read()
                        .as_ref()
                        .map(|s| s.language)
                        .unwrap_or(Language::Chinese);
                    let i18n = I18n::new(language);
                    rsx! {
                        label { class: "label", "{i18n.select_upload_account()}" }
                        select {
                            class: "select-input",
                            value: "{selected_user}",
                            onchange: move |evt| {
                                let language = settings
                                    .read()
                                    .as_ref()
                                    .map(|s| s.language)
                                    .unwrap_or(Language::Chinese);
                                let i18n = I18n::new(language);
                                selected_user.set(evt.value());
                                status.set(i18n.switched_to_user(&evt.value()));
                            },
                            if let Some(conf) = settings.read().as_ref() {
                                if conf.users.is_empty() {
                                    option { value: "", "{i18n.add_account_in_settings()}" }
                                } else {
                                    {
                                        let mut users: Vec<_> = conf.users.keys().collect();
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
                            } else {
                                option { "{i18n.no_config_found()}" }
                            }
                        }
                    }
                }
            }

            // 服务器信息展示
            div { class: "info-box",
                span { class: "info-label", "Server URL" }
                p { class: "info-value", "{server_url}" }
            }

            // 操作按钮组
            div { class: "button-group",
                div { class: "button-row",
                    {
                        let language = settings
                            .read()
                            .as_ref()
                            .map(|s| s.language)
                            .unwrap_or(Language::Chinese);
                        let i18n = I18n::new(language);
                        rsx! {
                            // 选择图片按钮
                            button {
                                class: "btn",
                                disabled: "{is_uploading}",
                                onclick: move |_| async move {
                                    let language = settings
                                        .read()
                                        .as_ref()
                                        .map(|s| s.language)
                                        .unwrap_or(Language::Chinese);
                                    let i18n = I18n::new(language);
                                    // 调用原生文件选择对话框
                                    let files = rfd::AsyncFileDialog::new()
                                        .add_filter("Media", &["jpg", "jpeg", "png", "heic", "webp", "mp4", "mov"])
                                        .set_title(i18n.select_photos_or_videos())
                                        .pick_files()
                                        .await;

                                    if let Some(files) = files {
                                        selected_files.set(files);
                                        status.set(i18n.files_selected(selected_files.read().len()));
                                    }
                                },
                                "{i18n.select_files()}"
                            }

                            // 清除选择按钮
                            button {
                                class: "btn",
                                disabled: "{is_uploading} || {selected_files.read().is_empty()}",
                                onclick: move |_| {
                                    let language = settings
                                        .read()
                                        .as_ref()
                                        .map(|s| s.language)
                                        .unwrap_or(Language::Chinese);
                                    let i18n = I18n::new(language);
                                    selected_files.set(Vec::new());
                                    status.set(i18n.selection_cleared().to_string());
                                },
                                "{i18n.clear()}"
                            }

                            // 开始上传按钮
                            button {
                                class: "btn",
                                disabled: "{is_uploading} || {selected_files.read().is_empty()}",
                                onclick: move |_| async move {
                                    is_uploading.set(true);
                                    let files = selected_files.read().clone();
                                    let total = files.len();
                                    
                                    // 获取当前语言设置
                                    let language = settings
                                        .read()
                                        .as_ref()
                                        .map(|s| s.language)
                                        .unwrap_or(Language::Chinese);
                                    let i18n = I18n::new(language);

                                    // 从配置中提取上传所需的参数
                                    let (server_url, api_key, concurrency) = {
                                        let current_config = settings.read();
                                        if let Some(c) = current_config.as_ref() {
                                            let user_key = selected_user.read();
                                            if let Some(user_info) = c.users.get(&*user_key) {
                                                (
                                                    c.server_url.clone(),
                                                    user_info.api_key.clone(),
                                                    c.concurrency as usize,
                                                )
                                            } else {
                                                status.set(i18n.user_api_key_not_found().to_string());
                                                is_uploading.set(false);
                                                return;
                                            }
                                        } else {
                                            status.set(i18n.config_file_not_found().to_string());
                                            is_uploading.set(false);
                                            return;
                                        }
                                    };

                                    status.set(i18n.start_parallel_upload(total, concurrency));
                                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
                                    let finished_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
                                    let error_occurred = std::sync::Arc::new(
                                        std::sync::atomic::AtomicBool::new(false),
                                    );
                                    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
                                    for file_handle in files {
                                        let server_url = server_url.clone();
                                        let api_key = api_key.clone();
                                        let tx = tx.clone();
                                        let finished_count = finished_count.clone();
                                        let error_occurred = error_occurred.clone();
                                        let semaphore = semaphore.clone();
                                        let file_name = file_handle.file_name();
                                        let i18n_clone = i18n.clone();
                                        tokio::spawn(async move {
                                            if error_occurred.load(std::sync::atomic::Ordering::Relaxed) {
                                                return;
                                            }
                                            let _permit = match semaphore.acquire_owned().await {
                                                Ok(p) => p,
                                                Err(_) => return,
                                            };
                                            let path = file_handle.path().to_path_buf();
                                            match immich_api::upload_asset(&server_url, &api_key, path).await {
                                                Ok(_) => {
                                                    let current = finished_count
                                                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                                                    let _ = tx.send(i18n_clone.upload_success(current, total, &file_name));
                                                }
                                                Err(e) => {
                                                    error_occurred.store(true, std::sync::atomic::Ordering::Relaxed);
                                                    let _ = tx.send(i18n_clone.upload_failed(&file_name, &e.to_string()));
                                                }
                                            }
                                        });
                                    }
                                    drop(tx);
                                    while let Some(msg) = rx.recv().await {
                                        status.set(msg.clone());
                                    }
                                    if !error_occurred.load(std::sync::atomic::Ordering::Relaxed) {
                                        status.set(i18n.all_files_uploaded(total));
                                        selected_files.set(Vec::new());
                                    }
                                    is_uploading.set(false);
                                },
                                {
                                    if *is_uploading.read() {
                                        let language = settings
                                            .read()
                                            .as_ref()
                                            .map(|s| s.language)
                                            .unwrap_or(Language::Chinese);
                                        let i18n = I18n::new(language);
                                        rsx! { "{i18n.uploading()}" }
                                    } else {
                                        let language = settings
                                            .read()
                                            .as_ref()
                                            .map(|s| s.language)
                                            .unwrap_or(Language::Chinese);
                                        let i18n = I18n::new(language);
                                        rsx! { "{i18n.start_upload()}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 待上传文件列表展示
            div { class: "file-list-container",
                if selected_files.read().is_empty() {
                    {
                        let language = settings
                            .read()
                            .as_ref()
                            .map(|s| s.language)
                            .unwrap_or(Language::Chinese);
                        let i18n = I18n::new(language);
                        rsx! {
                            div { class: "empty-state", "{i18n.no_files_selected()}" }
                        }
                    }
                } else {
                    {
                        let files = selected_files.read();
                        let total = files.len();
                        let language = settings
                            .read()
                            .as_ref()
                            .map(|s| s.language)
                            .unwrap_or(Language::Chinese);
                        let i18n = I18n::new(language);
                        rsx! {
                            h4 { class: "file-list-header", "{i18n.files_to_upload(total)}" }
                            ul { class: "file-list",
                                for file in files.iter() {
                                    li { class: "file-item",
                                        span { class: "file-bullet", "•" }
                                        "{file.path().display()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 状态栏
            div { class: "status-text", "{status}" }
        }
    )
}
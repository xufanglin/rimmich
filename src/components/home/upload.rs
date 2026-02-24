use crate::components::get_i18n;
use crate::core::{AppConfig, upload_asst};
use dioxus::prelude::*;
use rfd::{AsyncFileDialog, FileHandle};

#[component]
pub fn UploadButtons(
    config: Signal<AppConfig>,
    selected_user: Signal<String>,
    selected_files: Signal<Vec<FileHandle>>,
    is_uploading: Signal<bool>,
    status: Signal<String>,
) -> Element {
    let i18n = get_i18n(&config);

    rsx! {
        div { class: "button-row",
            button {
                disabled: "{is_uploading}",
                onclick: move |_| async move {
                    let i18n = get_i18n(&config);
                    let files = AsyncFileDialog::new()
                        .add_filter("media", &["jpg", "jpeg", "png", "heic", "webp", "mp4", "mov"])
                        .set_title(i18n.select_photos_or_videos())
                        .pick_files()
                        .await;
                    if let Some(file_list) = files {
                        selected_files.set(file_list);
                        status.set(i18n.files_selected(selected_files.read().len()));
                    }
                },
                "{i18n.select_files()}"
            }

            button {
                disabled: "{is_uploading} || {selected_files.read().is_empty()}",
                onclick: move |_| {
                    let i18n = get_i18n(&config);
                    selected_files.set(Vec::new());
                    status.set(i18n.selection_cleared().to_string());
                },
                "{i18n.clear()}"
            }

            button {
                disabled: "{is_uploading} || {selected_files.read().is_empty()}",
                onclick: move |_| async move {
                    handle_upload(config, selected_user, selected_files, is_uploading, status).await;
                },
                {if *is_uploading.read() { i18n.uploading() } else { i18n.start_upload() }}
            }
        }
    }
}

async fn handle_upload(
    config: Signal<AppConfig>,
    selected_user: Signal<String>,
    mut selected_files: Signal<Vec<FileHandle>>,
    mut is_uploading: Signal<bool>,
    mut status: Signal<String>,
) {
    is_uploading.set(true);
    let file_list = selected_files.read().clone();
    let total_files = file_list.len();
    let i18n = get_i18n(&config);

    let (server_url, api_key, concurrency) = {
        let current_config = config.read();
        let user_key = selected_user.read();
        if let Some(user_info) = current_config.users.get(&*user_key) {
            (
                current_config.server_url.clone(),
                user_info.api_key.clone(),
                current_config.concurrency as usize,
            )
        } else {
            tracing::warn!("API Key not found for user: {}", user_key);
            status.set(i18n.user_api_key_not_found().to_string());
            is_uploading.set(false);
            return;
        }
    };

    tracing::info!(
        "Starting batch upload: {} files with concurrency {}",
        total_files,
        concurrency
    );
    status.set(i18n.start_parallel_upload(total_files));

    let mut tasks = tokio::task::JoinSet::new();
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));

    for file_handle in file_list {
        let permit = semaphore.clone().acquire_owned().await.unwrap_or_else(|e| {
            tracing::error!("Failed to acquire semaphore: {}", e);
            panic!("Failed to acquire semaphore: {}", e);
        });
        let server_url = server_url.clone();
        let api_key = api_key.clone();
        let file_name = file_handle.file_name();
        let path = file_handle.path().to_path_buf();

        tasks.spawn(async move {
            let _permit = permit;
            upload_asst(&server_url, &api_key, path)
                .await
                .map(|_| file_name.clone())
                .map_err(|e| (file_name, e))
        });
    }

    let mut finished = 0;
    let mut has_error = false;

    while let Some(result) = tasks.join_next().await {
        match result.unwrap_or_else(|e| {
            tracing::error!("Task execution failed: {}", e);
            Err((
                "unknown file".to_string(),
                anyhow::anyhow!("Task execution failed: {}", e),
            ))
        }) {
            Ok(file_name) => {
                finished += 1;
                tracing::debug!(
                    "File uploaded successfully ({}/{}): {}",
                    finished,
                    total_files,
                    file_name
                );
                status.set(i18n.upload_success(finished, total_files, &file_name));
            }
            Err((file_name, e)) => {
                tracing::error!("File upload failed: {} - {}", file_name, e);
                status.set(i18n.upload_failed(&file_name, &e.to_string()));
                tasks.abort_all();
                has_error = true;
                break;
            }
        }
    }

    if !has_error {
        tracing::info!("All files uploaded successfully: {} files", total_files);
        status.set(i18n.all_files_uploaded(total_files));
        selected_files.set(Vec::new());
    }

    is_uploading.set(false);
}

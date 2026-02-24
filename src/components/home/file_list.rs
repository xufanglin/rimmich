use crate::components::get_i18n;
use crate::core::*;
use dioxus::prelude::*;
use rfd::FileHandle;

#[component]
pub fn FileList(selected_files: Signal<Vec<FileHandle>>) -> Element {
    let config = use_context::<Signal<AppConfig>>();
    let i18n = get_i18n(&config);

    rsx! {
        div { class: "files-list-container",
            if selected_files.read().is_empty() {
                div { class: "no-files-selected", "{i18n.no_files_selected()}" }
            } else {
                {
                    let file_list = selected_files.read();
                    let total_files = file_list.len();
                    rsx! {
                        h4 { "{i18n.files_to_upload(total_files)}" }
                        ul {
                            for file in file_list.iter() {
                                li {
                                    span { "." }
                                    "{file.path().display()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

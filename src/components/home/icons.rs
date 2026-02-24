use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::FiSettings;

#[component]
pub fn SettingsIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiSettings
    })
}

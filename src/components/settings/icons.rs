use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::{FiArrowLeftCircle, FiLock, FiStar, FiTrash2};

#[component]
pub fn HomeIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiArrowLeftCircle
    })
}

#[component]
pub fn StarIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiStar
    })
}

#[component]
pub fn LockIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiLock
    })
}

#[component]
pub fn DeleteIcon() -> Element {
    rsx!(Icon {
        class: "icon",
        icon: FiTrash2
    })
}

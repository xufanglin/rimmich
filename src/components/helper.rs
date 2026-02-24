use crate::core::{AppConfig, I18n};
use dioxus::prelude::*;

pub fn get_i18n(config: &Signal<AppConfig>) -> I18n {
    let language = config.read().language;
    I18n::new(language)
}

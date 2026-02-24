mod config;
mod i18n;
mod immich;
mod log;

// 只导出常用的核心类型，避免命名空间污染
pub use config::{AppConfig, UserConfig, load_config, save_config};
pub use i18n::{I18n, Language};
pub use immich::upload_asst;
pub use log::init_logger;

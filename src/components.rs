// 模块声明
mod helper;
mod home;
mod router;
mod settings;

// 公共导出
pub use helper::get_i18n;
pub use home::Home;
pub use router::{App, AppRoute};
pub use settings::Settings;

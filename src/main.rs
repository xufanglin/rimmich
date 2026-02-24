use dioxus::{LaunchBuilder, desktop};
use rimmich::components::App;
use rimmich::core::{AppConfig, init_logger, load_config};

fn main() {
    // 加载配置文件，失败时使用默认配置
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config, using default: {}", e);
        AppConfig::default()
    });

    // 初始化日志系统
    init_logger(&config.log_level).unwrap_or_else(|e| {
        eprintln!("Failed to initialize logger: {}", e);
    });

    // 启动桌面应用
    LaunchBuilder::desktop()
        .with_cfg(
            desktop::Config::new().with_window(
                desktop::WindowBuilder::new()
                    .with_title("rImmich")
                    .with_inner_size(desktop::LogicalSize::new(600.0, 800.0))
                    .with_min_inner_size(desktop::LogicalSize::new(400.0, 600.0))
                    .with_resizable(false),
            ),
        )
        .launch(App);
}

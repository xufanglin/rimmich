use dioxus::prelude::*;
use rimmich::components::home::Home;
use rimmich::components::settings_view::SettingsView;
use rimmich::services::settings;

fn main() {
    // 加载并设置窗口图标
    let icon_data = include_bytes!("../assets/icon.png");
    let icon = image::load_from_memory(icon_data).ok().and_then(|img| {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        dioxus::desktop::tao::window::Icon::from_rgba(rgba.into_raw(), width, height).ok()
    });

    // 初始化桌面窗口配置
    let mut window = dioxus::desktop::WindowBuilder::new()
        .with_title("rImmich")
        .with_inner_size(dioxus::desktop::LogicalSize::new(600.0, 800.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(400.0, 600.0))
        .with_resizable(true);

    if let Some(icon) = icon {
        window = window.with_window_icon(Some(icon));
    }

    let config = dioxus::desktop::Config::new().with_window(window);

    // 启动 Dioxus 桌面程序
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app);
}

/// 路由导航枚举
#[derive(Clone, Copy, PartialEq)]
enum Route {
    /// 主页界面
    Home,
    /// 设置界面
    Settings,
}

/// 应用程序主组件
fn app() -> Element {
    // 使用 Signal 响应式状态管理设置信息
    // 启动时尝试加载配置文件
    let settings_signal = use_signal(|| settings::load_settings().ok());

    // 路由状态管理，默认为主页
    let mut current_route = use_signal(|| Route::Home);

    // 根据当前路由渲染对应的页面组件
    match current_route() {
        Route::Home => rsx! {
            Home {
                settings: settings_signal,
                // 点击设置按钮时切换到设置页
                on_open_settings: move |_| current_route.set(Route::Settings),
            }
        },
        Route::Settings => rsx! {
            SettingsView {
                settings: settings_signal,
                // 点击返回按钮时切换回主页
                on_back: move |_| current_route.set(Route::Home),
            }
        },
    }
}

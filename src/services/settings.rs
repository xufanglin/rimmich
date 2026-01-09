use crate::models::AppSettings;
use std::fs;

/// 加载应用程序设置
///
/// 该函数会尝试从用户主目录下的 `.immich/config.toml` 文件读取配置。
/// 如果文件不存在，则会创建一个包含默认设置的新文件。
pub fn load_settings() -> Result<AppSettings, String> {
    // 获取用户主目录 (例如: /Users/username)
    let home = dirs::home_dir().ok_or("寻找$HOME目录失败")?;
    // 设置配置文件目录路径
    let settings_dir = home.join(".immich");
    // 设置配置文件完整路径
    let settings_path = settings_dir.join("config.toml");

    // 如果配置文件不存在，则进行初始化
    if !settings_path.exists() {
        // 递归创建配置目录
        fs::create_dir_all(&settings_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;

        // 使用模型中定义的默认配置
        let default_settings = AppSettings::default();
        // 将结构体序列化为 TOML 格式的字符串
        let toml_content =
            toml::to_string(&default_settings).map_err(|e| format!("序列化默认配置失败: {}", e))?;

        // 将配置内容写入文件
        fs::write(&settings_path, toml_content)
            .map_err(|e| format!("写入默认配置文件失败: {}", e))?;

        return Ok(default_settings);
    }

    // 读取现有的配置文件内容
    let content =
        fs::read_to_string(settings_path).map_err(|e| format!("读取配置文件失败: {}", e))?;

    // 将 TOML 字符串解析为 AppSettings 结构体
    let settings: AppSettings =
        toml::from_str(&content).map_err(|e| format!("解析设置文件失败: {}", e))?;

    Ok(settings)
}

/// 保存应用程序设置
///
/// 该函数将 AppSettings 结构体序列化为 TOML 并写入用户主目录下的 `.immich/config.toml` 文件。
pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let home = dirs::home_dir().ok_or("寻找$HOME目录失败")?;
    let settings_dir = home.join(".immich");
    let settings_path = settings_dir.join("config.toml");

    // 确保目录存在
    fs::create_dir_all(&settings_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;

    // 序列化
    let toml_content = toml::to_string(settings).map_err(|e| format!("序列化配置失败: {}", e))?;

    // 写入文件
    fs::write(settings_path, toml_content).map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}

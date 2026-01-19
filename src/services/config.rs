use crate::services::i18n::Language;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// 应用配置结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// 当前用户名
    #[serde(default)]
    pub current_user: String,
    /// 服务器地址
    #[serde(default = "default_server_url")]
    pub server_url: String,
    /// 并发数
    #[serde(default = "default_concurrency")]
    pub concurrency: u8,
    /// 界面语言
    #[serde(default)]
    pub language: Language,
    /// 日志级别
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// 用户列表
    #[serde(default)]
    pub users: HashMap<String, UserConfig>,
}

fn default_server_url() -> String {
    "http://localhost:2283".to_string()
}

fn default_concurrency() -> u8 {
    5
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            current_user: String::new(),
            server_url: "http://localhost:2283".to_string(),
            concurrency: 5,
            language: Language::default(),
            log_level: "info".to_string(),
            users: HashMap::new(),
        }
    }
}

/// 用户配置结构
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserConfig {
    pub api_key: String,
}

/// 从配置文件加载配置，不存在时创建默认配置
pub fn load_config() -> Result<AppConfig> {
    let home = dirs::home_dir().context("Failed to get home directory")?;
    let config_dir = home.join(".immich");
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        tracing::info!(
            "Config file not found, creating default config at: {:?}",
            config_path
        );
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        let default_config = AppConfig::default();
        let toml_content =
            toml::to_string(&default_config).context("Failed to serialize config")?;
        fs::write(&config_path, toml_content).context("Failed to write config file")?;
        return Ok(default_config);
    }

    tracing::debug!("Loading config from: {:?}", config_path);
    let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
    let config: AppConfig = toml::from_str(&content).context("Failed to parse config")?;
    Ok(config)
}

/// 保存配置到文件
pub fn save_config(config: &AppConfig) -> Result<()> {
    let home = dirs::home_dir().context("Failed to get home directory")?;
    let config_dir = home.join(".immich");
    let config_path = config_dir.join("config.toml");

    fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    let toml_content = toml::to_string(config).context("Failed to serialize config")?;
    fs::write(&config_path, toml_content).context("Failed to write config file")?;
    tracing::debug!("Config saved to: {:?}", config_path);
    Ok(())
}

use super::user::UserSetting;
use crate::i18n::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 应用程序全局设置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    /// 当前选中的用户名
    pub current_user: String,
    /// Immich 服务器的地址 (例如: http://192.168.1.100:2283)
    pub server_url: String,
    /// 上传时的并发数量限制
    pub concurrency: u8,
    /// 应用程序语言设置
    #[serde(default)]
    pub language: Language,
    /// 存储所有用户及其配置的映射表，Key 为用户名
    pub users: HashMap<String, UserSetting>,
}

impl Default for AppSettings {
    /// 提供默认配置，方便在找不到配置文件时进行初始化
    fn default() -> Self {
        Self {
            current_user: String::new(),
            server_url: "http://localhost:2283".to_string(),
            concurrency: 5,
            language: Language::default(),
            users: HashMap::new(),
        }
    }
}

impl AppSettings {
    /// 获取当前用户的 API Key
    /// 如果当前用户不存在于映射表中，则返回 None
    pub fn get_current_api_key(&self) -> Option<String> {
        self.users
            .get(&self.current_user)
            .map(|u| u.api_key.clone())
    }
}

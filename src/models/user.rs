use serde::{Deserialize, Serialize};

/// 用户设置结构体，存储与特定用户相关的配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserSetting {
    /// 用户的 API Key，用于身份验证
    pub api_key: String,
}

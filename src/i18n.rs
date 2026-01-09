//! 国际化 (i18n) 模块
//!
//! 提供多语言支持功能，目前支持中文和英文。
//! 包含语言枚举定义和所有界面文本的翻译。

use serde::{Deserialize, Serialize};

/// 支持的语言枚举
///
/// 目前支持中文和英文两种语言，使用 serde 进行序列化/反序列化
/// 以便在配置文件中保存用户的语言偏好。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Language {
    /// 中文
    #[serde(rename = "zh")]
    Chinese,
    /// 英文
    #[serde(rename = "en")]
    English,
}

impl Default for Language {
    /// 默认语言为英文
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    /// 获取语言代码
    ///
    /// # Returns
    ///
    /// * `&'static str` - 语言代码字符串 ("zh" 或 "en")
    pub fn code(&self) -> &'static str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }

    /// 获取语言的显示名称
    ///
    /// # Returns
    ///
    /// * `&'static str` - 用于界面显示的语言名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
        }
    }
}

/// 国际化管理器
///
/// 负责管理当前语言设置并提供所有界面文本的翻译功能。
/// 所有的界面文本都通过这个结构体的方法来获取，确保多语言支持的一致性。
#[derive(Clone)]
pub struct I18n {
    /// 当前选择的语言
    pub language: Language,
}

impl I18n {
    /// 创建新的国际化管理器实例
    ///
    /// # Arguments
    ///
    /// * `language` - 要使用的语言
    ///
    /// # Returns
    ///
    /// * `Self` - 新的 I18n 实例
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    // ========== 主页面文本 ==========
    /// 应用程序标题
    pub fn title(&self) -> &'static str {
        match self.language {
            Language::Chinese => "Immich Uploader",
            Language::English => "Immich Uploader",
        }
    }

    /// 设置按钮文本
    pub fn settings(&self) -> &'static str {
        match self.language {
            Language::Chinese => "设置",
            Language::English => "Settings",
        }
    }

    /// 选择上传账号标签
    pub fn select_upload_account(&self) -> &'static str {
        match self.language {
            Language::Chinese => "选择上传账号",
            Language::English => "Select Upload Account",
        }
    }

    /// 提示用户在设置中添加账号
    pub fn add_account_in_settings(&self) -> &'static str {
        match self.language {
            Language::Chinese => "请在设置中添加账号",
            Language::English => "Please add account in settings",
        }
    }

    /// 未找到配置文件提示
    pub fn no_config_found(&self) -> &'static str {
        match self.language {
            Language::Chinese => "未发现配置文件",
            Language::English => "No configuration found",
        }
    }

    /// 选择文件按钮文本
    pub fn select_files(&self) -> &'static str {
        match self.language {
            Language::Chinese => "选择文件",
            Language::English => "Select Files",
        }
    }

    /// 清除按钮文本
    pub fn clear(&self) -> &'static str {
        match self.language {
            Language::Chinese => "清除",
            Language::English => "Clear",
        }
    }

    /// 开始上传按钮文本
    pub fn start_upload(&self) -> &'static str {
        match self.language {
            Language::Chinese => "开始上传",
            Language::English => "Start Upload",
        }
    }

    /// 上传中状态文本
    pub fn uploading(&self) -> &'static str {
        match self.language {
            Language::Chinese => "正在上传...",
            Language::English => "Uploading...",
        }
    }

    /// 未选择文件提示
    pub fn no_files_selected(&self) -> &'static str {
        match self.language {
            Language::Chinese => "未选择文件",
            Language::English => "No files selected",
        }
    }

    /// 待上传文件列表标题
    ///
    /// # Arguments
    ///
    /// * `count` - 文件数量
    pub fn files_to_upload(&self, count: usize) -> String {
        match self.language {
            Language::Chinese => format!("待上传文件 ({}):", count),
            Language::English => format!("Files to upload ({}):", count),
        }
    }

    /// 准备就绪状态文本
    pub fn ready(&self) -> &'static str {
        match self.language {
            Language::Chinese => "准备就绪",
            Language::English => "Ready",
        }
    }

    /// 切换用户成功提示
    ///
    /// # Arguments
    ///
    /// * `username` - 用户名
    pub fn switched_to_user(&self, username: &str) -> String {
        match self.language {
            Language::Chinese => format!("已切换至用户: {}", username),
            Language::English => format!("Switched to user: {}", username),
        }
    }

    /// 文件选择成功提示
    ///
    /// # Arguments
    ///
    /// * `count` - 选择的文件数量
    pub fn files_selected(&self, count: usize) -> String {
        match self.language {
            Language::Chinese => format!("已选择 {} 个文件", count),
            Language::English => format!("{} files selected", count),
        }
    }

    /// 清除选择成功提示
    pub fn selection_cleared(&self) -> &'static str {
        match self.language {
            Language::Chinese => "已清除选择",
            Language::English => "Selection cleared",
        }
    }

    /// API Key 未找到错误提示
    pub fn user_api_key_not_found(&self) -> &'static str {
        match self.language {
            Language::Chinese => "未找到选定用户的 API Key",
            Language::English => "API Key not found for selected user",
        }
    }

    /// 配置文件未找到错误提示
    pub fn config_file_not_found(&self) -> &'static str {
        match self.language {
            Language::Chinese => "未找到配置文件",
            Language::English => "Configuration file not found",
        }
    }

    /// 开始并行上传提示
    ///
    /// # Arguments
    ///
    /// * `total` - 总文件数
    /// * `concurrency` - 并发数
    pub fn start_parallel_upload(&self, total: usize, concurrency: usize) -> String {
        match self.language {
            Language::Chinese => {
                format!("开始并行上传 {} 个文件 (并发数: {})...", total, concurrency)
            }
            Language::English => format!(
                "Starting parallel upload of {} files (concurrency: {})...",
                total, concurrency
            ),
        }
    }

    /// 单个文件上传成功提示
    ///
    /// # Arguments
    ///
    /// * `current` - 当前完成数量
    /// * `total` - 总文件数
    /// * `filename` - 文件名
    pub fn upload_success(&self, current: usize, total: usize, filename: &str) -> String {
        match self.language {
            Language::Chinese => format!("[{}/{}] 成功上传: {}", current, total, filename),
            Language::English => format!(
                "[{}/{}] Successfully uploaded: {}",
                current, total, filename
            ),
        }
    }

    /// 单个文件上传失败提示
    ///
    /// # Arguments
    ///
    /// * `filename` - 文件名
    /// * `error` - 错误信息
    pub fn upload_failed(&self, filename: &str, error: &str) -> String {
        match self.language {
            Language::Chinese => format!("上传 {} 失败: {}", filename, error),
            Language::English => format!("Failed to upload {}: {}", filename, error),
        }
    }

    /// 所有文件上传完成提示
    ///
    /// # Arguments
    ///
    /// * `total` - 总文件数
    pub fn all_files_uploaded(&self, total: usize) -> String {
        match self.language {
            Language::Chinese => format!("成功上传全部 {} 个文件！", total),
            Language::English => format!("Successfully uploaded all {} files!", total),
        }
    }

    /// 文件选择对话框标题
    pub fn select_photos_or_videos(&self) -> &'static str {
        match self.language {
            Language::Chinese => "选择照片或视频",
            Language::English => "Select photos or videos",
        }
    }

    // ========== 设置页面文本 ==========
    /// 返回按钮文本
    pub fn back(&self) -> &'static str {
        match self.language {
            Language::Chinese => "返回",
            Language::English => "Back",
        }
    }

    /// 服务器 URL 标签
    pub fn server_url(&self) -> &'static str {
        match self.language {
            Language::Chinese => "服务器 URL",
            Language::English => "Server URL",
        }
    }

    /// 保存按钮文本
    pub fn save(&self) -> &'static str {
        match self.language {
            Language::Chinese => "保存",
            Language::English => "Save",
        }
    }

    /// 并发上传数标签
    pub fn concurrency(&self) -> &'static str {
        match self.language {
            Language::Chinese => "并发上传数",
            Language::English => "Upload Concurrency",
        }
    }

    /// 语言设置标签
    pub fn language(&self) -> &'static str {
        match self.language {
            Language::Chinese => "语言",
            Language::English => "Language",
        }
    }

    /// 用户管理标题
    pub fn user_management(&self) -> &'static str {
        match self.language {
            Language::Chinese => "用户管理",
            Language::English => "User Management",
        }
    }

    /// 用户名标签
    pub fn username(&self) -> &'static str {
        match self.language {
            Language::Chinese => "用户名",
            Language::English => "Username",
        }
    }

    /// 添加按钮文本
    pub fn add(&self) -> &'static str {
        match self.language {
            Language::Chinese => "添加",
            Language::English => "Add",
        }
    }

    /// 无用户提示
    pub fn no_users(&self) -> &'static str {
        match self.language {
            Language::Chinese => "暂无用户",
            Language::English => "No users",
        }
    }

    /// 设为默认按钮文本
    pub fn set_as_default(&self) -> &'static str {
        match self.language {
            Language::Chinese => "设为默认",
            Language::English => "Set as Default",
        }
    }

    /// 删除按钮文本
    pub fn delete(&self) -> &'static str {
        match self.language {
            Language::Chinese => "删除",
            Language::English => "Delete",
        }
    }

    /// 设置页面副标题
    pub fn manage_your_settings(&self) -> &'static str {
        match self.language {
            Language::Chinese => "管理您的设置",
            Language::English => "Manage your settings",
        }
    }

    /// 保存失败错误提示
    ///
    /// # Arguments
    ///
    /// * `error` - 错误信息
    pub fn save_failed(&self, error: &str) -> String {
        match self.language {
            Language::Chinese => format!("保存失败: {}", error),
            Language::English => format!("Save failed: {}", error),
        }
    }

    /// 服务器 URL 保存成功提示
    pub fn server_url_saved(&self) -> &'static str {
        match self.language {
            Language::Chinese => "服务器 URL 已保存",
            Language::English => "Server URL saved",
        }
    }

    /// 并发设置保存成功提示
    pub fn concurrency_saved(&self) -> &'static str {
        match self.language {
            Language::Chinese => "并发设置已保存",
            Language::English => "Concurrency settings saved",
        }
    }

    /// 语言设置保存成功提示
    pub fn language_saved(&self) -> &'static str {
        match self.language {
            Language::Chinese => "语言设置已保存",
            Language::English => "Language settings saved",
        }
    }

    /// 请填写完整信息提示
    pub fn please_fill_complete_info(&self) -> &'static str {
        match self.language {
            Language::Chinese => "请填写完整信息",
            Language::English => "Please fill in complete information",
        }
    }

    /// 用户添加成功提示
    pub fn user_added(&self) -> &'static str {
        match self.language {
            Language::Chinese => "用户已添加",
            Language::English => "User added",
        }
    }

    pub fn default_user_changed(&self, username: &str) -> String {
        match self.language {
            Language::Chinese => format!("默认用户已更改为: {}", username),
            Language::English => format!("Default user changed to: {}", username),
        }
    }

    pub fn user_deleted(&self, username: &str) -> String {
        match self.language {
            Language::Chinese => format!("用户 {} 已删除", username),
            Language::English => format!("User {} deleted", username),
        }
    }
}

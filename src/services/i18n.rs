use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "en")]
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
        }
    }
}

#[derive(Clone)]
pub struct I18n {
    pub language: Language,
}

macro_rules! text {
    ($self:expr, $zh:expr, $en:expr) => {
        match $self.language {
            Language::Chinese => $zh,
            Language::English => $en,
        }
    };
}

impl I18n {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn home_header(&self) -> &'static str {
        text!(self, "上传照片 & 视频", "Upload Photos & Videos")
    }

    pub fn settings_header(&self) -> &'static str {
        text!(self, "设置", "Settings")
    }

    pub fn select_upload_account(&self) -> &'static str {
        text!(self, "选择上传账号", "Select Upload Account")
    }

    pub fn add_account_in_settings(&self) -> &'static str {
        text!(self, "请在设置中添加账号", "Please add account in settings")
    }

    pub fn select_files(&self) -> &'static str {
        text!(self, "选择文件", "Select Files")
    }

    pub fn clear(&self) -> &'static str {
        text!(self, "清除", "Clear")
    }

    pub fn start_upload(&self) -> &'static str {
        text!(self, "开始上传", "Start Upload")
    }

    pub fn uploading(&self) -> &'static str {
        text!(self, "正在上传...", "Uploading...")
    }

    pub fn no_files_selected(&self) -> &'static str {
        text!(self, "未选择文件", "No files selected")
    }

    pub fn files_to_upload(&self, count: usize) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("待上传文件 ({}):", count),
                format!("Files to upload ({}):", count)
            )
        )
    }

    pub fn ready(&self) -> &'static str {
        text!(self, "准备就绪", "Ready")
    }

    pub fn switched_to_user(&self, username: &str) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("已切换至用户: {}", username),
                format!("Switched to user: {}", username)
            )
        )
    }

    pub fn files_selected(&self, count: usize) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("已选择 {} 个文件", count),
                format!("{} files selected", count)
            )
        )
    }

    pub fn selection_cleared(&self) -> &'static str {
        text!(self, "已清除选择", "Selection cleared")
    }

    pub fn user_api_key_not_found(&self) -> &'static str {
        text!(
            self,
            "未找到选定用户的 API Key",
            "API Key not found for selected user"
        )
    }

    pub fn config_file_not_found(&self) -> &'static str {
        text!(self, "未找到配置文件", "Configuration file not found")
    }

    pub fn start_parallel_upload(&self, total: usize) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("开始并行上传{}个文件...", total),
                format!("Starting upload of {} files ...", total)
            )
        )
    }

    pub fn upload_success(&self, current: usize, total: usize, filename: &str) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("[{}/{}] 成功上传: {}", current, total, filename),
                format!(
                    "[{}/{}] Successfully uploaded: {}",
                    current, total, filename
                )
            )
        )
    }

    pub fn upload_failed(&self, filename: &str, error: &str) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("上传 {} 失败: {}", filename, error),
                format!("Failed to upload {}: {}", filename, error)
            )
        )
    }

    pub fn all_files_uploaded(&self, total: usize) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("成功上传全部 {} 个文件！", total),
                format!("Successfully uploaded all {} files!", total)
            )
        )
    }

    pub fn select_photos_or_videos(&self) -> &'static str {
        text!(self, "选择照片或视频", "Select photos or videos")
    }

    pub fn server_url(&self) -> &'static str {
        text!(self, "服务器URL", "Server URL")
    }

    pub fn save(&self) -> &'static str {
        text!(self, "保存", "Save")
    }

    pub fn concurrency(&self) -> &'static str {
        text!(self, "并发数", "Concurrency")
    }

    pub fn language(&self) -> &'static str {
        text!(self, "语言", "Language")
    }

    pub fn user_management(&self) -> &'static str {
        text!(self, "用户管理", "User Management")
    }

    pub fn username(&self) -> &'static str {
        text!(self, "用户名", "UserName")
    }

    pub fn add(&self) -> &'static str {
        text!(self, "添加", "Add")
    }

    pub fn no_users(&self) -> &'static str {
        text!(self, "暂无用户", "No users")
    }

    pub fn set_as_default(&self) -> &'static str {
        text!(self, "默认", "Default")
    }

    pub fn delete(&self) -> &'static str {
        text!(self, "删除", "Delete")
    }

    pub fn manage_your_settings(&self) -> &'static str {
        text!(self, "管理您的设置", "Manage your settings")
    }

    pub fn save_failed(&self, error: &str) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("保存失败: {}", error),
                format!("Save failed: {}", error)
            )
        )
    }

    pub fn server_url_saved(&self) -> &'static str {
        text!(self, "服务器URL已保存", "Server URL saved")
    }

    pub fn concurrency_saved(&self) -> &'static str {
        text!(self, "并发设置已保存", "Concurrency settings saved")
    }

    pub fn invalid_concurrency(&self) -> &'static str {
        text!(
            self,
            "并发数必须在 1-16 之间",
            "Concurrency must be between 1 and 16"
        )
    }

    pub fn language_saved(&self) -> &'static str {
        text!(self, "语言设置已保存", "Language settings saved")
    }

    pub fn please_fill_complete_info(&self) -> &'static str {
        text!(
            self,
            "请填写完整信息",
            "Please fill in complete information"
        )
    }

    pub fn user_added(&self) -> &'static str {
        text!(self, "用户已添加", "User added")
    }

    pub fn default_user_changed(&self, username: &str) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("默认用户已更改为: {}", username),
                format!("Default user changed to: {}", username)
            )
        )
    }

    pub fn user_deleted(&self, username: &str) -> String {
        format!(
            "{}",
            text!(
                self,
                format!("用户 {} 已删除", username),
                format!("User {} deleted", username)
            )
        )
    }
}

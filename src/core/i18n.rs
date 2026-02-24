use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextKey {
    HomeHeader,
    SettingsHeader,
    SelectUploadAccount,
    AddAccountInSettings,
    SelectFiles,
    Clear,
    StartUpload,
    Uploading,
    NoFilesSelected,
    FilesToUpload,
    Ready,
    SwitchedToUser,
    FilesSelected,
    SelectionCleared,
    UserApiKeyNotFound,
    ConfigFileNotFound,
    StartParallelUpload,
    UploadSuccess,
    UploadFailed,
    AllFilesUploaded,
    SelectPhotosOrVideos,
    ServerUrl,
    Save,
    Concurrency,
    Language,
    UserManagement,
    Username,
    Add,
    NoUsers,
    SetAsDefault,
    Delete,
    ManageYourSettings,
    SaveFailed,
    ServerUrlSaved,
    ConcurrencySaved,
    InvalidConcurrency,
    LanguageSaved,
    PleaseFillCompleteInfo,
    UserAdded,
    DefaultUserChanged,
    UserDeleted,
}

fn build_translations() -> HashMap<(Language, TextKey), &'static str> {
    let mut m = HashMap::new();
    m.insert((Language::Chinese, TextKey::HomeHeader), "上传照片 & 视频");
    m.insert((Language::Chinese, TextKey::SettingsHeader), "设置");
    m.insert(
        (Language::Chinese, TextKey::SelectUploadAccount),
        "选择上传账号",
    );
    m.insert(
        (Language::Chinese, TextKey::AddAccountInSettings),
        "请在设置中添加账号",
    );
    m.insert((Language::Chinese, TextKey::SelectFiles), "选择文件");
    m.insert((Language::Chinese, TextKey::Clear), "清除");
    m.insert((Language::Chinese, TextKey::StartUpload), "开始上传");
    m.insert((Language::Chinese, TextKey::Uploading), "正在上传...");
    m.insert((Language::Chinese, TextKey::NoFilesSelected), "未选择文件");
    m.insert(
        (Language::Chinese, TextKey::FilesToUpload),
        "待上传文件 ({}):",
    );
    m.insert((Language::Chinese, TextKey::Ready), "准备就绪");
    m.insert(
        (Language::Chinese, TextKey::SwitchedToUser),
        "已切换至用户: {}",
    );
    m.insert(
        (Language::Chinese, TextKey::FilesSelected),
        "已选择 {} 个文件",
    );
    m.insert((Language::Chinese, TextKey::SelectionCleared), "已清除选择");
    m.insert(
        (Language::Chinese, TextKey::UserApiKeyNotFound),
        "未找到选定用户的 API Key",
    );
    m.insert(
        (Language::Chinese, TextKey::ConfigFileNotFound),
        "未找到配置文件",
    );
    m.insert(
        (Language::Chinese, TextKey::StartParallelUpload),
        "开始并行上传{}个文件...",
    );
    m.insert(
        (Language::Chinese, TextKey::UploadSuccess),
        "[{}/{}] 成功上传: {}",
    );
    m.insert(
        (Language::Chinese, TextKey::UploadFailed),
        "上传 {} 失败: {}",
    );
    m.insert(
        (Language::Chinese, TextKey::AllFilesUploaded),
        "成功上传全部 {} 个文件！",
    );
    m.insert(
        (Language::Chinese, TextKey::SelectPhotosOrVideos),
        "选择照片或视频",
    );
    m.insert((Language::Chinese, TextKey::ServerUrl), "服务器URL");
    m.insert((Language::Chinese, TextKey::Save), "保存");
    m.insert((Language::Chinese, TextKey::Concurrency), "并发数");
    m.insert((Language::Chinese, TextKey::Language), "语言");
    m.insert((Language::Chinese, TextKey::UserManagement), "用户管理");
    m.insert((Language::Chinese, TextKey::Username), "用户名");
    m.insert((Language::Chinese, TextKey::Add), "添加");
    m.insert((Language::Chinese, TextKey::NoUsers), "暂无用户");
    m.insert((Language::Chinese, TextKey::SetAsDefault), "默认");
    m.insert((Language::Chinese, TextKey::Delete), "删除");
    m.insert(
        (Language::Chinese, TextKey::ManageYourSettings),
        "管理您的设置",
    );
    m.insert((Language::Chinese, TextKey::SaveFailed), "保存失败: {}");
    m.insert(
        (Language::Chinese, TextKey::ServerUrlSaved),
        "服务器URL已保存",
    );
    m.insert(
        (Language::Chinese, TextKey::ConcurrencySaved),
        "并发设置已保存",
    );
    m.insert(
        (Language::Chinese, TextKey::InvalidConcurrency),
        "并发数必须在 1-16 之间",
    );
    m.insert(
        (Language::Chinese, TextKey::LanguageSaved),
        "语言设置已保存",
    );
    m.insert(
        (Language::Chinese, TextKey::PleaseFillCompleteInfo),
        "请填写完整信息",
    );
    m.insert((Language::Chinese, TextKey::UserAdded), "用户已添加");
    m.insert(
        (Language::Chinese, TextKey::DefaultUserChanged),
        "默认用户已更改为: {}",
    );
    m.insert((Language::Chinese, TextKey::UserDeleted), "用户 {} 已删除");
    m.insert(
        (Language::English, TextKey::HomeHeader),
        "Upload Photos & Videos",
    );
    m.insert((Language::English, TextKey::SettingsHeader), "Settings");
    m.insert(
        (Language::English, TextKey::SelectUploadAccount),
        "Select Upload Account",
    );
    m.insert(
        (Language::English, TextKey::AddAccountInSettings),
        "Please add account in settings",
    );
    m.insert((Language::English, TextKey::SelectFiles), "Select Files");
    m.insert((Language::English, TextKey::Clear), "Clear");
    m.insert((Language::English, TextKey::StartUpload), "Start Upload");
    m.insert((Language::English, TextKey::Uploading), "Uploading...");
    m.insert(
        (Language::English, TextKey::NoFilesSelected),
        "No files selected",
    );
    m.insert(
        (Language::English, TextKey::FilesToUpload),
        "Files to upload ({}):",
    );
    m.insert((Language::English, TextKey::Ready), "Ready");
    m.insert(
        (Language::English, TextKey::SwitchedToUser),
        "Switched to user: {}",
    );
    m.insert(
        (Language::English, TextKey::FilesSelected),
        "{} files selected",
    );
    m.insert(
        (Language::English, TextKey::SelectionCleared),
        "Selection cleared",
    );
    m.insert(
        (Language::English, TextKey::UserApiKeyNotFound),
        "API Key not found for selected user",
    );
    m.insert(
        (Language::English, TextKey::ConfigFileNotFound),
        "Configuration file not found",
    );
    m.insert(
        (Language::English, TextKey::StartParallelUpload),
        "Starting upload of {} files ...",
    );
    m.insert(
        (Language::English, TextKey::UploadSuccess),
        "[{}/{}] Successfully uploaded: {}",
    );
    m.insert(
        (Language::English, TextKey::UploadFailed),
        "Failed to upload {}: {}",
    );
    m.insert(
        (Language::English, TextKey::AllFilesUploaded),
        "Successfully uploaded all {} files!",
    );
    m.insert(
        (Language::English, TextKey::SelectPhotosOrVideos),
        "Select photos or videos",
    );
    m.insert((Language::English, TextKey::ServerUrl), "Server URL");
    m.insert((Language::English, TextKey::Save), "Save");
    m.insert((Language::English, TextKey::Concurrency), "Concurrency");
    m.insert((Language::English, TextKey::Language), "Language");
    m.insert(
        (Language::English, TextKey::UserManagement),
        "User Management",
    );
    m.insert((Language::English, TextKey::Username), "UserName");
    m.insert((Language::English, TextKey::Add), "Add");
    m.insert((Language::English, TextKey::NoUsers), "No users");
    m.insert((Language::English, TextKey::SetAsDefault), "Default");
    m.insert((Language::English, TextKey::Delete), "Delete");
    m.insert(
        (Language::English, TextKey::ManageYourSettings),
        "Manage your settings",
    );
    m.insert((Language::English, TextKey::SaveFailed), "Save failed: {}");
    m.insert(
        (Language::English, TextKey::ServerUrlSaved),
        "Server URL saved",
    );
    m.insert(
        (Language::English, TextKey::ConcurrencySaved),
        "Concurrency settings saved",
    );
    m.insert(
        (Language::English, TextKey::InvalidConcurrency),
        "Concurrency must be between 1 and 16",
    );
    m.insert(
        (Language::English, TextKey::LanguageSaved),
        "Language settings saved",
    );
    m.insert(
        (Language::English, TextKey::PleaseFillCompleteInfo),
        "Please fill in complete information",
    );
    m.insert((Language::English, TextKey::UserAdded), "User added");
    m.insert(
        (Language::English, TextKey::DefaultUserChanged),
        "Default user changed to: {}",
    );
    m.insert((Language::English, TextKey::UserDeleted), "User {} deleted");
    m
}

#[derive(Clone)]
pub struct I18n {
    pub language: Language,
    translations: HashMap<(Language, TextKey), &'static str>,
}

impl I18n {
    fn get(&self, key: TextKey) -> &'static str {
        self.translations
            .get(&(self.language, key))
            .copied()
            .unwrap_or("Missing translation")
    }

    pub fn new(language: Language) -> Self {
        Self {
            language,
            translations: build_translations(),
        }
    }

    pub fn home_header(&self) -> &'static str {
        self.get(TextKey::HomeHeader)
    }
    pub fn settings_header(&self) -> &'static str {
        self.get(TextKey::SettingsHeader)
    }
    pub fn select_upload_account(&self) -> &'static str {
        self.get(TextKey::SelectUploadAccount)
    }
    pub fn add_account_in_settings(&self) -> &'static str {
        self.get(TextKey::AddAccountInSettings)
    }
    pub fn select_files(&self) -> &'static str {
        self.get(TextKey::SelectFiles)
    }
    pub fn clear(&self) -> &'static str {
        self.get(TextKey::Clear)
    }
    pub fn start_upload(&self) -> &'static str {
        self.get(TextKey::StartUpload)
    }
    pub fn uploading(&self) -> &'static str {
        self.get(TextKey::Uploading)
    }
    pub fn no_files_selected(&self) -> &'static str {
        self.get(TextKey::NoFilesSelected)
    }
    pub fn files_to_upload(&self, count: usize) -> String {
        self.get(TextKey::FilesToUpload)
            .replace("{}", &count.to_string())
    }
    pub fn ready(&self) -> &'static str {
        self.get(TextKey::Ready)
    }
    pub fn switched_to_user(&self, username: &str) -> String {
        self.get(TextKey::SwitchedToUser).replace("{}", username)
    }
    pub fn files_selected(&self, count: usize) -> String {
        self.get(TextKey::FilesSelected)
            .replace("{}", &count.to_string())
    }
    pub fn selection_cleared(&self) -> &'static str {
        self.get(TextKey::SelectionCleared)
    }
    pub fn user_api_key_not_found(&self) -> &'static str {
        self.get(TextKey::UserApiKeyNotFound)
    }
    pub fn config_file_not_found(&self) -> &'static str {
        self.get(TextKey::ConfigFileNotFound)
    }
    pub fn start_parallel_upload(&self, total: usize) -> String {
        self.get(TextKey::StartParallelUpload)
            .replace("{}", &total.to_string())
    }
    pub fn upload_success(&self, current: usize, total: usize, filename: &str) -> String {
        self.get(TextKey::UploadSuccess)
            .replacen("{}", &current.to_string(), 1)
            .replacen("{}", &total.to_string(), 1)
            .replacen("{}", filename, 1)
    }
    pub fn upload_failed(&self, filename: &str, error: &str) -> String {
        self.get(TextKey::UploadFailed)
            .replacen("{}", filename, 1)
            .replacen("{}", error, 1)
    }
    pub fn all_files_uploaded(&self, total: usize) -> String {
        self.get(TextKey::AllFilesUploaded)
            .replace("{}", &total.to_string())
    }
    pub fn select_photos_or_videos(&self) -> &'static str {
        self.get(TextKey::SelectPhotosOrVideos)
    }
    pub fn server_url(&self) -> &'static str {
        self.get(TextKey::ServerUrl)
    }
    pub fn save(&self) -> &'static str {
        self.get(TextKey::Save)
    }
    pub fn concurrency(&self) -> &'static str {
        self.get(TextKey::Concurrency)
    }
    pub fn language(&self) -> &'static str {
        self.get(TextKey::Language)
    }
    pub fn user_management(&self) -> &'static str {
        self.get(TextKey::UserManagement)
    }
    pub fn username(&self) -> &'static str {
        self.get(TextKey::Username)
    }
    pub fn add(&self) -> &'static str {
        self.get(TextKey::Add)
    }
    pub fn no_users(&self) -> &'static str {
        self.get(TextKey::NoUsers)
    }
    pub fn set_as_default(&self) -> &'static str {
        self.get(TextKey::SetAsDefault)
    }
    pub fn delete(&self) -> &'static str {
        self.get(TextKey::Delete)
    }
    pub fn manage_your_settings(&self) -> &'static str {
        self.get(TextKey::ManageYourSettings)
    }
    pub fn save_failed(&self, error: &str) -> String {
        self.get(TextKey::SaveFailed).replace("{}", error)
    }
    pub fn server_url_saved(&self) -> &'static str {
        self.get(TextKey::ServerUrlSaved)
    }
    pub fn concurrency_saved(&self) -> &'static str {
        self.get(TextKey::ConcurrencySaved)
    }
    pub fn invalid_concurrency(&self) -> &'static str {
        self.get(TextKey::InvalidConcurrency)
    }
    pub fn language_saved(&self) -> &'static str {
        self.get(TextKey::LanguageSaved)
    }
    pub fn please_fill_complete_info(&self) -> &'static str {
        self.get(TextKey::PleaseFillCompleteInfo)
    }
    pub fn user_added(&self) -> &'static str {
        self.get(TextKey::UserAdded)
    }
    pub fn default_user_changed(&self, username: &str) -> String {
        self.get(TextKey::DefaultUserChanged)
            .replace("{}", username)
    }
    pub fn user_deleted(&self, username: &str) -> String {
        self.get(TextKey::UserDeleted).replace("{}", username)
    }
}

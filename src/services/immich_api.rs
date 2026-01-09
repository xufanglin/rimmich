use chrono::{SecondsFormat, Utc};
use reqwest::{Body, multipart};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// 上传资源（图片或视频）到 Immich 服务器
///
/// # 参数
/// * `server_url` - Immich 服务器的主机地址
/// * `api_key` - 用户的 API Key
/// * `file_path` - 要上传的文件的本地路径
pub async fn upload_asset(
    server_url: &str,
    api_key: &str,
    file_path: PathBuf,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    // 构造 API 终点，确保服务器 URL 末尾没有多余的斜杠
    let url = format!("{}/api/assets", server_url.trim_end_matches("/"));

    // 获取文件名，如果获取失败则默认使用 "image.jpg"
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image.jpg");

    // 读取文件头部用于检测 MIME 类型
    let mut buffer = [0; 512];
    let mut file_for_detection = File::open(&file_path).await.map_err(|e| e.to_string())?;
    let bytes_read = tokio::io::AsyncReadExt::read(&mut file_for_detection, &mut buffer)
        .await
        .map_err(|e| e.to_string())?;

    // 使用 magic number 检测 MIME 类型
    let mime_type = infer::get(&buffer[..bytes_read])
        .map(|kind| kind.mime_type())
        .unwrap_or("application/octet-stream");

    // 获取文件元数据（如创建时间和修改时间）
    let metadata = std::fs::metadata(&file_path).map_err(|e| e.to_string())?;
    let file_size = metadata.len();

    // 打开文件用于流式读取
    let file = File::open(&file_path).await.map_err(|e| e.to_string())?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    // 将时间转换为 ISO 8601 格式: 2024-01-04T12:34:56.000Z
    let created: chrono::DateTime<Utc> = metadata
        .created()
        .unwrap_or(std::time::SystemTime::now())
        .into();
    let modified: chrono::DateTime<Utc> = metadata
        .modified()
        .unwrap_or(std::time::SystemTime::now())
        .into();

    let created_str = created.to_rfc3339_opts(SecondsFormat::Millis, true);
    let modified_str = modified.to_rfc3339_opts(SecondsFormat::Millis, true);

    // 生成设备端的资源 ID，这里使用 文件名-长度 的组合作为简单标识
    let device_asset_id = format!("{}-{}", file_name, file_size);

    // 构造 multipart 表单中的文件部分，使用流式上传
    let file_part = multipart::Part::stream(body)
        .file_name(file_name.to_string())
        .mime_str(mime_type)
        .map_err(|e| e.to_string())?;

    // 构造 multipart 表单
    // 重要提示：某些多部分解析器要求元数据字段必须在文件数据之前
    let form = multipart::Form::new()
        .text("deviceAssetId", device_asset_id)
        .text("deviceId", "rimmich-desktop") // 设备标识，可自定义
        .text("fileCreatedAt", created_str)
        .text("fileModifiedAt", modified_str)
        .text("isFavorite", "false")
        .text("isReadOnly", "false")
        .text("filename", file_name.to_string())
        .part("assetData", file_part); // 实际的文件数据

    // 发送 POST 请求
    let res = client
        .post(url)
        .header("x-api-key", api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    // 检查响应状态
    if status.is_success() {
        Ok(())
    } else {
        // 如果失败，尝试读取服务器返回的错误详情
        let error_text = res
            .text()
            .await
            .unwrap_or_else(|_| "无法读取错误详情".to_string());
        Err(format!("返回状态: {}, 详情: {}", status, error_text))
    }
}

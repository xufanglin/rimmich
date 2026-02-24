use anyhow::Result;
use std::path::PathBuf;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 初始化日志系统
///
/// 日志会同时输出到控制台和文件 (~/.immich/immich.log)
pub fn init_logger(log_level: &str) -> Result<()> {
    // 创建日志目录
    let log_dir = dirs::home_dir()
        .map(|home| home.join(".immich"))
        .unwrap_or_else(|| PathBuf::from(".immich"));

    std::fs::create_dir_all(&log_dir)?;

    // 配置日志级别
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    // 文件日志 appender - 每天滚动，自动保留最近7天
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("immich")
        .filename_suffix("log")
        .max_log_files(7)
        .build(&log_dir)?;

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 组合控制台和文件输出
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    // 防止 _guard 被提前 drop
    std::mem::forget(_guard);

    tracing::info!("Application started with log level: {}", log_level);

    Ok(())
}

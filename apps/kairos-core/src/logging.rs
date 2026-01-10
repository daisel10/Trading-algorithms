// Logging module - Simplified hybrid console + file logging

use crate::config::{LoggingSettings, Settings};
use anyhow::Result;
use std::io;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the hybrid logging system
pub fn init_logging(settings: &Settings) -> Result<()> {
    // Set environment variables for tracing
    std::env::set_var("RUST_LOG", &settings.logging.rust_log);
    std::env::set_var("RUST_BACKTRACE", &settings.logging.rust_backtrace);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&settings.logging.rust_log));

    if settings.logging.enable_file_logging {
        init_hybrid_logging(&settings.logging, env_filter)?;
    } else {
        init_console_only_logging(env_filter)?;
    }

    Ok(())
}

/// Initialize hybrid logging (console + file)  
fn init_hybrid_logging(config: &LoggingSettings, env_filter: EnvFilter) -> Result<()> {
    std::fs::create_dir_all(&config.log_directory)?;

    let rotation = match config.rotation.as_str() {
        "hourly" => Rotation::HOURLY,
        "daily" => Rotation::DAILY,
        _ => Rotation::DAILY,
    };

    let file_appender = RollingFileAppender::new(
        rotation,
        &config.log_directory,
        &format!("{}.log", config.log_file_prefix),
    );

    // Console layer - human readable
    let console_layer = fmt::layer()
        .compact()
        .with_target(false)
        .with_writer(io::stdout);

    // File layer - using fmt::Layer with file appender
    // Note: For true JSON, we'd need a different approach, but this gives structured output
    let file_layer = fmt::layer()
        .with_ansi(false) // No color codes in files
        .with_writer(file_appender);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(())
}

/// Initialize console-only logging
fn init_console_only_logging(env_filter: EnvFilter) -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .compact()
        .with_target(false)
        .init();

    Ok(())
}

/// Clean up old log files
pub fn cleanup_old_logs(config: &LoggingSettings) -> Result<()> {
    if config.max_log_files == 0 {
        return Ok(());
    }

    let log_dir = std::path::Path::new(&config.log_directory);
    if !log_dir.exists() {
        return Ok(());
    }

    let mut log_files: Vec<_> = std::fs::read_dir(log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(&config.log_file_prefix)
        })
        .collect();

    log_files.sort_by_key(|entry| {
        entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    let files_to_delete = log_files.len().saturating_sub(config.max_log_files);
    for entry in log_files.iter().take(files_to_delete) {
        std::fs::remove_file(entry.path()).ok();
    }

    Ok(())
}

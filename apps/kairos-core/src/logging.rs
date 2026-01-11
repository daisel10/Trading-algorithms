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

    // Determine console format
    let console_format = config.console_format.as_str();
    let file_format = config.file_format.as_str();

    // We need to use different initialization approaches based on format combinations
    // because different formats produce different layer types that can't be easily combined

    match (console_format, file_format) {
        // Both JSON - requires json feature in tracing-subscriber
        ("json", "json") => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer().with_writer(io::stdout).json())
                .with(fmt::layer().with_writer(file_appender).json())
                .init();
        }
        // Console: human, File: JSON (recommended for production)
        ("human", "json") | (_, "json") => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .compact()
                        .with_target(false)
                        .with_writer(io::stdout),
                )
                .with(fmt::layer().with_writer(file_appender).json())
                .init();
        }
        // Console: JSON, File: human
        ("json", "human") | ("json", _) => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer().with_writer(io::stdout).json())
                .with(
                    fmt::layer()
                        .compact()
                        .with_ansi(false)
                        .with_writer(file_appender),
                )
                .init();
        }
        // Both human (default)
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .compact()
                        .with_target(false)
                        .with_writer(io::stdout),
                )
                .with(
                    fmt::layer()
                        .compact()
                        .with_ansi(false)
                        .with_writer(file_appender),
                )
                .init();
        }
    }

    Ok(())
}

/// Initialize console-only logging
fn init_console_only_logging(env_filter: EnvFilter) -> Result<()> {
    // Note: We don't have access to config here, so we default to human-readable
    // If console JSON is needed without file logging, pass config to this function
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

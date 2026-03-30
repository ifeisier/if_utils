//! `flexi_logger` 日志.
//!
//! 使用例子:
//!
//! ```no_run
//! use rust_tools::flexi_logger::init_flexi_logger;
//!
//! async fn async_main() {
//!    let logger = init_flexi_logger("quant_crawler", "./logs", "debug");
//!    let logger = match logger {
//!        Ok(v) => v,
//!        Err(e) => {
//!            eprintln!("初始化日志失败: {e}");
//!            exit(1);
//!        }
//!    };
//!
//!    logger.flush();
//!    logger.shutdown();
//!}
//! ```

use anyhow::Result;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
    WriteMode,
};
use log::Record;
use std::{thread, time::Duration};

/// 初始化 `flexi_logger`.
///
/// # Arguments
///
/// * `name` - 日志文件名
/// * `log_directory` - 日志文件目录
/// * `level` - 日志级别
///
/// # Errors
///
/// 如果日志初始化失败，会返回 `flexi_logger::FlexiLoggerError`.
#[allow(dead_code)]
pub fn init_flexi_logger(name: &str, log_directory: &str, level: &str) -> Result<LoggerHandle> {
    let log_directory = format!("{log_directory}/{name}");
    let file_spec = FileSpec::default()
        .directory(log_directory)
        .basename(name)
        .suppress_timestamp();

    let mut logger = Logger::try_with_str(level)?;
    logger = logger.log_to_file(file_spec);
    logger = logger.format(log_format);

    #[cfg(debug_assertions)]
    {
        logger = logger.duplicate_to_stdout(Duplicate::All);
    }
    #[cfg(not(debug_assertions))]
    {
        logger = logger.duplicate_to_stdout(Duplicate::Error);
    }

    logger = logger.write_mode(WriteMode::BufferAndFlushWith(64000, Duration::from_secs(5)));

    // 1GB = 1000000000 bytes
    // 10MB = 10000000 bytes
    // 50MB = 50000000 bytes
    // 100MB = 100000000 bytes
    logger = logger
        .rotate(
            Criterion::AgeOrSize(Age::Day, 10_000_000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .append();
    Ok(logger.start()?)
}

/// 自定义日志格式
fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] T[{}] {} [{}:{}] ",
        now.format("%Y-%m-%d %H:%M:%S%.6f"),
        thread::current().name().unwrap_or("<unnamed>"),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )?;

    // #[cfg(feature = "kv")]
    // write_key_value_pairs(w, record)?;

    write!(w, "{}", &record.args())
}

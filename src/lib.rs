extern crate log;
extern crate colored;
extern crate chrono;

use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};
use colored::*;

struct ColoredLogger;

impl log::Log for ColoredLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let time_string = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            match record.level() {
                LogLevel::Error => {
                    println!("[{}] {}", time_string.red(), record.args());
                }
                LogLevel::Warn => {
                    println!("[{}] {}", time_string.yellow(), record.args());
                }
                LogLevel::Info => {
                    println!("[{}] {}", time_string.blue(), record.args());
                }
                LogLevel::Debug => {
                    println!("[{}] {}", time_string.white(), record.args());
                }
                LogLevel::Trace => {
                    println!("[{}] {}", time_string.white(), record.args());
                }
            }
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(ColoredLogger)
    })
}

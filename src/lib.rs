extern crate log;
extern crate colored;

use log::{LogLevel, LogMetadata, LogRecord};
use colored::*;

struct ColoredLogger;

impl log::Log for ColoredLogger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            match record.level() {
                LogLevel::Error => {
                    println!("[{}] {}", "error".red(), record.args());
                }
                LogLevel::Warn => {
                    println!("[{}] {}", "warn".yellow(), record.args());
                }
                LogLevel::Info => {
                    println!("[{}] {}", "info".blue(), record.args());
                }
                LogLevel::Debug => {
                    println!("[{}] {}", "debug".white(), record.args());
                }
                LogLevel::Trace => {
                    println!("[{}] {}", "trace".white(), record.args());
                }
            }
        }
    }
}

extern crate log;
extern crate chrono;
extern crate term_painter;

use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};
use term_painter::{Color, ToStyle};

struct ColoredLogger;

#[cfg(not(target_os = "windows"))]
pub const DIMM_COLOR: Color = Color::Custom(243);
#[cfg(target_os = "windows")]
pub const DIMM_COLOR: Color = Color::White;

impl ColoredLogger {
    /// Filter some unreadable (on dark background) or nasty colors
    fn hashed_color(item: &str) -> Color {
        match item.bytes().fold(42u16, |c, x| c ^ x as u16) {
            c @ 0...1 => Color::Custom(c + 2),
            c @ 16...21 => Color::Custom(c + 6),
            c @ 52...55 | c @ 126...129 => Color::Custom(c + 4),
            c @ 163...165 | c @ 200...201 => Color::Custom(c + 3),
            c @ 207 => Color::Custom(c + 1),
            c @ 232...240 => Color::Custom(c + 9),
            c => Color::Custom(c),
        }
    }
}

impl log::Log for ColoredLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let time_string = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let level_color = match record.level() {
                LogLevel::Trace | LogLevel::Debug => DIMM_COLOR,
                LogLevel::Info => Color::Green,
                LogLevel::Warn => Color::Yellow,
                LogLevel::Error => Color::Red,
            };

            let module_color = Self::hashed_color(record.location().module_path());

            println!("{} {}: {}",
                     level_color.paint(time_string),
                     module_color.paint(record.location().module_path()),
                     record.args());
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
                        max_log_level.set(LogLevelFilter::Debug);
                        Box::new(ColoredLogger)
                    })
}

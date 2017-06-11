#[macro_use]
extern crate log;
extern crate colored_logger;

use log::LogLevel;

fn log() {
    trace!("this is a trace message");
    debug!("this is a debug message");
    info!("this is a info message");
    warn!("and a warning message");
    error!("something happened... error message");
}

mod a {
    pub fn log() {
        trace!("this is a trace message");
        debug!("this is a debug message");
        info!("this is a info message");
        warn!("and a warning message");
        error!("something happened... error message");
    }
}

mod b {
    pub fn log() {
        trace!("this is a trace message");
        debug!("this is a debug message");
        info!("this is a info message");
        warn!("and a warning message");
        error!("something happened... error message");
    }
}

mod c {
    pub fn log() {
        trace!("this is a trace message");
        debug!("this is a debug message");
        info!("this is a info message");
        warn!("and a warning message");
        error!("something happened... error message");
    }
}

fn main() {
    colored_logger::init(LogLevel::Trace).unwrap();

    log();
    a::log();
    b::log();
    c::log();
}

#[macro_use]
extern crate log;
extern crate colored_logger;

fn log() {
    info!("this is a info message");
    warn!("and a warning message");
    error!("something happened... error message");
}

mod a {
    pub fn log() {
        info!("this is a info message");
        warn!("and a warning message");
        error!("something happened... error message");
    }
}

mod b {
    pub fn log() {
        info!("this is a info message");
        warn!("and a warning message");
        error!("something happened... error message");
    }
}

mod c {
    pub fn log() {
        info!("this is a info message");
        warn!("and a warning message");
        error!("something happened... error message");
    }
}

fn main() {
    colored_logger::init().unwrap();

    log();
    a::log();
    b::log();
    c::log();
}

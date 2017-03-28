#[macro_use]
extern crate log;
extern crate colored_logger;

fn main() {
    colored_logger::init().unwrap();

    info!("this is info message");
    warn!("warning message");
    error!("something happened... error message");
}

# colored_logger

A simple colored logger for Rust. Implements
[`log`](https://github.com/rust-lang-nursery/log) interface.

## Usage

```rust
#[macro_use] extern crate log;
extern crate colored_logger;

pub fn main() {
    colored_logger::init().unwrap();

    info!("info message");
    warn!("warning message");
    error!("error message");
}
```

For detailed usage info check
[`log` crate docs](https://doc.rust-lang.org/log/log/index.html).

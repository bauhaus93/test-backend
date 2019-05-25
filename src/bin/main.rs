#[macro_use]
extern crate log;

extern crate test_backend;

use test_backend::utility::init_logger;

fn main() {
    init_logger();
    info!("Hello, world!");
}


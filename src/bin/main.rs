#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

use std::io::Write;
use env_logger::{ Builder, fmt::Formatter };
use log::Record;


fn main() {
    init_custom_logger();
    info!("Hello, world!");
}

fn init_custom_logger() {
    let format = |buf: &mut Formatter, record: &Record| {
        let time = chrono::Local::now();
        writeln!(buf, "[{} {:-5}] {}", time.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args()) 
    };
    Builder::from_default_env()
        .format(format)
        .init();
}

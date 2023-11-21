extern crate log;
extern crate env_logger;

use log::{info};

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::builder().format_timestamp_millis().init();

    info!("first message");
}
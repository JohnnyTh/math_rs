use std::env;
use log::info;
use env_logger::builder;

mod algorithms;
mod ops_nalgebra;
use crate::algorithms::LinearRegression;

fn main() {
    env::set_var("RUST_LOG", "info");
    builder().format_timestamp_millis().init();

    info!("first message");
}
#![cfg_attr(debug_assertions, allow(dead_code))]
use std::{thread, time::Duration};

use application::logging::start_logger;
use dotenv::dotenv;
use network::socket::init_socket_connection;

mod application;
mod game;
mod network;

pub mod utils;

fn main() {
    dotenv().ok();
    let _logger_handle = start_logger();

    init_socket_connection();

    loop {
        thread::sleep(Duration::from_secs(60))
    }
}

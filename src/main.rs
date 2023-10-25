#![cfg_attr(debug_assertions, allow(dead_code))]
use std::{thread, time::Duration};

use application::{database::init_database_connection, logging::start_logger};
use dotenvy::dotenv;
use network::socket::init_socket_connection;

mod application;
mod game;
mod network;

pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _logger_handle = start_logger();

    init_database_connection().await;
    init_socket_connection().await;

    loop {
        thread::sleep(Duration::from_secs(60))
    }
}

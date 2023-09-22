use std::{thread, time::Duration};

use dotenv::dotenv;
use network::init_connection;

mod game;
mod network;
pub mod utils;

fn main() {
    dotenv().ok();

    init_connection();

    loop {
        thread::sleep(Duration::from_secs(60))
    }
}

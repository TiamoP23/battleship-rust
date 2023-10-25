use rust_socketio::asynchronous::ClientBuilder;
use rust_socketio::Event;
use rust_socketio::TransportType;

use super::controller::handle_close;
use super::controller::handle_connect;
use super::controller::handle_data;
use super::controller::handle_error;

pub async fn init_socket_connection() {
    let gameserver = std::env::var("GAMESERVER").expect("GAMESERVER not set");

    let _socket = ClientBuilder::new(gameserver)
        .transport_type(TransportType::Websocket)
        .on(Event::Error, handle_error)
        .on(Event::Connect, handle_connect)
        .on(Event::Close, handle_close)
        .on(Event::Custom(String::from("data")), handle_data)
        .connect()
        .await
        .expect("Connection failed");
}

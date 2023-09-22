use std::time::Duration;

use rust_socketio::{ClientBuilder, Event, Payload, RawClient, TransportType};
use serde_json::json;

use crate::{game::handle_game_event, utils::deserialize_payload};

pub mod models;

pub fn init_connection() {
    let gameserver = std::env::var("GAMESERVER").expect("GAMESERVER not set");

    let _socket = ClientBuilder::new(gameserver)
        .transport_type(TransportType::Websocket)
        .on(Event::Error, handle_error)
        .on(Event::Connect, handle_connect)
        .on(Event::Close, handle_close)
        .on(Event::Custom(String::from("data")), handle_data)
        .connect()
        .expect("Connection failed");
}

fn handle_connect(_: Payload, client: RawClient, _: Option<i32>) {
    println!("Connected to server!");

    authenticate(client);
}

fn handle_close(_: Payload, _: RawClient, _: Option<i32>) {
    println!("Connection closed");
}

fn handle_error(payload: Payload, _: RawClient, _: Option<i32>) {
    println!("Error: {:#?}", payload);
}

fn handle_data(payload: Payload, client: RawClient, packet_id: Option<i32>) {
    let event = deserialize_payload(&payload).unwrap();

    let response = handle_game_event(event);

    if let Some(response) = response {
        client
            .emit_ack(packet_id, serde_json::to_string(&response).unwrap())
            .unwrap_or_else(|err| println!("Server unreachable: {}", err));
    }
}

fn authenticate(client: RawClient) {
    let secret = std::env::var("SECRET").expect("SECRET not set");

    client
        .emit_with_ack(
            "authenticate",
            json!(secret),
            Duration::from_secs(2),
            |payload, _, _| {
                let success = deserialize_payload::<(bool,)>(&payload).unwrap().0;

                if success {
                    println!("Authenticated successfully");
                } else {
                    println!("Authentication failed");
                }
            },
        )
        .expect("Server unreachable");
}

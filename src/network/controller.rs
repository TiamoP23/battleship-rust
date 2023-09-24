use log::{debug, error, info, warn};
use rust_socketio::{Payload, RawClient};

use crate::{
    game::controller::handle_game_event, network::handler::authenticate,
    utils::payload::deserialize_payload,
};

pub fn handle_connect(_: Payload, client: RawClient, _: Option<i32>) {
    info!("Connected to server!");

    authenticate(client);
}

pub fn handle_close(_: Payload, _: RawClient, _: Option<i32>) {
    warn!("Connection closed");
}

pub fn handle_error(payload: Payload, _: RawClient, _: Option<i32>) {
    error!("Error: {:#?}", payload);
}

pub fn handle_data(payload: Payload, client: RawClient, packet_id: Option<i32>) {
    match deserialize_payload(&payload) {
        Ok(event) => {
            if let Some(packet_id) = packet_id {
                debug!("Received event ({:#?}): {:#?}", packet_id, event);
            } else {
                debug!("Received event: {:#?}", event);
            }

            let response = handle_game_event(event);

            if let Some(response) = response {
                debug!(
                    "Sending response ({:#?}): {:#?}",
                    packet_id.expect("Packet ID not found"),
                    response
                );

                client
                    .emit_ack(packet_id, serde_json::to_string(&response).unwrap())
                    .unwrap_or_else(|err| error!("Server unreachable: {}", err));
            }
        }
        Err(err) => {
            error!("Invalid payload: {:#?}", payload);
            error!("{:#?}", err);
        }
    }
}

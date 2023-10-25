use futures_util::{future::BoxFuture, FutureExt};
use log::{debug, error, info, warn};
use rust_socketio::{asynchronous::Client, Payload};

use crate::{
    game::controller::handle_game_event, network::handler::authenticate,
    utils::payload::deserialize_payload,
};

pub fn handle_connect(_: Payload, client: Client, _: Option<i32>) -> BoxFuture<'static, ()> {
    async move {
        info!("Connected to server!");

        authenticate(client).await;
    }
    .boxed()
}

pub fn handle_close(_: Payload, _: Client, _: Option<i32>) -> BoxFuture<'static, ()> {
    async move {
        warn!("Connection closed");
    }
    .boxed()
}

pub fn handle_error(payload: Payload, _: Client, _: Option<i32>) -> BoxFuture<'static, ()> {
    async move {
        error!("Error: {:#?}", payload);
    }
    .boxed()
}

pub fn handle_data(
    payload: Payload,
    client: Client,
    packet_id: Option<i32>,
) -> BoxFuture<'static, ()> {
    async move {
        match deserialize_payload(&payload) {
            Ok(event) => {
                if let Some(packet_id) = packet_id {
                    debug!("Received event ({:#?}): {:#?}", packet_id, event);
                } else {
                    debug!("Received event: {:#?}", event);
                }

                let response = handle_game_event(event).await;

                if let Some(response) = response {
                    debug!(
                        "Sending response ({:#?}): {:#?}",
                        packet_id.expect("Packet ID not found"),
                        response
                    );

                    client
                        .emit_ack(packet_id, serde_json::to_string(&response).unwrap())
                        .await
                        .unwrap_or_else(|err| error!("Server unreachable: {}", err));
                }
            }
            Err(err) => {
                error!("Invalid payload: {:#?}", payload);
                error!("{:#?}", err);
            }
        }
    }
    .boxed()
}

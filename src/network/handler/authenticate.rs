use std::time::Duration;

use log::{error, info};
use rust_socketio::RawClient;
use serde_json::json;

use crate::utils::payload::deserialize_payload;

pub fn authenticate(client: RawClient) {
    let secret = std::env::var("SECRET").expect("SECRET not set");

    client
        .emit_with_ack(
            "authenticate",
            json!(secret),
            Duration::from_secs(2),
            |payload, _, _| {
                let success = deserialize_payload::<(bool,)>(&payload).unwrap().0;

                if success {
                    info!("Authenticated successfully");
                } else {
                    error!("Authentication failed");
                }
            },
        )
        .expect("Server unreachable");
}

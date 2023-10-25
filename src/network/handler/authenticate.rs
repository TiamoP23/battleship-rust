use std::time::Duration;

use futures_util::FutureExt;
use log::{error, info};
use rust_socketio::asynchronous::Client;
use serde_json::json;

use crate::utils::payload::deserialize_payload;

pub async fn authenticate(client: Client) {
    let secret = std::env::var("SECRET").expect("SECRET not set");

    let callback = |payload, _, _| {
        async move {
            let success = deserialize_payload::<(bool,)>(&payload).unwrap().0;

            if success {
                info!("Authenticated successfully");
            } else {
                error!("Authentication failed");
            }
        }
        .boxed()
    };

    client
        .emit_with_ack(
            "authenticate",
            json!(secret),
            Duration::from_secs(2),
            callback,
        )
        .await
        .expect("Server unreachable");
}

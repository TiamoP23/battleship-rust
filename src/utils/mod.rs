use serde::Deserialize;

use rust_socketio::Payload;

pub fn deserialize_payload<'a, T>(payload: &'a Payload) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    let payload = match payload {
        Payload::String(data) => data,
        _ => Err(String::from("Payload is not a string"))?,
    };

    let event = match serde_json::from_str::<T>(&payload) {
        Ok(event) => event,
        Err(err) => Err(format!("Parsing Error: {:#?}", err))?,
    };

    Ok(event)
}

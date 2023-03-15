use super::model::ShutterStatus;
use crate::{listen_command, mqtt::client::MqttMessage, send_command};

fn map(message: MqttMessage, channel: &str) -> Option<ShutterStatus> {
    let value = serde_json::from_str::<serde_json::Value>(&message.payload)
        .ok()?
        .as_object_mut()
        .and_then(move |map| map.remove(&format!("Shutter{channel}")))?;

    serde_json::from_value(value).ok()
}

send_command!(
    set_shutter_position,
    "cmnd/{device_name}/shutterposition{channel}"
);

listen_command!(
    get_shutter_position,
    "stat/{device_name}/RESULT",
    ShutterStatus,
    map
);

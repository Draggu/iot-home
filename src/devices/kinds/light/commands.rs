use super::model::LightStatus;
use crate::{listen_command, mqtt::client::MqttMessage, send_command};

fn map(message: MqttMessage, channel: &str) -> Option<LightStatus> {
    let value = serde_json::Value::from(message.payload)
        .as_object_mut()
        .and_then(|map| map.remove(&format!("Channel{channel}")))?;

    serde_json::from_value(value).ok()
}

send_command!(set_brightness, "cmnd/{device_name}/channel{channel}");

listen_command!(
    get_brightness,
    "stat/{device_name}/RESULT",
    LightStatus,
    map
);

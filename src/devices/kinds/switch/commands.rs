use crate::{listen_command, mqtt::client::MqttMessage, send_command};

fn map(message: MqttMessage, channel: &str) -> Option<bool> {
    let serde_value = serde_json::Value::from(message.payload)
        .as_object_mut()
        .and_then(|map| map.remove(&format!("POWER{channel}")))?;

    let value = serde_value.as_str()?;

    match value {
        "ON" => Some(true),
        "OFF" => Some(false),
        _ => None,
    }
}

send_command!(set_switch_status, "cmnd/{device_name}/power{channel}");
listen_command!(get_switch_status, "stat/{device_name}/RESULT", bool, map);

use super::model::{Sensor, SensorMqtt};
use crate::{listen_command, mqtt::client::MqttMessage};

fn map(message: MqttMessage, _: &str) -> Option<Sensor> {
    serde_json::from_str::<SensorMqtt>(&message.payload)
        .ok()
        .map(Into::into)
}

listen_command!(get_voltage, "tele/{device_name}/SENSOR", Sensor, map);

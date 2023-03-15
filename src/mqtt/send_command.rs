#[macro_export]
macro_rules! send_command {
    ($name:ident,$pattern:tt) => {
        pub async fn $name<'a>(
            mqtt: &crate::mqtt::client::MqttClient,
            devices: &crate::devices::registry::service::DeviceService,
            device_id: uuid::Uuid,
            payload: impl Into<String>,
        ) -> Result<crate::gql::Void, &'a str> {
            let device = devices.get_by_id(device_id).await?;

            let channel = device.channel_as_string();
            let device_name = device.device_name;

            mqtt.publish(
                // see: https://stackoverflow.com/a/41911995
                format!(
                    concat!($pattern, "{channel:.0}{device_name:.0}"),
                    channel = channel,
                    device_name = device_name
                ),
                payload,
            )
            .await?;

            Ok(crate::gql::Void)
        }
    };
}

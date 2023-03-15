#[macro_export]
macro_rules! listen_command {
    ($name:ident,$pattern:tt,$return:ty,$parse:path) => {
        pub async fn $name<'a>(
            mqtt: &crate::mqtt::client::MqttClient,
            device: &crate::devices::registry::model::Model,
        ) -> Result<impl futures::Stream<Item = $return>, &'a str> {
            use futures::StreamExt;

            let channel = device.channel_as_string();

            Ok(mqtt
                .subscribe(format!(
                    concat!($pattern, "{channel:.0}{device_name:.0}"),
                    channel = channel,
                    device_name = device.device_name
                ))
                .await?
                .filter_map(move |message| {
                    let result = $parse(message, &channel);

                    async move { result }
                }))
        }
    };
}

use crate::{
    device_registry::device::{DeviceKind, DevicesManager},
    mqtt::client::MqttClient,
};
use async_graphql::{Context, Object, Subscription, ID};
use futures::{Stream, StreamExt};

use super::model::LightStatus;

#[derive(Default)]
pub struct DeviceLightMutation {}
#[derive(Default)]
pub struct DeviceLightSubscription {}

#[Object]
impl DeviceLightMutation {
    async fn light_device_brightness(
        &self,
        context: &Context<'_>,
        display_name: ID,
        #[graphql(validator(maximum = 100))] brightness: u8,
    ) -> Result<bool, &str> {
        let device = context
            .data_unchecked::<DevicesManager>()
            .get_by_display_name(DeviceKind::Light, &*display_name)
            .await?;

        let channel = device.channel_as_string();
        let device_name = device.device_name;

        context
            .data_unchecked::<MqttClient>()
            .publish(
                format!("cmnd/{device_name}/channel{channel}"),
                brightness.to_string(),
            )
            .await?;

        Ok(true)
    }
}

#[Subscription]
impl DeviceLightSubscription {
    async fn light_device_status<'a>(
        &'a self,
        context: &Context<'_>,
        display_name: ID,
    ) -> Result<impl Stream<Item = LightStatus>, &'a str> {
        let device = context
            .data_unchecked::<DevicesManager>()
            .get_by_display_name(DeviceKind::Light, &*display_name)
            .await?;

        let channel = device.channel_as_string();
        let device_name = device.device_name;

        Ok(context
            .data_unchecked::<MqttClient>()
            .subscribe(format!("stat/{device_name}/RESULT"))
            .await?
            .filter_map(move |message| {
                let key = format!("Channel{channel}");

                let maybe_value = serde_json::Value::from(message.payload)
                    .as_object_mut()
                    .and_then(move |map| map.remove(&key));

                async move { serde_json::from_value(maybe_value?).ok() }
            }))
    }
}

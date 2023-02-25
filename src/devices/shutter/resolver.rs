use crate::{
    device_registry::device::{DeviceKind, DevicesManager},
    mqtt::client::MqttClient,
};
use async_graphql::{Context, Object, Subscription, ID};
use futures::{Stream, StreamExt};

use super::model::ShutterStatus;

#[derive(Default)]
pub struct DeviceShutterMutation {}
#[derive(Default)]
pub struct DeviceShutterSubscription {}

#[Object]
impl DeviceShutterMutation {
    async fn shutter_device(
        &self,
        context: &Context<'_>,
        display_name: ID,
        #[graphql(validator(maximum = 100))] position: u8,
    ) -> Result<bool, &str> {
        let device = context
            .data_unchecked::<DevicesManager>()
            .get_by_display_name(DeviceKind::Shutter, &*display_name)
            .await?;

        let channel = device.channel_as_string();
        let device_name = device.device_name;

        context
            .data_unchecked::<MqttClient>()
            .publish(
                format!("cmnd/{device_name}/shutterposition{channel}"),
                position.to_string(),
            )
            .await?;

        Ok(true)
    }
}

#[Subscription]
impl DeviceShutterSubscription {
    async fn shutter_device_status(
        &self,
        context: &Context<'_>,
        display_name: ID,
    ) -> Result<impl Stream<Item = ShutterStatus>, &str> {
        let device = context
            .data_unchecked::<DevicesManager>()
            .get_by_display_name(DeviceKind::Shutter, &*display_name)
            .await?;

        let channel = device.channel_as_string();
        let device_name = device.device_name;

        Ok(context
            .data_unchecked::<MqttClient>()
            .subscribe(format!("stat/{device_name}/RESULT"))
            .await?
            .filter_map(move |message| {
                let key = format!("Shutter{channel}");

                let maybe_value = serde_json::Value::from(message.payload)
                    .as_object_mut()
                    .and_then(move |map| map.remove(&key));

                async move { serde_json::from_value(maybe_value?).ok() }
            }))
    }
}

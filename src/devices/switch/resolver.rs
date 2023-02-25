use crate::{
    device_registry::device::{DeviceKind, DevicesManager},
    mqtt::client::MqttClient,
};
use async_graphql::{Context, Enum, Object, Subscription, ID};
use futures::{Stream, StreamExt};

#[derive(Default)]
pub struct DeviceSwitchMutation {}
#[derive(Default)]
pub struct DeviceSwitchSubscription {}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
enum SwitchStatus {
    On,
    Off,
    Toggle,
}

impl SwitchStatus {
    fn as_payload(&self) -> &'static str {
        match self {
            Self::On => "ON",
            Self::Off => "OFF",
            Self::Toggle => "TOGGLE",
        }
    }
}

#[Object]
impl DeviceSwitchMutation {
    async fn switch_device(
        &self,
        context: &Context<'_>,
        display_name: ID,
        switch: SwitchStatus,
    ) -> Result<bool, &str> {
        self.send_command(context, display_name, Some(switch)).await
    }

    async fn switch_device_refresh(
        &self,
        context: &Context<'_>,
        display_name: ID,
    ) -> Result<bool, &str> {
        self.send_command(context, display_name, None).await
    }

    #[graphql(skip)]
    async fn send_command(
        &self,
        context: &Context<'_>,
        display_name: ID,
        switch: Option<SwitchStatus>,
    ) -> Result<bool, &str> {
        let device = context
            .data_unchecked::<DevicesManager>()
            .get_by_display_name(DeviceKind::Switch, &*display_name)
            .await?;

        let channel = device.channel_as_string();
        let device_name = device.device_name;
        let topic = format!("cmnd/{device_name}/power{channel}");

        context
            .data_unchecked::<MqttClient>()
            .publish(
                topic,
                switch.map(|status| status.as_payload()).unwrap_or(""),
            )
            .await?;

        Ok(true)
    }
}

#[Subscription]
impl DeviceSwitchSubscription {
    async fn switch_device_status(
        &self,
        context: &Context<'_>,
        display_name: ID,
    ) -> Result<impl Stream<Item = bool>, &str> {
        let device = context
            .data_unchecked::<DevicesManager>()
            .get_by_display_name(DeviceKind::Switch, &*display_name)
            .await?;

        let channel = device.channel_as_string();
        let device_name = device.device_name;

        Ok(context
            .data_unchecked::<crate::mqtt::client::MqttClient>()
            .subscribe(format!("stat/{device_name}/RESULT"))
            .await?
            .filter_map(move |message| {
                let key = format!("POWER{channel}");

                let maybe_value = serde_json::Value::from(message.payload)
                    .as_object_mut()
                    .and_then(move |map| map.remove(&key));

                async move {
                    let value = maybe_value?;

                    let value = value.as_str()?;

                    match value {
                        "ON" => Some(true),
                        "OFF" => Some(false),
                        _ => None,
                    }
                }
            }))
    }
}

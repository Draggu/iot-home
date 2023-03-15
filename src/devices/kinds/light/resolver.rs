use super::{
    commands::{self, get_brightness},
    model::LightStatus,
};
use crate::{devices::registry::service::DeviceService, gql::Void, mqtt::client::MqttClient};
use async_graphql::{Context, Object, Subscription};
use futures::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct DeviceLightMutation {}
#[derive(Default)]
pub struct DeviceLightSubscription {}

#[Object]
impl DeviceLightMutation {
    async fn light_device(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(validator(maximum = 100))] brightness: u8,
    ) -> Result<Void, &str> {
        commands::set_brightness(
            context.data_unchecked::<MqttClient>(),
            context.data_unchecked::<DeviceService>(),
            id,
            brightness.to_string(),
        )
        .await
    }

    async fn light_device_refresh(&self, context: &Context<'_>, id: Uuid) -> Result<Void, &str> {
        commands::set_brightness(
            context.data_unchecked::<MqttClient>(),
            context.data_unchecked::<DeviceService>(),
            id,
            String::new(),
        )
        .await
    }
}

#[Subscription]
impl DeviceLightSubscription {
    async fn light_device_status<'a>(
        &'a self,
        context: &Context<'_>,
        id: Uuid,
    ) -> Result<impl Stream<Item = LightStatus>, &'a str> {
        get_brightness(
            context.data_unchecked::<MqttClient>(),
            &context
                .data_unchecked::<DeviceService>()
                .get_by_id(id)
                .await?,
        )
        .await
    }
}

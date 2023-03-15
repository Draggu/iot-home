use super::{
    commands::{self, get_shutter_position},
    model::ShutterStatus,
};
use crate::{devices::registry::service::DeviceService, gql::Void, mqtt::client::MqttClient};
use async_graphql::{Context, Object, Subscription};
use futures::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct DeviceShutterMutation;
#[derive(Default)]
pub struct DeviceShutterSubscription;

#[Object]
impl DeviceShutterMutation {
    // exists also STOP command but seems useless

    async fn shutter_device(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(validator(maximum = 100))] position: u8,
    ) -> Result<Void, &str> {
        commands::set_shutter_position(
            context.data_unchecked::<MqttClient>(),
            context.data_unchecked::<DeviceService>(),
            id,
            position.to_string(),
        )
        .await
    }

    async fn shutter_device_refresh(&self, context: &Context<'_>, id: Uuid) -> Result<Void, &str> {
        commands::set_shutter_position(
            context.data_unchecked::<MqttClient>(),
            context.data_unchecked::<DeviceService>(),
            id,
            String::new(),
        )
        .await
    }
}

#[Subscription]
impl DeviceShutterSubscription {
    async fn shutter_device_status(
        &self,
        context: &Context<'_>,
        id: Uuid,
    ) -> Result<impl Stream<Item = ShutterStatus>, &str> {
        get_shutter_position(
            context.data_unchecked::<MqttClient>(),
            &context
                .data_unchecked::<DeviceService>()
                .get_by_id(id)
                .await?,
        )
        .await
    }
}

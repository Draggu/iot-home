use super::{
    commands::{self, get_switch_status},
    model::SwitchStatus,
};
use crate::{devices::registry::service::DeviceService, gql::Void, mqtt::client::MqttClient};
use async_graphql::{Context, Object, Subscription};
use futures::Stream;
use uuid::Uuid;

#[derive(Default)]
pub struct DeviceSwitchMutation;
#[derive(Default)]
pub struct DeviceSwitchSubscription;

#[Object]
impl DeviceSwitchMutation {
    async fn switch_device(
        &self,
        context: &Context<'_>,
        id: Uuid,
        switch: SwitchStatus,
    ) -> Result<Void, &str> {
        commands::set_switch_status(
            context.data_unchecked::<MqttClient>(),
            context.data_unchecked::<DeviceService>(),
            id,
            switch.as_payload(),
        )
        .await
    }

    async fn switch_device_refresh(&self, context: &Context<'_>, id: Uuid) -> Result<Void, &str> {
        commands::set_switch_status(
            context.data_unchecked::<MqttClient>(),
            context.data_unchecked::<DeviceService>(),
            id,
            String::new(),
        )
        .await
    }
}

#[Subscription]
impl DeviceSwitchSubscription {
    async fn switch_device_status(
        &self,
        context: &Context<'_>,
        id: Uuid,
    ) -> Result<impl Stream<Item = bool>, &str> {
        get_switch_status(
            context.data_unchecked::<MqttClient>(),
            &context
                .data_unchecked::<DeviceService>()
                .get_by_id(id)
                .await?,
        )
        .await
    }
}

use super::{commands::get_voltage, model::Sensor, service::VoltageService};
use crate::{devices::registry::model::Model as Device, mqtt::client::MqttClient};
use async_graphql::{Context, Object, Subscription};
use futures::Stream;

#[derive(Default)]
pub struct VoltageQuery;
#[derive(Default)]
pub struct VoltageSubscription;

#[Object]
impl VoltageQuery {
    async fn last_day_voltage(
        &self,
        context: &Context<'_>,
        device_name: String,
    ) -> Result<Vec<Sensor>, &str> {
        Ok(context
            .data_unchecked::<VoltageService>()
            .get_last_day(device_name)
            .await?)
    }
}

#[Subscription]
impl VoltageSubscription {
    async fn voltage_report(
        &self,
        context: &Context<'_>,
        device_name: String,
    ) -> Result<impl Stream<Item = Sensor>, &str> {
        get_voltage(
            &context.data_unchecked::<MqttClient>(),
            &Device {
                device_name,
                //get_voltage uses `device_name` as ident but require `Device` so create fake one
                ..Default::default()
            },
        )
        .await
    }
}

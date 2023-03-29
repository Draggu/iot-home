use super::model::Model as DeviceModel;
use crate::{devices::registry::service::DeviceService, gql::Void};
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct DevicesQuery;
#[derive(Default)]
pub struct DevicesMutation;

#[Object]
impl DevicesQuery {
    async fn all_devices(&self, context: &Context<'_>) -> Result<Vec<DeviceModel>, &str> {
        Ok(context.data_unchecked::<DeviceService>().to_gql().await?)
    }
}

#[Object]
impl DevicesMutation {
    async fn add_device(
        &self,
        context: &Context<'_>,
        device: DeviceModel,
    ) -> Result<DeviceModel, &str> {
        Ok(context
            .data_unchecked::<DeviceService>()
            .add(device)
            .await?)
    }

    async fn remove_device(&self, context: &Context<'_>, display_name: Uuid) -> Result<Void, &str> {
        Ok(context
            .data_unchecked::<DeviceService>()
            .remove(display_name)
            .await?
            .into())
    }
}

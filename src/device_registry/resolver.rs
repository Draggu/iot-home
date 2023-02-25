use crate::device_registry::device::{AllDevices, Device, DeviceKind, DevicesManager};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct DevicesQuery {}
#[derive(Default)]
pub struct DevicesMutation {}

#[Object]
impl DevicesQuery {
    async fn devices(&self, context: &Context<'_>) -> AllDevices {
        context.data_unchecked::<DevicesManager>().to_gql().await
    }
}

#[Object]
impl DevicesMutation {
    async fn add_device(&self, context: &Context<'_>, kind: DeviceKind, device: Device) -> bool {
        context
            .data_unchecked::<DevicesManager>()
            .add(kind, device)
            .await
    }

    async fn remove_device(
        &self,
        context: &Context<'_>,
        kind: DeviceKind,
        display_name: String,
    ) -> bool {
        context
            .data_unchecked::<DevicesManager>()
            .remove(kind, display_name)
            .await
    }
}

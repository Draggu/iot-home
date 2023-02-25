use crate::{
    device_registry::resolver::{DevicesMutation, DevicesQuery},
    devices::{
        light::resolver::{DeviceLightMutation, DeviceLightSubscription},
        shutter::resolver::{DeviceShutterMutation, DeviceShutterSubscription},
        switch::resolver::{DeviceSwitchMutation, DeviceSwitchSubscription},
    },
};
use async_graphql::{MergedObject, MergedSubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;

pub async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, Subscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(MergedObject, Default)]
pub struct Query(DevicesQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(
    DevicesMutation,
    DeviceSwitchMutation,
    DeviceLightMutation,
    DeviceShutterMutation,
);

#[derive(MergedSubscription, Default)]
pub struct Subscription(
    DeviceSwitchSubscription,
    DeviceLightSubscription,
    DeviceShutterSubscription,
);

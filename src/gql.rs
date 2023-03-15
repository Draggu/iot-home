use crate::devices::{
    kinds::{
        light::resolver::{DeviceLightMutation, DeviceLightSubscription},
        shutter::resolver::{DeviceShutterMutation, DeviceShutterSubscription},
        switch::resolver::{DeviceSwitchMutation, DeviceSwitchSubscription},
    },
    registry::resolver::{DevicesMutation, DevicesQuery},
    specials::voltage::resolver::{VoltageQuery, VoltageSubscription},
};
use async_graphql::{scalar, MergedObject, MergedSubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Void;

scalar!(Void);

impl From<()> for Void {
    fn from(_: ()) -> Self {
        Self
    }
}

pub async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, Subscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(MergedObject, Default)]
pub struct Query(DevicesQuery, VoltageQuery);

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
    VoltageSubscription,
);

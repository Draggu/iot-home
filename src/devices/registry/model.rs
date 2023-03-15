use async_graphql::{Enum, InputObject, SimpleObject};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, DeriveEntityModel, SimpleObject, InputObject,
)]
#[sea_orm(table_name = "device")]
#[graphql(input_name = "DeviceInput", name = "Device")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[graphql(skip_input)]
    pub id: Uuid,
    pub display_name: String,
    pub device_name: String,
    pub channel: Option<u8>,
    pub kind: DeviceKind,
    pub is_reporting_voltage: bool,
}

impl Model {
    pub fn channel_as_string(&self) -> String {
        self.channel.map_or(String::new(), |c| c.to_string())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    Enum,
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum DeviceKind {
    #[default] // does not matter
    Switch = 0,
    Shutter = 1,
    Light = 2,
}

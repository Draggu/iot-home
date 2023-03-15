use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

#[derive(Serialize, Deserialize, Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "voltage")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    device_name: String,
    voltage: u16,
    timestamp: OffsetDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize)]
pub struct SensorMqtt {
    #[serde(rename = "Time", with = "iso8601")]
    time: OffsetDateTime,

    #[serde(rename = "ENERGY")]
    energy: SensorMqttEnergy,
}

#[derive(Deserialize)]
pub struct SensorMqttEnergy {
    #[serde(rename = "Voltage")]
    voltage: u16,
}

#[derive(Serialize, SimpleObject)]
#[graphql(name = "VoltageReportUnit")]
pub struct Sensor {
    pub timestamp: OffsetDateTime,
    pub voltage: u16,
}

impl From<SensorMqtt> for Sensor {
    fn from(value: SensorMqtt) -> Self {
        Self {
            timestamp: value.time,
            voltage: value.energy.voltage,
        }
    }
}

impl From<Model> for Sensor {
    fn from(value: Model) -> Self {
        Self {
            timestamp: value.timestamp,
            voltage: value.voltage,
        }
    }
}

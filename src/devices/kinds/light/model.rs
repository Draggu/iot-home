use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct LightStatus {
    #[serde(rename = "Power")]
    power: bool,

    #[serde(rename = "Light")]
    brightness: u8,
}

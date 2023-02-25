use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct ShutterStatus {
    #[serde(rename = "Position")]
    position: u8,

    #[serde(rename = "Direction")]
    direction: u8,

    #[serde(rename = "Target")]
    target: u8,
}

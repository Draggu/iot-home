use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct ShutterStatus {
    #[serde(rename = "Position")]
    position: u8,

    #[serde(rename = "Direction")]
    direction: ShutterDirection,

    #[serde(rename = "Target")]
    target: u8,
}

#[derive(
    Enum, Copy, Clone, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr, PartialEq, Debug,
)]
#[repr(i8)]
enum ShutterDirection {
    Closing = 1,
    Stop = 0,
    Opening = -1,
}

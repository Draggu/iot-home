use async_graphql::Enum;

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
pub enum SwitchStatus {
    On,
    Off,
    Toggle,
}

impl SwitchStatus {
    pub fn as_payload(&self) -> &'static str {
        match self {
            Self::On => "ON",
            Self::Off => "OFF",
            Self::Toggle => "TOGGLE",
        }
    }
}

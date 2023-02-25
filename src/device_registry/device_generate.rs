#[macro_export]
macro_rules! devices {
    ($($key:ident),*) => {
        paste::paste! {
            #[derive(
                serde::Serialize,
                serde::Deserialize,
                async_graphql::SimpleObject,
                async_graphql::InputObject,
                Default,
                Clone,
                PartialEq,
                Eq,
                Hash,
            )]
            #[graphql(input_name = "DeviceInput")]
            pub struct Device {
                pub device_name: String,
                pub display_name: async_graphql::ID,
                pub channel: Option<u8>,
            }

            impl Device {
                pub fn channel_as_string(&self) -> String {
                    self.channel.map_or(String::new(), |c| c.to_string())
                }
            }

            #[derive(async_graphql::SimpleObject, Default, Clone)]
            pub struct AllDevices {
                $(
                    $key: Vec<Device>,
                )*
            }

            impl From<AllDevicesInner> for AllDevices {
                fn from(from: AllDevicesInner) -> Self {
                    Self {
                        $(
                            $key: from.$key.values().into_iter().map(Clone::clone).collect(),
                        )*
                    }
                }
            }

            #[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
            pub struct AllDevicesInner {
                $(
                    $key: std::collections::HashMap<String, Device>,
                )*
            }

            impl AllDevicesInner {
                fn get_mut_by_key(&mut self, key: DeviceKind) -> &mut std::collections::HashMap<String, Device> {
                    match key {
                        $(
                            DeviceKind::[<$key:camel>] => &mut self.$key,
                        )*
                    }
                }
            }

            #[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
            pub enum DeviceKind{
                $(
                    [<$key:camel>],
                )*
            }
        }
    }
}

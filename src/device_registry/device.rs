use crate::devices;
use futures::lock::{Mutex, MutexGuard};
use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

devices! {
    switch,
    shutter,
    light
}

pub struct DevicesManager {
    inner: Mutex<AllDevicesInner>,
    file: PathBuf,
}

pub struct DeviceNotFound;

impl DeviceNotFound {
    fn new() -> Self {
        Self {}
    }
}

impl From<DeviceNotFound> for &str {
    fn from(_: DeviceNotFound) -> &'static str {
        "there is no such a name registered"
    }
}

impl DevicesManager {
    #[inline]
    pub async fn to_gql(&self) -> AllDevices {
        self.inner.lock().await.clone().into()
    }

    #[inline]
    pub async fn get_by_display_name(
        &self,
        kind: DeviceKind,
        display_name: impl AsRef<str>,
    ) -> Result<Device, DeviceNotFound> {
        self.inner
            .lock()
            .await
            .get_mut_by_key(kind)
            .get(display_name.as_ref())
            .map(Clone::clone)
            .ok_or_else(DeviceNotFound::new)
    }

    #[inline]
    pub async fn add(&self, kind: DeviceKind, device: Device) -> bool {
        let mut guard = self.inner.lock().await;

        let is_changed = guard
            .get_mut_by_key(kind)
            .insert((*device.display_name).clone(), device)
            .is_none();

        if is_changed {
            self.save(&guard).await; //TODO
        }

        is_changed
    }

    #[inline]
    pub async fn remove(&self, kind: DeviceKind, display_name: String) -> bool {
        let mut guard = self.inner.lock().await;

        let is_changed = guard.get_mut_by_key(kind).remove(&display_name).is_none();

        if is_changed {
            self.save(&guard).await; //TODO
        }

        is_changed
    }

    #[inline]
    pub fn load(path: String) -> Self {
        let path = PathBuf::from(path);

        Self {
            inner: Mutex::new(
                serde_json::from_str(
                    read_to_string(path.as_path())
                        .expect("problem during reading of devices file")
                        .as_str(),
                )
                .expect("invalid devices file"),
            ),
            file: path,
        }
    }

    #[inline]
    async fn save(&self, guard: &MutexGuard<'_, AllDevicesInner>) -> bool {
        write(
            self.file.as_path(),
            serde_json::to_string(&**guard).unwrap(),
        )
        .is_ok()
    }
}

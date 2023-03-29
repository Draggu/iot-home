use super::model::{
    ActiveModel as ActiveDeviceModel, Column as DeviceColumn, Entity as DeviceEntity,
    Model as DeviceModel,
};
use crate::db::DbErrWrapper;
use futures::{Stream, StreamExt};
use sea_orm::{entity::*, query::*, ColumnTrait, DatabaseConnection, DbErr, EntityTrait};
use tokio::sync::broadcast::{self, Sender};
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

pub enum DeviceErr {
    NotFound,
    DbErr(DbErrWrapper),
}

impl From<DeviceErr> for &str {
    fn from(device_err: DeviceErr) -> &'static str {
        match device_err {
            DeviceErr::NotFound => "there is no such a name registered",
            DeviceErr::DbErr(err) => err.into(),
        }
    }
}

impl From<DbErr> for DeviceErr {
    fn from(err: DbErr) -> DeviceErr {
        DeviceErr::DbErr(DbErrWrapper::new(err))
    }
}

#[derive(Clone)]
pub struct DeviceService {
    db: DatabaseConnection,
    device_change_tx: Sender<()>,
}

impl DeviceService {
    #[inline]
    pub fn on_change(&self) -> impl Stream<Item = ()> {
        BroadcastStream::new(self.device_change_tx.subscribe()).map(Result::unwrap)
    }

    #[inline]
    fn on_change_notify(&self) {
        self.device_change_tx.send(()).ok();
    }

    #[inline]
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            device_change_tx: broadcast::channel(10).0,
        }
    }

    #[inline]
    pub async fn to_gql(&self) -> Result<Vec<DeviceModel>, DbErrWrapper> {
        Ok(DeviceEntity::find().all(&self.db).await?)
    }

    #[inline]
    pub async fn get_by_id(&self, id: Uuid) -> Result<DeviceModel, DeviceErr> {
        DeviceEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DeviceErr::NotFound)
    }

    #[inline]
    pub async fn get_by_voltage_reporting(
        &self,
        is_reporting: bool,
    ) -> Result<Vec<DeviceModel>, DbErrWrapper> {
        Ok(DeviceEntity::find()
            .filter(DeviceColumn::IsReportingVoltage.eq(is_reporting))
            .all(&self.db)
            .await?)
    }

    #[inline]
    pub async fn add(&self, device: DeviceModel) -> Result<DeviceModel, DbErrWrapper> {
        let result = DeviceEntity::insert(ActiveDeviceModel {
            id: Set(Uuid::new_v4()),
            channel: Set(device.channel),
            device_name: Set(device.device_name),
            display_name: Set(device.display_name),
            kind: Set(device.kind),
            is_reporting_voltage: Set(device.is_reporting_voltage),
        })
        .exec_with_returning(&self.db)
        .await?;

        self.on_change_notify();

        Ok(result)
    }

    #[inline]
    pub async fn remove(&self, id: Uuid) -> Result<(), DeviceErr> {
        let r = DeviceEntity::delete_by_id(id).exec(&self.db).await?;

        if r.rows_affected == 0 {
            Err(DeviceErr::NotFound)
        } else {
            self.on_change_notify();

            Ok(())
        }
    }
}

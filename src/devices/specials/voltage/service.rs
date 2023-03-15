use super::{
    commands::get_voltage,
    model::{ActiveModel, Column, Entity, Sensor},
};
use crate::{
    db::DbErrWrapper, devices::registry::service::DeviceService, mqtt::client::MqttClient,
};
use futures::{FutureExt, StreamExt};
use sea_orm::{entity::*, sea_query::Expr, DatabaseConnection, EntityTrait, QueryFilter};
use stream_cancel::{StreamExt as Cancel, Tripwire};

pub struct VoltageService {
    db: DatabaseConnection,
}

impl VoltageService {
    pub fn new(db: DatabaseConnection, devices: DeviceService, mqtt: MqttClient) -> Self {
        let (mut trigger, tripwire) = Tripwire::new();

        tokio::spawn(Self::setup_and_start_recording(
            db.clone(),
            devices.clone(),
            mqtt.clone(),
            tripwire,
        ));

        let result = Self { db: db.clone() };

        tokio::spawn(async move {
            while let Some(()) = devices.on_change().next().await {
                let (new_trigger, tripwire) = Tripwire::new();

                trigger.cancel();

                trigger = new_trigger;

                Self::setup_and_start_recording(
                    db.clone(),
                    devices.clone(),
                    mqtt.clone(),
                    tripwire,
                )
                .await;
            }
        });

        result
    }

    pub async fn get_last_day<'a>(&self, device_name: String) -> Result<Vec<Sensor>, DbErrWrapper> {
        let results = Entity::find()
            .filter(Column::DeviceName.eq(device_name))
            .filter(Expr::col(Column::Timestamp).gte(Expr::cust("DATE('now','-1 day')")))
            .all(&self.db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(results)
    }

    async fn setup_and_start_recording(
        db: DatabaseConnection,
        devices: DeviceService,
        mqtt: MqttClient,
        tripwire: Tripwire,
    ) -> () {
        for device in devices
            .get_by_voltage_reporting(true)
            .await
            .expect("could not load devices to listen on")
            .into_iter()
        {
            let mqtt = mqtt.clone();
            let db = db.clone();
            let tripwire = tripwire.clone();

            tokio::spawn(async move {
                get_voltage(&mqtt, &device)
                    .await
                    .unwrap()
                    .take_until_if(tripwire)
                    .for_each_concurrent(None, |sensor| {
                        Entity::insert(ActiveModel {
                            id: ActiveValue::NotSet,
                            device_name: Set(device.device_name.clone()),
                            timestamp: Set(sensor.timestamp.clone()),
                            voltage: Set(sensor.voltage.clone()),
                        })
                        .exec_without_returning(&db)
                        .map(|_| ())
                    })
                    .await;
            });
        }
    }
}

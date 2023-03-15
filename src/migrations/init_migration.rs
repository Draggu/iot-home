use crate::devices::registry::model as device_model;
use crate::devices::specials::voltage::model as voltage_model;
use sea_orm::schema::Schema;
use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230313_155353_init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Sqlite);

        manager
            .create_table(
                schema
                    .create_table_from_entity(device_model::Entity)
                    .index(
                        IndexCreateStatement::new()
                            .unique()
                            .col(device_model::Column::DeviceName)
                            .col(device_model::Column::DisplayName)
                            .col(device_model::Column::Channel),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                schema
                    .create_table_from_entity(voltage_model::Entity)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(device_model::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(voltage_model::Entity)
                    .to_owned(),
            )
            .await
    }
}

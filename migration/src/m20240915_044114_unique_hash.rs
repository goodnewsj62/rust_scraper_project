use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240926_000001_create_school_table::SchoolData;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(SchoolData::Table)
                    .modify_column(ColumnDef::new(Alias::new("name_hash")).unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

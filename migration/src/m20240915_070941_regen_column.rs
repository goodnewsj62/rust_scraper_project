use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

use crate::m20240926_000001_create_school_table::SchoolData;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE school_data DROP CONSTRAINT school_data_name_hash_key")
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(SchoolData::Table)
                    .modify_column(ColumnDef::new(Alias::new("name_hash")).unique_key())
                    .to_owned(),
            )
            .await
    }
}

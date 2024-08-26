use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, migrate: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SchoolDate::Table)
                    .if_not_exists()
                    .col(pk_auto(SchoolData::Id))
                    .col(string(SchoolData::SchoolName))
                    .col(string(SchoolData::Location))
                    .col(ColumnDef::new(SchoolData::Logo).string().null())
                    .col(ColumnDef::new(SchoolData::City).string().null())
                    .col(ColumnDef::new(SchoolData::Country).string().not_null())
                    .col(ColumnDef::new(SchoolData::SchoolType).string().not_null())
                    .col(ColumnDef::new(SchoolData::Algorithm).string().not_null())
                    .col(ColumnDef::new(SchoolData::NameHash).string().not_null())
                    .create_index(Index::create().name("id_index").col(SchoolData::Id))
                    .create_index(Index::create().name("hash_index").col(SchoolData::NameHash))
                    .create_index(Index::create().name("name_index").col(SchoolData::Name)),
            )
            .await?
    }

    async fn down(&self, migrate: &SchemaManager) -> Result<(), DbErr> {}
}

#[derive(DeriveIden)]
pub enum SchoolData {
    Table,
    Id,
    SchoolName,
    Location,
    Contact,
    Logo,
    City,
    Country,
    SchoolType,
    NameHash,
    Algorithm,
}

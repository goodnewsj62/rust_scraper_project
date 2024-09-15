use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SchoolData::Table)
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
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("id_index")
                    .table(SchoolData::Table)
                    .col(SchoolData::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("hash_index")
                    .table(SchoolData::Table)
                    .col(SchoolData::NameHash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("name_index")
                    .table(SchoolData::Table)
                    .col(SchoolData::SchoolName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SchoolData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SchoolData {
    Table,
    Id,
    #[sea_orm(iden = "school_name")]
    SchoolName,
    Location,
    Contact,
    Logo,
    City,
    Country,
    SchoolType,
    #[sea_orm(iden = "name_hash")]
    NameHash,
    Algorithm,
}

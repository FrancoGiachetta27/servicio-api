use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Direccion::Table)
                    .if_not_exists()
                    .col(uuid(Direccion::Uuid).primary_key().not_null())
                    .col(string(Direccion::Provincia))
                    .col(string(Direccion::Calle))
                    .col(integer(Direccion::Altura))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Direccion::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Direccion {
    Table,
    Uuid,
    Provincia,
    Calle,
    Altura,
}

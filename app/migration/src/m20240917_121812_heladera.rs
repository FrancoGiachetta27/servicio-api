use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240913_225657_ubicacion::Ubicacion;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Heladera::Table)
                    .if_not_exists()
                    .col(uuid(Heladera::Uuid).primary_key().not_null())
                    .col(uuid(Heladera::DireccionId).not_null())
                    .col(integer(Heladera::CantidadViandas))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-hela-direccion_id")
                            .from(Heladera::Table, Heladera::DireccionId)
                            .to(Ubicacion::Table, Ubicacion::Uuid),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Heladera::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Heladera {
    Table,
    Uuid,
    DireccionId,
    CantidadViandas,
}

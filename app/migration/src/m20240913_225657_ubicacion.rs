use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240913_225704_direccion::Direccion;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ubicacion::Table)
                    .if_not_exists()
                    .col(uuid(Ubicacion::Uuid).primary_key().not_null())
                    .col(string(Ubicacion::Nombre))
                    .col(double(Ubicacion::Latitud))
                    .col(double(Ubicacion::Longitud))
                    .col(uuid(Ubicacion::DireccionId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ubic-direccion_id")
                            .from(Ubicacion::Table, Ubicacion::DireccionId)
                            .to(Direccion::Table, Direccion::Uuid),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ubicacion::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Ubicacion {
    Table,
    Uuid,
    Nombre,
    Latitud,
    Longitud,
    DireccionId,
}

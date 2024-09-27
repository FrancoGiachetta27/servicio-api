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
                    .table(PersonaVulnerable::Table)
                    .if_not_exists()
                    .col(uuid(PersonaVulnerable::Uuid).primary_key().not_null())
                    .col(string(PersonaVulnerable::Nombre).not_null())
                    .col(string(PersonaVulnerable::Apellido).not_null())
                    .col(uuid(PersonaVulnerable::DireccionId).not_null())
                    .col(uuid(PersonaVulnerable::ParienteACargo).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pers-direccion_id")
                            .from(PersonaVulnerable::Table, PersonaVulnerable::DireccionId)
                            .to(Ubicacion::Table, Ubicacion::Uuid),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pariente_id")
                            .from(PersonaVulnerable::Table, PersonaVulnerable::ParienteACargo)
                            .to(PersonaVulnerable::Table, PersonaVulnerable::Uuid),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PersonaVulnerable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PersonaVulnerable {
    Table,
    Uuid,
    Nombre,
    Apellido,
    DireccionId,
    ParienteACargo,
}

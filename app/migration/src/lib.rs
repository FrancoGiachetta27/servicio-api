pub use sea_orm_migration::prelude::*;

mod m20240913_225644_persona_vulnerable;
mod m20240913_225657_ubicacion;
mod m20240913_225704_direccion;
mod m20240917_121812_heladera;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240913_225704_direccion::Migration),
            Box::new(m20240913_225657_ubicacion::Migration),
            Box::new(m20240913_225644_persona_vulnerable::Migration),
            Box::new(m20240917_121812_heladera::Migration),
        ]
    }
}

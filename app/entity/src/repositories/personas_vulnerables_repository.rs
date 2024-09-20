use std::env;

use sea_orm::Database;
use sea_orm::{
    sea_query::IntoCondition, DatabaseConnection, DeleteResult, EntityOrSelect, EntityTrait,
    InsertResult, QueryFilter,
};
use uuid::Uuid;

use super::Repository;
use crate::persona_vulnerable::{ActiveModel, Entity as PersonaVulnerable, Model};

#[derive(Clone)]
pub struct PersonaVulnerableRepository {
    db: DatabaseConnection,
}

impl PersonaVulnerableRepository {
    pub async fn new() -> Result<Self, sea_orm::DbErr> {
        let db = Database::connect(
            env::var("DATABASE_URL").expect("No se pudo conectarse a la base de datos"),
        )
        .await?;

        Ok(Self { db })
    }
}

impl Repository<Model, ActiveModel> for PersonaVulnerableRepository {
    async fn all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        PersonaVulnerable::find().all(&self.db).await
    }
    async fn filter<C: IntoCondition>(&self, filter: C) -> Result<Vec<Model>, sea_orm::DbErr> {
        PersonaVulnerable::find()
            .select()
            .filter(filter)
            .all(&self.db)
            .await
    }

    async fn save(
        &self,
        insertable: ActiveModel,
    ) -> Result<InsertResult<ActiveModel>, sea_orm::DbErr> {
        PersonaVulnerable::insert(insertable).exec(&self.db).await
    }
    async fn update(&self, insertable: ActiveModel) -> Result<Model, sea_orm::DbErr> {
        PersonaVulnerable::update(insertable).exec(&self.db).await
    }
    async fn delete(&self, id: Uuid) -> Result<DeleteResult, sea_orm::DbErr> {
        PersonaVulnerable::delete_by_id(id).exec(&self.db).await
    }
}

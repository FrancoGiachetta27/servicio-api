use sea_orm::{Condition, Statement};
use sea_orm::{
    sea_query::IntoCondition, DatabaseConnection, DeleteResult, EntityOrSelect, EntityTrait,
    InsertResult, QueryFilter,
};
use uuid::Uuid;

use super::Repository;
use crate::persona_vulnerable::{
    ActiveModel, Entity as PersonaVulnerable, Model, SelfReferencingLink,
};
use crate::ubicacion;

#[derive(Clone)]
pub struct PersonaVulnerableRepository {
    db: DatabaseConnection,
}

impl PersonaVulnerableRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, sea_orm::DbErr> {
        Ok(Self { db })
    }
}

impl Repository<Model, ActiveModel> for PersonaVulnerableRepository {
    async fn all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        PersonaVulnerable::find().all(&self.db).await
    }

    async fn filter(&self, filter: Condition) -> Result<Vec<Model>, sea_orm::DbErr> {
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

    async fn raw(&self, query: Statement) -> Result<Vec<Model>, sea_orm::DbErr> {
        PersonaVulnerable::find()
            .from_raw_sql(query)
            .all(&self.db)
            .await
    }
}

impl PersonaVulnerableRepository {
    pub async fn find_related(
        &self,
        filter: Option<Condition>,
        entity: ubicacion::Entity,
    ) -> Result<Vec<(Model, Vec<ubicacion::Model>)>, sea_orm::DbErr> {
        let query = PersonaVulnerable::find().find_with_related(entity);

        match filter {
            Some(f) => query.filter(f).all(&self.db).await,
            None => query.all(&self.db).await,
        }
    }

    pub async fn find_self_related<C: IntoCondition>(
        &self,
        filter: Option<C>,
    ) -> Result<Vec<(Model, Vec<Model>)>, sea_orm::DbErr> {
        let query = PersonaVulnerable::find().find_with_linked(SelfReferencingLink);

        match filter {
            Some(f) => query.filter(f).all(&self.db).await,
            None => query.all(&self.db).await,
        }
    }
}

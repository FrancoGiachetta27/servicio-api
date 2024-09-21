use sea_orm::QuerySelect;
use sea_orm::{
    sea_query::IntoCondition, DatabaseConnection, DeleteResult, EntityOrSelect, EntityTrait,
    InsertResult, QueryFilter, Statement,
};
use uuid::Uuid;

use super::Repository;
use crate::heladera::{self, ActiveModel, Entity as Heladera, Model};
use crate::ubicacion::{self, Entity as Ubicacion};

#[derive(Clone)]
pub struct HeladeraRepository {
    db: DatabaseConnection,
}

impl HeladeraRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, sea_orm::DbErr> {
        Ok(Self { db })
    }
}

impl Repository<Model, ActiveModel> for HeladeraRepository {
    async fn all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        Heladera::find().all(&self.db).await
    }
    async fn filter<C: IntoCondition>(&self, filter: C) -> Result<Vec<Model>, sea_orm::DbErr> {
        Heladera::find().select().filter(filter).all(&self.db).await
    }
    async fn save(
        &self,
        insertable: ActiveModel,
    ) -> Result<InsertResult<ActiveModel>, sea_orm::DbErr> {
        Heladera::insert(insertable).exec(&self.db).await
    }
    async fn update(&self, insertable: ActiveModel) -> Result<Model, sea_orm::DbErr> {
        Heladera::update(insertable).exec(&self.db).await
    }
    async fn delete(&self, id: Uuid) -> Result<DeleteResult, sea_orm::DbErr> {
        Heladera::delete_by_id(id).exec(&self.db).await
    }
    async fn raw(&self, query: Statement) -> Result<Vec<Model>, sea_orm::DbErr> {
        Heladera::find().from_raw_sql(query).all(&self.db).await
    }
}

impl HeladeraRepository {
    pub async fn find_related<C: IntoCondition>(
        &self,
        filter: Option<C>,
        entity: ubicacion::Entity,
    ) -> Result<Vec<(Model, Vec<ubicacion::Model>)>, sea_orm::DbErr> {
        let query = Heladera::find().find_with_related(entity);

        match filter {
            Some(f) => query.filter(f).all(&self.db).await,
            None => query.all(&self.db).await,
        }
    }
}

use sea_orm::sea_query::SimpleExpr;
use sea_orm::{
    sea_query::IntoCondition, DatabaseConnection, DeleteResult, EntityOrSelect, EntityTrait,
    InsertResult, QueryFilter, Statement,
};
use uuid::Uuid;

use super::Repository;
use crate::heladera::{ActiveModel, Entity as Heladera, Model};
use crate::ubicacion;

#[derive(Clone)]
pub struct HeladeraRepository {
    db: DatabaseConnection,
}

impl HeladeraRepository {
    pub async fn new(db: &DatabaseConnection) -> Result<Self, sea_orm::DbErr> {
        let db = db.clone();

        Ok(Self { db })
    }

    pub async fn find_related(
        &self,
        filter: Option<SimpleExpr>,
        entity: ubicacion::Entity,
    ) -> Result<Vec<(Model, Option<ubicacion::Model>)>, sea_orm::DbErr> {
        let query = Heladera::find().find_also_related(entity);

        match filter {
            Some(f) => query.filter(f).all(&self.db).await,
            None => query.all(&self.db).await,
        }
    }
}

impl Repository<Model, ActiveModel> for HeladeraRepository {
    async fn all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        Heladera::find().all(&self.db).await
    }

    async fn filter<C: IntoCondition>(&self, filter: C) -> Result<Vec<Model>, sea_orm::DbErr> {
        Heladera::find().filter(filter).all(&self.db).await
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

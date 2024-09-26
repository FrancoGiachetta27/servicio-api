use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use super::Repository;
use crate::direccion::{ActiveModel, Entity as Direccion, Model};

#[derive(Clone)]
pub struct DireccionRepository {
    db: DatabaseConnection,
}

impl DireccionRepository {
    pub async fn new(db: &DatabaseConnection) -> Result<Self, sea_orm::DbErr> {
        let db = db.clone();

        Ok(Self { db })
    }
}

impl Repository<Model, ActiveModel> for DireccionRepository {
    async fn all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        Direccion::find().all(&self.db).await
    }

    async fn save(
        &self,
        insertable: ActiveModel,
    ) -> Result<sea_orm::InsertResult<ActiveModel>, sea_orm::DbErr> {
        Direccion::insert(insertable).exec(&self.db).await
    }

    async fn update(&self, insertable: ActiveModel) -> Result<Model, sea_orm::DbErr> {
        Direccion::update(insertable).exec(&self.db).await
    }
    async fn delete(&self, id: uuid::Uuid) -> Result<sea_orm::DeleteResult, sea_orm::DbErr> {
        Direccion::delete_by_id(id).exec(&self.db).await
    }
    async fn filter<C: sea_orm::sea_query::IntoCondition>(
        &self,
        filter: C,
    ) -> Result<Vec<Model>, sea_orm::DbErr> {
        Direccion::find().filter(filter).all(&self.db).await
    }

    async fn raw(&self, query: sea_orm::Statement) -> Result<Vec<Model>, sea_orm::DbErr> {
        Direccion::find().from_raw_sql(query).all(&self.db).await
    }
}

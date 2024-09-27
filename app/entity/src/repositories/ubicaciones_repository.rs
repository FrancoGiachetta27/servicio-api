use sea_orm::sea_query::{IntoCondition, SimpleExpr};
use sea_orm::Statement;
use sea_orm::{DatabaseConnection, DeleteResult, EntityOrSelect, EntityTrait, QueryFilter};
use uuid::Uuid;

use super::Repository;
use crate::direccion;
use crate::ubicacion::{ActiveModel, Entity as Ubicacion, Model};

#[derive(Clone)]
pub struct UbicacionRepository {
    db: DatabaseConnection,
}

impl UbicacionRepository {
    pub async fn new(db: &DatabaseConnection) -> Result<Self, sea_orm::DbErr> {
        let db = db.clone();
        Ok(Self { db })
    }

    pub async fn find_related(
        &self,
        filter: Option<SimpleExpr>,
        entity: direccion::Entity,
    ) -> Result<Vec<(Model, Option<direccion::Model>)>, sea_orm::DbErr> {
        let query = Ubicacion::find().find_also_related(entity);

        match filter {
            Some(f) => query.filter(f).all(&self.db).await,
            None => query.all(&self.db).await,
        }
    }
}

impl Repository<Model, ActiveModel> for UbicacionRepository {
    async fn all(&self) -> Result<Vec<Model>, sea_orm::DbErr> {
        Ubicacion::find().all(&self.db).await
    }

    async fn filter<C: IntoCondition>(&self, filter: C) -> Result<Vec<Model>, sea_orm::DbErr> {
        Ubicacion::find()
            .select()
            .filter(filter)
            .all(&self.db)
            .await
    }

    async fn save(&self, insertable: ActiveModel) -> Result<Model, sea_orm::DbErr> {
        Ubicacion::insert(insertable)
            .exec_with_returning(&self.db)
            .await
    }

    async fn update(&self, insertable: ActiveModel) -> Result<Model, sea_orm::DbErr> {
        Ubicacion::update(insertable).exec(&self.db).await
    }

    async fn delete(&self, id: Uuid) -> Result<DeleteResult, sea_orm::DbErr> {
        Ubicacion::delete_by_id(id).exec(&self.db).await
    }

    async fn raw(&self, query: Statement) -> Result<Vec<Model>, sea_orm::DbErr> {
        Ubicacion::find().from_raw_sql(query).all(&self.db).await
    }
}

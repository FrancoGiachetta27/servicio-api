pub mod heladeras_repository;
pub mod personas_vulnerables_repository;
pub mod ubicaciones_repository;

use sea_orm::{sea_query::IntoCondition, ActiveModelTrait, DeleteResult, InsertResult, ModelTrait, Statement};
use uuid::Uuid;

#[macro_export]
macro_rules! find_join {
    ($db : expr, $model : ty, $($join : expr), *) => {
        <$model>::find() $(
            .inner_join($join)
        )*
        .all($db).await
    }
}

pub trait Repository<M, A>
where
    M: ModelTrait,
    A: ActiveModelTrait,
{
    async fn all(&self) -> Result<Vec<M>, sea_orm::DbErr>;
    async fn save(&self, insertable: A) -> Result<InsertResult<A>, sea_orm::DbErr>;
    async fn update(&self, insertable: A) -> Result<M, sea_orm::DbErr>;
    async fn delete(&self, id: Uuid) -> Result<DeleteResult, sea_orm::DbErr>;
    async fn filter<C: IntoCondition>(&self, filter: C) -> Result<Vec<M>, sea_orm::DbErr>;
    async fn raw(&self, query: Statement) -> Result<Vec<M>, sea_orm::DbErr>;
}

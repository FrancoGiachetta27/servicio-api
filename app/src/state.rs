use entity::repositories::{
    heladeras_repository::HeladeraRepository,
    personas_vulnerables_repository::PersonaVulnerableRepository,
    ubicaciones_repository::UbicacionRepository,
};
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct AppState {
    pub personas_vulnerables_repo: PersonaVulnerableRepository,
    pub heladeras_repo: HeladeraRepository,
    pub ubicaciones_repo: UbicacionRepository,
}

impl AppState {
    pub async fn new(db: DatabaseConnection) -> Result<Self, DbErr> {
        let personas_vulnerables_repo = PersonaVulnerableRepository::new(&db).await.unwrap();
        let heladeras_repo = HeladeraRepository::new(&db).await.unwrap();
        let ubicaciones_repo = UbicacionRepository::new(&db).await.unwrap();
        let direccion_repo = DireccionRepository::new(&db).await.unwrap();
        
        Ok(AppState {
            personas_vulnerables_repo,
            heladeras_repo,
            ubicaciones_repo,
        })
    }
}

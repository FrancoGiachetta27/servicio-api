use chrono::NaiveDate;
use entity::{
    direccion::{ActiveModel as DireccionModel, Entity as DireccionEntity},
    heladera::{ActiveModel as HeladeraModel, Entity as HeladeraEntity},
    persona_vulnerable::{ActiveModel as PersonaModel, Entity as PersonaEntity},
    repositories::{
        heladeras_repository::HeladeraRepository,
        personas_vulnerables_repository::PersonaVulnerableRepository,
    },
    ubicacion::{ActiveModel as UbicacionModel, Entity as UbicacionEntity},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveValue::Set, Database, DatabaseConnection, EntityTrait};
use test_context::AsyncTestContext;
use uuid::Uuid;

pub struct TestContext {
    pub personas_repo: PersonaVulnerableRepository,
    pub heladeras_repo: HeladeraRepository,
    pub db: DatabaseConnection,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> TestContext {
        let _ = dotenv::dotenv();

        let db = Database::connect(std::env::var("DATABASE_LOCAL").unwrap())
            .await
            .unwrap();

        let personas_repo = PersonaVulnerableRepository::new(&db).await.unwrap();
        let heladeras_repo = HeladeraRepository::new(&db).await.unwrap();

        migrate(&db).await;

        TestContext {
            personas_repo,
            heladeras_repo,
            db,
        }
    }

    async fn teardown(self) {
        // dropea todas las tablas
        Migrator::reset(&self.db).await.ok();
    }
}

pub async fn migrate(db: &DatabaseConnection) {
    // migrar esquemas
    Migrator::up(db, None).await.ok();

    // Direcciones

    let direcciones = vec![
        DireccionModel {
            uuid: Set(Uuid::from_u128(1).into()),
            calle: Set("Urquiza".into()),
            altura: Set(200),
            provincia: Set("Santa Fe".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(2).into()),
            calle: Set("Hipolito Hirigoyen".into()),
            altura: Set(300),
            provincia: Set("Cordoba".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(3).into()),
            calle: Set("Beltran".into()),
            altura: Set(100),
            provincia: Set("Mendoza".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(4).into()),
            calle: Set("Paraguay".into()),
            altura: Set(90),
            provincia: Set("Corrientes".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(5).into()),
            calle: Set("Corrientes".into()),
            altura: Set(700),
            provincia: Set("CABA".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(6).into()),
            calle: Set("25 de Mayo".into()),
            altura: Set(450),
            provincia: Set("Entre Rios".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(7).into()),
            calle: Set("Juan B Justo".into()),
            altura: Set(22),
            provincia: Set("CABA".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(8).into()),
            calle: Set("Medrano".into()),
            altura: Set(951),
            provincia: Set("CABA".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(9).into()),
            calle: Set("Directorio".into()),
            altura: Set(532),
            provincia: Set("CABA".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(10).into()),
            calle: Set("Acoyte".into()),
            altura: Set(300),
            provincia: Set("CABA".into()),
        },
        DireccionModel {
            uuid: Set(Uuid::from_u128(11).into()),
            calle: Set("Medrano".into()),
            altura: Set(22),
            provincia: Set("CABA".into()),
        },
    ];

    // Ubicaciones

    let ubicaciones = vec![
        UbicacionModel {
            uuid: Set(Uuid::from_u128(1).into()),
            latitud: Set(-33.15),
            longitud: Set(-60.49),
            direccion_id: Set(Uuid::from_u128(1).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(2).into()),
            latitud: Set(-33.12),
            longitud: Set(-64.34),
            direccion_id: Set(Uuid::from_u128(2).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(3).into()),
            latitud: Set(-32.92),
            longitud: Set(-68.84),
            direccion_id: Set(Uuid::from_u128(3).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(4).into()),
            latitud: Set(-29.13),
            longitud: Set(-59.25),
            direccion_id: Set(Uuid::from_u128(4).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(5).into()),
            latitud: Set(-38.71),
            longitud: Set(-62.25),
            direccion_id: Set(Uuid::from_u128(5).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(6).into()),
            latitud: Set(-31.62),
            longitud: Set(-58.50),
            direccion_id: Set(Uuid::from_u128(6).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(7).into()),
            latitud: Set(-34.59),
            longitud: Set(-58.42),
            direccion_id: Set(Uuid::from_u128(7).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(8).into()),
            latitud: Set(-34.43),
            longitud: Set(-61.82),
            direccion_id: Set(Uuid::from_u128(8).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(9).into()),
            latitud: Set(-34.66),
            longitud: Set(-58.41),
            direccion_id: Set(Uuid::from_u128(9).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(10).into()),
            latitud: Set(-34.60),
            longitud: Set(-58.44),
            direccion_id: Set(Uuid::from_u128(10).into()),
        },
        UbicacionModel {
            uuid: Set(Uuid::from_u128(11).into()),
            latitud: Set(-33.80),
            longitud: Set(-59.51),
            direccion_id: Set(Uuid::from_u128(11).into()),
        },
    ];

    // Heladera

    let heladeras = vec![
        HeladeraModel {
            uuid: Set(Uuid::from_u128(1).into()),
            direccion_id: Set(Uuid::from_u128(1).into()),
            cantidad_viandas: Set(2),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(2).into()),
            direccion_id: Set(Uuid::from_u128(2).into()),
            cantidad_viandas: Set(10),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(3).into()),
            direccion_id: Set(Uuid::from_u128(3).into()),
            cantidad_viandas: Set(23),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(4).into()),
            direccion_id: Set(Uuid::from_u128(4).into()),
            cantidad_viandas: Set(2),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(5).into()),
            direccion_id: Set(Uuid::from_u128(5).into()),
            cantidad_viandas: Set(14),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(6).into()),
            direccion_id: Set(Uuid::from_u128(6).into()),
            cantidad_viandas: Set(43),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(7).into()),
            direccion_id: Set(Uuid::from_u128(7).into()),
            cantidad_viandas: Set(10),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(8).into()),
            direccion_id: Set(Uuid::from_u128(8).into()),
            cantidad_viandas: Set(28),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(9).into()),
            direccion_id: Set(Uuid::from_u128(9).into()),
            cantidad_viandas: Set(30),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(10).into()),
            direccion_id: Set(Uuid::from_u128(10).into()),
            cantidad_viandas: Set(34),
        },
        HeladeraModel {
            uuid: Set(Uuid::from_u128(11).into()),
            direccion_id: Set(Uuid::from_u128(11).into()),
            cantidad_viandas: Set(25),
        },
    ];

    // Personas

    let personas = vec![
        PersonaModel {
            uuid: Set(Uuid::from_u128(1).into()),
            nombre: Set("Pablo".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(1).into()),
            pariente_a_cargo: Set(None),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(2).into()),
            nombre: Set("Alan".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(2).into()),
            pariente_a_cargo: Set(Some(Uuid::from_u128(1).into())),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(3).into()),
            nombre: Set("Santiago".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(3).into()),
            pariente_a_cargo: Set(Some(Uuid::from_u128(1).into())),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(4).into()),
            nombre: Set("Marcos".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(4).into()),
            pariente_a_cargo: Set(Some(Uuid::from_u128(1).into())),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(5).into()),
            nombre: Set("Elina".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(5).into()),
            pariente_a_cargo: Set(Some(Uuid::from_u128(3).into())),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(6).into()),
            nombre: Set("Sofia".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(6).into()),
            pariente_a_cargo: Set(None),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(7).into()),
            nombre: Set("Maria".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(7).into()),
            pariente_a_cargo: Set(None),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(8).into()),
            nombre: Set("Delfina".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(8).into()),
            pariente_a_cargo: Set(Some(Uuid::from_u128(7).into())),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(9).into()),
            nombre: Set("Nicole".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(9).into()),
            pariente_a_cargo: Set(None),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(10).into()),
            nombre: Set("Florencia".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(10).into()),
            pariente_a_cargo: Set(None),
        },
        PersonaModel {
            uuid: Set(Uuid::from_u128(11).into()),
            nombre: Set("Fiona".into()),
            apellido: Set("Perez".into()),
            direccion_id: Set(Uuid::from_u128(11).into()),
            pariente_a_cargo: Set(None),
        },
    ];

    DireccionEntity::insert_many(direcciones)
        .exec(db)
        .await
        .ok();
    UbicacionEntity::insert_many(ubicaciones)
        .exec(db)
        .await
        .ok();
    PersonaEntity::insert_many(personas).exec(db).await.ok();
    HeladeraEntity::insert_many(heladeras).exec(db).await.ok();
}

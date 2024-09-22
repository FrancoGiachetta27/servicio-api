mod common;

use common::TestContext;
use entity::{
    direccion::{Column as DireccionColumn, Entity as DireccionEntity},
    heladera::{Column as HeladeraColumn, Entity as Heladera},
    persona_vulnerable::{Column as PersonaColumn, Entity as Persona},
    ubicacion::{Column as UbicacionColumn, Entity as Ubicacion},
};
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect};
use test_context::test_context;
use uuid::Uuid;

#[derive(FromQueryResult, PartialEq, Eq, Debug)]
struct PartialPersona {
    pub uuid: Vec<u8>,
    pub nombre: String,
}

impl PartialPersona {
    pub fn new(uuid: Uuid, nombre: &str) -> Self {
        let uuid = uuid.into_bytes().to_vec();

        Self {
            uuid,
            nombre: nombre.to_string(),
        }
    }
}

#[test_context(TestContext)]
#[tokio::test]
async fn test_personas_query(ctx: &mut TestContext) {
    let nombres = Persona::find()
        .select_only()
        .columns([PersonaColumn::Uuid, PersonaColumn::Nombre])
        .into_model::<PartialPersona>()
        .all(&ctx.db)
        .await
        .unwrap();

    let nombres_esperados = Vec::from([
        PartialPersona::new(Uuid::from_u128(1), "Pablo"),
        PartialPersona::new(Uuid::from_u128(2), "Alan"),
        PartialPersona::new(Uuid::from_u128(3), "Santiago"),
        PartialPersona::new(Uuid::from_u128(4), "Marcos"),
        PartialPersona::new(Uuid::from_u128(5), "Elina"),
        PartialPersona::new(Uuid::from_u128(6), "Sofia"),
        PartialPersona::new(Uuid::from_u128(7), "Maria"),
        PartialPersona::new(Uuid::from_u128(8), "Delfina"),
        PartialPersona::new(Uuid::from_u128(9), "Nicole"),
        PartialPersona::new(Uuid::from_u128(10), "Florencia"),
        PartialPersona::new(Uuid::from_u128(11), "Fiona"),
    ]);

    assert_eq!(nombres_esperados, nombres);
}

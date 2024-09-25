use crate::common::TestContext;
use chrono::NaiveDate;
use entity::{
    direccion::{Column as DireccionColumn, Entity as DireccionEntity},
    heladera::{Column as HeladeraColumn, Entity as Heladera},
    persona_vulnerable::{
        Column as PersonaColumn, Entity as Persona, Model as PersonaModel, SelfLinkHijos,
    },
    repositories::Repository,
    ubicacion::{Column as UbicacionColumn, Entity as Ubicacion, Model as UbicacionModel},
};
use sea_orm::{ColumnTrait, EntityTrait};
use test_context::test_context;
use uuid::Uuid;

#[test_context(TestContext)]
#[tokio::test]
async fn test_personas_query(ctx: &mut TestContext) {
    let nombres = ctx.personas_repo.all().await.unwrap();

    let nombres = nombres
        .into_iter()
        .map(|p| (Uuid::from_slice(&p.uuid).unwrap(), p.nombre))
        .collect::<Vec<(Uuid, String)>>();

    let nombres_esperados = Vec::from([
        (Uuid::from_u128(1), "Pablo".into()),
        (Uuid::from_u128(2), "Alan".into()),
        (Uuid::from_u128(3), "Santiago".into()),
        (Uuid::from_u128(4), "Marcos".into()),
        (Uuid::from_u128(5), "Elina".into()),
        (Uuid::from_u128(6), "Sofia".into()),
        (Uuid::from_u128(7), "Maria".into()),
        (Uuid::from_u128(8), "Delfina".into()),
        (Uuid::from_u128(9), "Nicole".into()),
        (Uuid::from_u128(10), "Florencia".into()),
        (Uuid::from_u128(11), "Fiona".into()),
    ]);

    assert_eq!(nombres_esperados, nombres);
}

#[test_context(TestContext)]
#[tokio::test]
async fn test_personas_join(ctx: &mut TestContext) {
    let uuid = Uuid::from_u128(1);

    let persona_ubicacion = ctx
        .personas_repo
        .find_related(Some(PersonaColumn::Uuid.eq(uuid)), Ubicacion)
        .await
        .unwrap();

    let persona_ubicacion_esperada = vec![(
        PersonaModel {
            uuid: Uuid::from_u128(1).into(),
            nombre: "Pablo".into(),
            apellido: "Perez".into(),
            fecha_nacimiento: NaiveDate::from_ymd_opt(1978, 3, 22).unwrap(),
            direccion_id: Uuid::from_u128(1).into(),
            pariente_a_cargo: None,
        },
        Some(UbicacionModel {
            uuid: Uuid::from_u128(1).into(),
            nombre: "Dom Pablo".into(),
            latitud: -33.15,
            longitud: -60.49,
            direccion_id: Uuid::from_u128(1).into(),
        }),
    )];

    assert_eq!(persona_ubicacion_esperada, persona_ubicacion);
}

#[test_context(TestContext)]
#[tokio::test]
async fn test_personas_auto_join(ctx: &mut TestContext) {
    let uuid = Uuid::from_u128(1);

    let persona_hijos = ctx
        .personas_repo
        .find_self_related(Some(PersonaColumn::Uuid.eq(uuid)), SelfLinkHijos)
        .await
        .unwrap();

    let persona_hijos_esperado = vec![(
        PersonaModel {
            uuid: Uuid::from_u128(1).into(),
            nombre: "Pablo".into(),
            apellido: "Perez".into(),
            fecha_nacimiento: NaiveDate::from_ymd_opt(1978, 3, 22).unwrap(),
            direccion_id: Uuid::from_u128(1).into(),
            pariente_a_cargo: None,
        },
        vec![
            PersonaModel {
                uuid: Uuid::from_u128(2).into(),
                nombre: "Alan".into(),
                apellido: "Perez".into(),
                fecha_nacimiento: NaiveDate::from_ymd_opt(2002, 11, 8).unwrap(),
                direccion_id: Uuid::from_u128(2).into(),
                pariente_a_cargo: Some(Uuid::from_u128(1).into()),
            },
            PersonaModel {
                uuid: Uuid::from_u128(3).into(),
                nombre: "Santiago".into(),
                apellido: "Perez".into(),
                fecha_nacimiento: NaiveDate::from_ymd_opt(2003, 4, 4).unwrap(),
                direccion_id: Uuid::from_u128(3).into(),
                pariente_a_cargo: Some(Uuid::from_u128(1).into()),
            },
            PersonaModel {
                uuid: Uuid::from_u128(4).into(),
                nombre: "Marcos".into(),
                apellido: "Perez".into(),
                fecha_nacimiento: NaiveDate::from_ymd_opt(2003, 6, 26).unwrap(),
                direccion_id: Uuid::from_u128(4).into(),
                pariente_a_cargo: Some(Uuid::from_u128(1).into()),
            },
        ],
    )];

    assert_eq!(persona_hijos_esperado, persona_hijos);
}

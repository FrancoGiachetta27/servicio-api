use entity::{
    heladera::{Column as HeladeraColumn, Model as HeladeraModel},
    repositories::Repository,
    ubicacion::{Entity as Ubicacion, Model as UbicacionModel},
};
use sea_orm::ColumnTrait;
use test_context::test_context;
use uuid::Uuid;

use crate::common::TestContext;

#[test_context(TestContext)]
#[tokio::test]
async fn test_heladeras_query(ctx: &mut TestContext) {
    let heladeras = ctx.heladeras_repo.all().await.unwrap();

    let heladeras = heladeras
        .into_iter()
        .map(|h| (Uuid::from_slice(&h.uuid).unwrap(), h.cantidad_viandas))
        .collect::<Vec<(Uuid, i16)>>();

    let heladeras_esperados = Vec::from([
        (Uuid::from_u128(1), 2),
        (Uuid::from_u128(2), 10),
        (Uuid::from_u128(3), 23),
        (Uuid::from_u128(4), 2),
        (Uuid::from_u128(5), 14),
        (Uuid::from_u128(6), 43),
        (Uuid::from_u128(7), 10),
        (Uuid::from_u128(8), 28),
        (Uuid::from_u128(9), 30),
        (Uuid::from_u128(10), 34),
        (Uuid::from_u128(11), 25),
    ]);

    assert_eq!(heladeras_esperados, heladeras);
}

#[test_context(TestContext)]
#[tokio::test]
async fn test_join_heladeras(ctx: &mut TestContext) {
    let uuid = Uuid::from_u128(1);

    let heladera_ubicacion = ctx
        .heladeras_repo
        .find_related(Some(HeladeraColumn::Uuid.eq(uuid)), Ubicacion)
        .await
        .unwrap();

    let heladera_ubicacion_esperada = vec![(
        HeladeraModel {
            uuid: uuid.into(),
            direccion_id: Uuid::from_u128(1).into(),
            cantidad_viandas: 2,
        },
        Some(UbicacionModel {
            uuid: Uuid::from_u128(1).into(),
            nombre: "Dom Pablo".into(),
            latitud: -33.15,
            longitud: -60.49,
            direccion_id: Uuid::from_u128(1).into(),
        }),
    )];

    assert_eq!(heladera_ubicacion_esperada, heladera_ubicacion);
}

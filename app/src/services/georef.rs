use serde::{Deserialize, Serialize};

const GEOREF: &str = "https://apis.datos.gob.ar/georef/api/";

#[derive(Deserialize, Serialize, Debug)]
struct Altura {
    pub unidad: Option<String>,
    pub valor: i16,
}

#[derive(Deserialize, Serialize, Debug)]
struct Calle {
    pub id: String,
    pub categoria: String,
    pub nombre: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Departamento {
    pub id: String,
    pub nombre: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Localidad {
    pub id: String,
    pub nombre: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Provincia {
    pub id: String,
    pub nombre: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coordenadas {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Direccion {
    pub altura: Altura,
    departamento: Option<Departamento>,
    pub calle: Calle,
    localidad_censal: Localidad,
    nomenclatura: String,
    piso: Option<String>,
    pub provincia: Provincia,
    pub ubicacion: Coordenadas,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeoRefIn {
    pub cantidad: i16,
    pub direcciones: Vec<Direccion>,
}

pub fn request_georef_direccion(
    calle: String,
    altura: i16,
    provincia: Option<String>,
) -> Result<ureq::Response, ureq::Error> {
    let endpoint = GEOREF.to_string() + "direcciones";
    let direccion = calle + &altura.to_string();

    let query_params: [(&str, &str); 3] = [
        ("direccion", &direccion),
        (
            "provincia",
            match provincia {
                Some(p) => &p.clone(),
                None => "",
            },
        ),
        ("max", "1"),
    ];

    ureq::get(&endpoint).query_pairs(query_params).call()
}

pub fn request_georef_ubicacion(
    latitud: f64,
    longitud: f64,
) -> Result<ureq::Response, ureq::Error> {
    let latitud = latitud.to_string();
    let longitud = longitud.to_string();
    let query_params: [(&str, &str); 2] = [("lat", &latitud), ("lon", &longitud)];

    ureq::get(GEOREF).query_pairs(query_params).call()
}

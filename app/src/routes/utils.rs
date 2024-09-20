use std::f32::consts::PI;

pub fn distancia_haversine(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let radio = 6371.0; // km
    let pi_grados = PI / 180.0;

    let a = 0.5 - ((lat2 - lat1) * pi_grados).cos() / 2.0
        + (lat1 * pi_grados).cos()
            * (lat2 * pi_grados).cos()
            * (1.0 - ((lon2 - lon1) * pi_grados)).cos()
            / 2.0;

    2.0 * radio * a.sqrt().cos()
}

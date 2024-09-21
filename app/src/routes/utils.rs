use std::f32::consts::PI;

pub fn distancia_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let radio = 6371.0; // km
    let pi_grados = PI / 180.0;

    let a = 0.5 - ((lat2 - lat1) * pi_grados as f64).cos() / 2.0
        + (lat1 * pi_grados as f64).cos()
            * (lat2 * pi_grados as f64).cos()
            * (1.0 - ((lon2 - lon1) * pi_grados as f64)).cos()
            / 2.0;

    2.0 * radio * a.sqrt().cos()
}

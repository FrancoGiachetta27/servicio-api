pub fn distancia_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let radio = 6372.8; // km

    let dist_lat = (lat2 - lat1).to_radians();
    let dist_lon = (lon2 - lon1).to_radians();
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();

    let a = (dist_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (dist_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    radio * c
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::distancia_haversine;

    #[rstest]
    #[case(36.12, -86.67, 33.94, -118.40, 2887.0)]
    fn test_haversine(
        #[case] lat1: f64,
        #[case] lon1: f64,
        #[case] lat2: f64,
        #[case] lon2: f64,
        #[case] dist_esperada: f64,
    ) {
        let dist = distancia_haversine(lat1, lon1, lat2, lon2);

        assert_eq!(dist_esperada, dist.floor());
    }
}

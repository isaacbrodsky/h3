use crate::h3api::LatLng;

/** @struct Vec3D
 *  @brief 3D floating point structure
 */
pub struct Vec3d {
    // x component
    pub x: f64,
    // y component
    pub y: f64,
    // z component
    pub z: f64,
}

/**
 * Square of a number
 *
 * @param x The input number.
 * @return The square of the input number.
 */
pub fn _square(x: f64) -> f64 {
    x * x
}

/**
 * Calculate the square of the distance between two 3D coordinates.
 *
 * @param v1 The first 3D coordinate.
 * @param v2 The second 3D coordinate.
 * @return The square of the distance between the given points.
 */
pub fn _pointSquareDist(v1: Vec3d, v2: Vec3d) -> f64 {
    _square(v1.x - v2.x) + _square(v1.y - v2.y) + _square(v1.z - v2.z)
}

/**
 * Calculate the 3D coordinate on unit sphere from the latitude and longitude.
 *
 * @param geo The latitude and longitude of the point.
 * @param v The 3D coordinate of the point.
 */
pub fn _geoToVec3d(geo: LatLng, v: &mut Vec3d) {
    let r = geo.lat.cos();

    v.z = geo.lat.sin();
    v.x = geo.lng.cos() * r;
    v.y = geo.lng.sin() * r;
}

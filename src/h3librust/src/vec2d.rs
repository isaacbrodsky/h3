/** @struct Vec2D
 *  @brief 2D floating point structure
 */
#[derive(Copy, Clone)]
pub struct Vec2d {
    // x component
    pub x: f64,
    // y component
    pub y: f64,
}

/**
 * Calculates the magnitude of a 2D cartesian vector.
 * @param v The 2D cartesian vector.
 * @return The magnitude of the vector.
 */
pub fn _v2dMag(v: Vec2d) -> f64 {
    (v.x * v.x + v.y * v.y).sqrt()
}

/**
 * Finds the intersection between two lines. Assumes that the lines intersect
 * and that the intersection is not at an endpoint of either line.
 * @param p0 The first endpoint of the first line.
 * @param p1 The second endpoint of the first line.
 * @param p2 The first endpoint of the second line.
 * @param p3 The second endpoint of the second line.
 * @param inter The intersection point.
 */
pub fn _v2dIntersect(p0: Vec2d, p1: Vec2d, p2: Vec2d, p3: Vec2d, inter: &mut Vec2d) {
    let s1 = Vec2d {
        x: p1.x - p0.x,
        y: p1.y - p0.y,
    };
    let s2 = Vec2d {
        x: p3.x - p2.x,
        y: p3.y - p2.y,
    };

    let t = (s2.x * (p0.y - p2.y) - s2.y * (p0.x - p2.x)) / (-s2.x * s1.y + s1.x * s2.y);

    inter.x = p0.x + (t * s1.x);
    inter.y = p0.y + (t * s1.y);
}

/**
 * Whether two 2D vectors are equal. Does not consider possible false
 * negatives due to floating-point errors.
 * @param v1 First vector to compare
 * @param v2 Second vector to compare
 * @return Whether the vectors are equal
 */
pub fn _v2dEquals(v1: Vec2d, v2: Vec2d) -> bool {
    v1.x == v2.x && v1.y == v2.y
}

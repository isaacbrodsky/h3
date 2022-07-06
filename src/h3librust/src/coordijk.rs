use crate::constants::{M_SIN60, M_SQRT3_2};
use crate::h3api::CoordIJ;
use crate::vec2d::Vec2d;
use std::convert::TryFrom;

/** @struct CoordIJK
 * @brief IJK hexagon coordinates
 *
 * Each axis is spaced 120 degrees apart.
 */
#[derive(Copy, Clone)]
pub struct CoordIJK {
    /// i component
    pub i: i32,
    /// j component
    pub j: i32,
    /// k component
    pub k: i32,
}

/** @brief CoordIJK unit vectors corresponding to the 7 H3 digits.
 */
pub const UNIT_VECS: [CoordIJK; 7] = [
    CoordIJK { i: 0, j: 0, k: 0 }, // direction 0
    CoordIJK { i: 0, j: 0, k: 1 }, // direction 1
    CoordIJK { i: 0, j: 1, k: 0 }, // direction 2
    CoordIJK { i: 0, j: 1, k: 1 }, // direction 3
    CoordIJK { i: 1, j: 0, k: 0 }, // direction 4
    CoordIJK { i: 1, j: 0, k: 1 }, // direction 5
    CoordIJK { i: 1, j: 1, k: 0 }, // direction 6
];

/** @brief H3 digit representing ijk+ axes direction.
 * Values will be within the lowest 3 bits of an integer.
 */
#[repr(i8)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Direction {
    /** H3 digit in center */
    CENTER_DIGIT = 0,
    /** H3 digit in k-axes direction */
    K_AXES_DIGIT = 1,
    /** H3 digit in j-axes direction */
    J_AXES_DIGIT = 2,
    /** H3 digit in j == k direction */
    JK_AXES_DIGIT = 3, /* J_AXES_DIGIT | K_AXES_DIGIT */
    /** H3 digit in i-axes direction */
    I_AXES_DIGIT = 4,
    /** H3 digit in i == k direction */
    IK_AXES_DIGIT = 5, /* I_AXES_DIGIT | K_AXES_DIGIT */
    /** H3 digit in i == j direction */
    IJ_AXES_DIGIT = 6, /* I_AXES_DIGIT | J_AXES_DIGIT */
    /** H3 digit in the invalid direction */
    INVALID_DIGIT = 7,
}
// TODO: Implement trait so that Direction can be used to index into UNIT_VECS

pub const DIGITS: [Direction; 7] = [
    Direction::CENTER_DIGIT,
    Direction::K_AXES_DIGIT,
    Direction::J_AXES_DIGIT,
    Direction::JK_AXES_DIGIT,
    Direction::I_AXES_DIGIT,
    Direction::IK_AXES_DIGIT,
    Direction::IJ_AXES_DIGIT,
];

impl TryFrom<u64> for Direction {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::CENTER_DIGIT),
            1 => Ok(Direction::K_AXES_DIGIT),
            2 => Ok(Direction::J_AXES_DIGIT),
            3 => Ok(Direction::JK_AXES_DIGIT),
            4 => Ok(Direction::I_AXES_DIGIT),
            5 => Ok(Direction::IK_AXES_DIGIT),
            6 => Ok(Direction::IJ_AXES_DIGIT),
            7 => Ok(Direction::INVALID_DIGIT),
            _ => Err("Value out of range for direction digit"),
        }
    }
}

impl TryFrom<i8> for Direction {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::CENTER_DIGIT),
            1 => Ok(Direction::K_AXES_DIGIT),
            2 => Ok(Direction::J_AXES_DIGIT),
            3 => Ok(Direction::JK_AXES_DIGIT),
            4 => Ok(Direction::I_AXES_DIGIT),
            5 => Ok(Direction::IK_AXES_DIGIT),
            6 => Ok(Direction::IJ_AXES_DIGIT),
            7 => Ok(Direction::INVALID_DIGIT),
            _ => Err("Value out of range for direction digit"),
        }
    }
}

impl TryFrom<Direction> for usize {
    type Error = &'static str;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::CENTER_DIGIT => Ok(0),
            Direction::K_AXES_DIGIT => Ok(1),
            Direction::J_AXES_DIGIT => Ok(2),
            Direction::JK_AXES_DIGIT => Ok(3),
            Direction::I_AXES_DIGIT => Ok(4),
            Direction::IK_AXES_DIGIT => Ok(5),
            Direction::IJ_AXES_DIGIT => Ok(6),
            Direction::INVALID_DIGIT => Ok(7),
            _ => Err("Value out of range for direction digit"),
        }
    }
}

/** Valid digits will be less than this value. Same value as INVALID_DIGIT.
 */
pub const NUM_DIGITS: i8 = 7;

/** Child digit which is skipped for pentagons */
pub const PENTAGON_SKIPPED_DIGIT: Direction = Direction::K_AXES_DIGIT;

/**
 * Sets an IJK coordinate to the specified component values.
 *
 * @param ijk The IJK coordinate to set.
 * @param i The desired i component value.
 * @param j The desired j component value.
 * @param k The desired k component value.
 */
fn _setIJK(ijk: &mut CoordIJK, i: i32, j: i32, k: i32) {
    ijk.i = i;
    ijk.j = j;
    ijk.k = k;
}

/**
 * Determine the containing hex in ijk+ coordinates for a 2D cartesian
 * coordinate vector (from DGGRID).
 *
 * @param v The 2D cartesian coordinate vector.
 * @param h The ijk+ coordinates of the containing hex.
 */
pub fn _hex2dToCoordIJK(v: Vec2d, h: &mut CoordIJK) {
    // quantize into the ij system and then normalize
    h.k = 0;

    let a1: f64 = v.x.abs();
    let a2: f64 = v.y.abs();

    // first do a reverse conversion
    let x2: f64 = a2 / M_SIN60;
    let x1: f64 = a1 + x2 / 2.0;

    // check if we have the center of a hex
    let m1: i32 = x1 as i32;
    let m2: i32 = x2 as i32;

    // otherwise round correctly
    let r1: f64 = x1 - m1 as f64;
    let r2: f64 = x2 - m2 as f64;

    if r1 < 0.5 {
        if r1 < 1.0 / 3.0 {
            if r2 < (1.0 + r1) / 2.0 {
                h.i = m1;
                h.j = m2;
            } else {
                h.i = m1;
                h.j = m2 + 1;
            }
        } else {
            if r2 < (1.0 - r1) {
                h.j = m2;
            } else {
                h.j = m2 + 1;
            }

            if (1.0 - r1) <= r2 && r2 < (2.0 * r1) {
                h.i = m1 + 1;
            } else {
                h.i = m1;
            }
        }
    } else {
        if r1 < 2.0 / 3.0 {
            if r2 < (1.0 - r1) {
                h.j = m2;
            } else {
                h.j = m2 + 1;
            }

            if (2.0 * r1 - 1.0) < r2 && r2 < (1.0 - r1) {
                h.i = m1;
            } else {
                h.i = m1 + 1;
            }
        } else {
            if r2 < (r1 / 2.0) {
                h.i = m1 + 1;
                h.j = m2;
            } else {
                h.i = m1 + 1;
                h.j = m2 + 1;
            }
        }
    }

    // now fold across the axes if necessary

    if v.x < 0.0 {
        if (h.j % 2) == 0
        // even
        {
            let axisi: i64 = (h.j / 2).into();
            let diff: i64 = h.i as i64 - axisi;
            h.i = ((h.i as f64) - 2.0 * (diff as f64)) as i32;
        } else {
            let axisi: i64 = ((h.j + 1) / 2).into();
            let diff: i64 = h.i as i64 - axisi;
            h.i = ((h.i as f64) - (2.0 * (diff as f64) + 1.0)) as i32;
        }
    }

    if v.y < 0.0 {
        h.i = h.i - (2 * h.j + 1) / 2;
        h.j = -1 * h.j;
    }

    _ijkNormalize(h);
}

/**
 * Find the center point in 2D cartesian coordinates of a hex.
 *
 * @param h The ijk coordinates of the hex.
 * @param v The 2D cartesian coordinates of the hex center point.
 */
pub fn _ijkToHex2d(h: CoordIJK, v: &mut Vec2d) {
    let i = h.i - h.k;
    let j = h.j - h.k;

    v.x = i as f64 - 0.5 * j as f64;
    v.y = j as f64 * M_SQRT3_2;
}

/**
 * Returns whether or not two ijk coordinates contain exactly the same
 * component values.
 *
 * @param c1 The first set of ijk coordinates.
 * @param c2 The second set of ijk coordinates.
 * @return 1 if the two addresses match, 0 if they do not.
 */
fn _ijkMatches(c1: CoordIJK, c2: CoordIJK) -> bool {
    c1.i == c2.i && c1.j == c2.j && c1.k == c2.k
}

/**
 * Add two ijk coordinates.
 *
 * @param h1 The first set of ijk coordinates.
 * @param h2 The second set of ijk coordinates.
 * @param sum The sum of the two sets of ijk coordinates.
 */
pub fn _ijkAdd(h1: CoordIJK, h2: CoordIJK, sum: &mut CoordIJK) {
    sum.i = h1.i + h2.i;
    sum.j = h1.j + h2.j;
    sum.k = h1.k + h2.k;
}

/**
 * Subtract two ijk coordinates.
 *
 * @param h1 The first set of ijk coordinates.
 * @param h2 The second set of ijk coordinates.
 * @param diff The difference of the two sets of ijk coordinates (h1 - h2).
 */
pub fn _ijkSub(h1: CoordIJK, h2: CoordIJK, diff: &mut CoordIJK) {
    diff.i = h1.i - h2.i;
    diff.j = h1.j - h2.j;
    diff.k = h1.k - h2.k;
}

/**
 * Uniformly scale ijk coordinates by a scalar. Works in place.
 *
 * @param c The ijk coordinates to scale.
 * @param factor The scaling factor.
 */
pub fn _ijkScale(c: &mut CoordIJK, factor: i32) {
    c.i *= factor;
    c.j *= factor;
    c.k *= factor;
}

/**
 * Normalizes ijk coordinates by setting the components to the smallest possible
 * values. Works in place.
 *
 * @param c The ijk coordinates to normalize.
 */
pub fn _ijkNormalize(c: &mut CoordIJK) {
    // remove any negative values
    if c.i < 0 {
        c.j -= c.i;
        c.k -= c.i;
        c.i = 0;
    }

    if c.j < 0 {
        c.i -= c.j;
        c.k -= c.j;
        c.j = 0;
    }

    if c.k < 0 {
        c.i -= c.k;
        c.j -= c.k;
        c.k = 0;
    }

    // remove the min value if needed
    let mut min = c.i;
    if c.j < min {
        min = c.j;
    }
    if c.k < min {
        min = c.k;
    }
    if min > 0 {
        c.i -= min;
        c.j -= min;
        c.k -= min;
    }
}

/**
 * Determines the H3 digit corresponding to a unit vector in ijk coordinates.
 *
 * @param ijk The ijk coordinates; must be a unit vector.
 * @return The H3 digit (0-6) corresponding to the ijk unit vector, or
 * INVALID_DIGIT on failure.
 */
pub fn _unitIjkToDigit(ijk: CoordIJK) -> Direction {
    for i in DIGITS {
        if _ijkMatches(ijk, UNIT_VECS[i as usize]) {
            return i;
        }
    }
    return Direction::INVALID_DIGIT;
}

/**
 * Find the normalized ijk coordinates of the indexing parent of a cell in a
 * counter-clockwise aperture 7 grid. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _upAp7(ijk: &mut CoordIJK) {
    // convert to CoordIJ
    let i = ijk.i - ijk.k;
    let j = ijk.j - ijk.k;

    ijk.i = ((3 * i - j) as f64 / 7.0).round() as i32;
    ijk.j = ((i + 2 * j) as f64 / 7.0).round() as i32;
    ijk.k = 0;
    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the indexing parent of a cell in a
 * clockwise aperture 7 grid. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _upAp7r(ijk: &mut CoordIJK) {
    // convert to CoordIJ
    let i = ijk.i - ijk.k;
    let j = ijk.j - ijk.k;

    ijk.i = ((2 * i + j) as f64 / 7.0).round() as i32;
    ijk.j = ((3 * j - i) as f64 / 7.0).round() as i32;
    ijk.k = 0;
    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 7 counter-clockwise resolution. Works in
 * place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _downAp7(ijk: &mut CoordIJK) {
    // res r unit vectors in res r+1
    let mut iVec = CoordIJK { i: 3, j: 0, k: 1 };
    let mut jVec = CoordIJK { i: 1, j: 3, k: 0 };
    let mut kVec = CoordIJK { i: 0, j: 1, k: 3 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 7 clockwise resolution. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _downAp7r(ijk: &mut CoordIJK) {
    // res r unit vectors in res r+1
    let mut iVec = CoordIJK { i: 3, j: 1, k: 0 };
    let mut jVec = CoordIJK { i: 0, j: 3, k: 1 };
    let mut kVec = CoordIJK { i: 1, j: 0, k: 3 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex in the specified digit
 * direction from the specified ijk coordinates. Works in place.
 *
 * @param ijk The ijk coordinates.
 * @param digit The digit direction from the original ijk coordinates.
 */
pub fn _neighbor(ijk: &mut CoordIJK, digit: Direction) {
    if digit > Direction::CENTER_DIGIT && digit < Direction::INVALID_DIGIT {
        _ijkAdd(*ijk, UNIT_VECS[digit as usize], ijk);
        _ijkNormalize(ijk);
    }
}

/**
 * Rotates ijk coordinates 60 degrees counter-clockwise. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _ijkRotate60ccw(ijk: &mut CoordIJK) {
    // unit vector rotations
    let mut iVec = CoordIJK { i: 1, j: 1, k: 0 };
    let mut jVec = CoordIJK { i: 0, j: 1, k: 1 };
    let mut kVec = CoordIJK { i: 1, j: 0, k: 1 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Rotates ijk coordinates 60 degrees clockwise. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _ijkRotate60cw(ijk: &mut CoordIJK) {
    // unit vector rotations
    let mut iVec = CoordIJK { i: 1, j: 0, k: 1 };
    let mut jVec = CoordIJK { i: 1, j: 1, k: 0 };
    let mut kVec = CoordIJK { i: 0, j: 1, k: 1 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Rotates indexing digit 60 degrees counter-clockwise. Returns result.
 *
 * @param digit Indexing digit (between 1 and 6 inclusive)
 */
pub fn _rotate60ccw(digit: Direction) -> Direction {
    match digit {
        Direction::CENTER_DIGIT => Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT => Direction::IK_AXES_DIGIT,
        Direction::IK_AXES_DIGIT => Direction::I_AXES_DIGIT,
        Direction::I_AXES_DIGIT => Direction::IJ_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT => Direction::J_AXES_DIGIT,
        Direction::J_AXES_DIGIT => Direction::JK_AXES_DIGIT,
        Direction::JK_AXES_DIGIT => Direction::K_AXES_DIGIT,
        _ => panic!("invalid digit"),
    }
}

/**
 * Rotates indexing digit 60 degrees clockwise. Returns result.
 *
 * @param digit Indexing digit (between 1 and 6 inclusive)
 */
pub fn _rotate60cw(digit: Direction) -> Direction {
    match digit {
        Direction::CENTER_DIGIT => Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT => Direction::JK_AXES_DIGIT,
        Direction::JK_AXES_DIGIT => Direction::J_AXES_DIGIT,
        Direction::J_AXES_DIGIT => Direction::IJ_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT => Direction::I_AXES_DIGIT,
        Direction::I_AXES_DIGIT => Direction::IK_AXES_DIGIT,
        Direction::IK_AXES_DIGIT => Direction::K_AXES_DIGIT,
        _ => panic!("invalid digit"),
    }
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 3 counter-clockwise resolution. Works in
 * place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _downAp3(ijk: &mut CoordIJK) {
    // res r unit vectors in res r+1
    let mut iVec = CoordIJK { i: 2, j: 0, k: 1 };
    let mut jVec = CoordIJK { i: 1, j: 2, k: 0 };
    let mut kVec = CoordIJK { i: 0, j: 1, k: 2 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 3 clockwise resolution. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
pub fn _downAp3r(ijk: &mut CoordIJK) {
    // res r unit vectors in res r+1
    let mut iVec = CoordIJK { i: 2, j: 1, k: 0 };
    let mut jVec = CoordIJK { i: 0, j: 2, k: 1 };
    let mut kVec = CoordIJK { i: 1, j: 0, k: 2 };

    _ijkScale(&mut iVec, ijk.i);
    _ijkScale(&mut jVec, ijk.j);
    _ijkScale(&mut kVec, ijk.k);

    _ijkAdd(iVec, jVec, ijk);
    _ijkAdd(*ijk, kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Finds the distance between the two coordinates. Returns result.
 *
 * @param c1 The first set of ijk coordinates.
 * @param c2 The second set of ijk coordinates.
 */
fn ijkDistance(c1: CoordIJK, c2: CoordIJK) -> i32 {
    let mut diff = CoordIJK { i: 0, j: 0, k: 0 };
    _ijkSub(c1, c2, &mut diff);
    _ijkNormalize(&mut diff);
    let absDiff = CoordIJK {
        i: diff.i.abs(),
        j: diff.j.abs(),
        k: diff.k.abs(),
    };
    absDiff.i.max(absDiff.j).max(absDiff.k)
}

/**
 * Transforms coordinates from the IJK+ coordinate system to the IJ coordinate
 * system.
 *
 * @param ijk The input IJK+ coordinates
 * @param ij The output IJ coordinates
 */
fn ijkToIj(ijk: CoordIJK, ij: &mut CoordIJ) {
    ij.i = ijk.i - ijk.k;
    ij.j = ijk.j - ijk.k;
}

/**
 * Transforms coordinates from the IJ coordinate system to the IJK+ coordinate
 * system.
 *
 * @param ij The input IJ coordinates
 * @param ijk The output IJK+ coordinates
 */
fn ijToIjk(ij: CoordIJ, ijk: &mut CoordIJK) {
    ijk.i = ij.i;
    ijk.j = ij.j;
    ijk.k = 0;

    _ijkNormalize(ijk);
}

/**
 * Convert IJK coordinates to cube coordinates, in place
 * @param ijk Coordinate to convert
 */
fn ijkToCube(ijk: &mut CoordIJK) {
    ijk.i = -ijk.i + ijk.k;
    ijk.j = ijk.j - ijk.k;
    ijk.k = -ijk.i - ijk.j;
}

/**
 * Convert cube coordinates to IJK coordinates, in place
 * @param ijk Coordinate to convert
 */
fn cubeToIjk(ijk: &mut CoordIJK) {
    ijk.i = -ijk.i;
    ijk.k = 0;
    _ijkNormalize(ijk);
}

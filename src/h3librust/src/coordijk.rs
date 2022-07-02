use std::convert::TryFrom;

/** @struct CoordIJK
 * @brief IJK hexagon coordinates
 *
 * Each axis is spaced 120 degrees apart.
 */
pub struct CoordIJK {
    /// i component
    i: i32,
    /// j component
    j: i32,
    /// k component
    k: i32, 
 }

/** @brief CoordIJK unit vectors corresponding to the 7 H3 digits.
 */
pub const UNIT_VECS: [CoordIJK; 7] = [
    CoordIJK { i: 0, j: 0, k: 0 },  // direction 0
    CoordIJK { i: 0, j: 0, k: 1 },  // direction 1
    CoordIJK { i: 0, j: 1, k: 0 },  // direction 2
    CoordIJK { i: 0, j: 1, k: 1 },  // direction 3
    CoordIJK { i: 1, j: 0, k: 0 },  // direction 4
    CoordIJK { i: 1, j: 0, k: 1 },  // direction 5
    CoordIJK { i: 1, j: 1, k: 0 }   // direction 6
];

/** @brief H3 digit representing ijk+ axes direction.
 * Values will be within the lowest 3 bits of an integer.
 */
#[repr(i8)]
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

/** Valid digits will be less than this value. Same value as INVALID_DIGIT.
    */
pub const NUM_DIGITS: i8 = 7;

    /** Child digit which is skipped for pentagons */
pub const PENTAGON_SKIPPED_DIGIT: Direction = Direction::K_AXES_DIGIT;
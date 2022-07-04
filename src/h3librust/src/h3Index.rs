use crate::constants::MAX_H3_RES;
use crate::coordijk::Direction;
use crate::h3api::H3Error;

// The number of bits in an H3 index.
const H3_NUM_BITS: i8 = 64;

// The bit offset of the max resolution digit in an H3 index.
const H3_MAX_OFFSET: i8 = 63;

// The bit offset of the mode in an H3 index.
const H3_MODE_OFFSET: i8 = 59;

// The bit offset of the base cell in an H3 index.
const H3_BC_OFFSET: i8 = 45;

// The bit offset of the resolution in an H3 index.
const H3_RES_OFFSET: i8 = 52;

// The bit offset of the reserved bits in an H3 index.
const H3_RESERVED_OFFSET: i8 = 56;

// The number of bits in a single H3 resolution digit.
const H3_PER_DIGIT_OFFSET: i8 = 3;

// 1 in the highest bit, 0's everywhere else.
const H3_HIGH_BIT_MASK: u64 = 1u64 << H3_MAX_OFFSET;

// 0 in the highest bit, 1's everywhere else.
const H3_HIGH_BIT_MASK_NEGATIVE: u64 = !H3_HIGH_BIT_MASK;

// 1's in the 4 mode bits, 0's everywhere else.
const H3_MODE_MASK: u64 = 15u64 << H3_MODE_OFFSET;

// 0's in the 4 mode bits, 1's everywhere else.
const H3_MODE_MASK_NEGATIVE: u64 = !H3_MODE_MASK;

// 1's in the 7 base cell bits, 0's everywhere else.
const H3_BC_MASK: u64 = 127u64 << H3_BC_OFFSET;

// 0's in the 7 base cell bits, 1's everywhere else.
const H3_BC_MASK_NEGATIVE: u64 = !H3_BC_MASK;

// 1's in the 4 resolution bits, 0's everywhere else.
const H3_RES_MASK: u64 = 15u64 << H3_RES_OFFSET;

// 0's in the 4 resolution bits, 1's everywhere else.
const H3_RES_MASK_NEGATIVE: u64 = !H3_RES_MASK;

// 1's in the 3 reserved bits, 0's everywhere else.
const H3_RESERVED_MASK: u64 = 7u64 << H3_RESERVED_OFFSET;

// 0's in the 3 reserved bits, 1's everywhere else.
const H3_RESERVED_MASK_NEGATIVE: u64 = !H3_RESERVED_MASK;

// 1's in the 3 bits of res 15 digit bits, 0's everywhere else.
const H3_DIGIT_MASK: u64 = 7u64;

// 0's in the 7 base cell bits, 1's everywhere else.
const H3_DIGIT_MASK_NEGATIVE: u64 = !H3_DIGIT_MASK;

// H3 index with mode 0, res 0, base cell 0, and 7 for all index digits.
// Typically used to initialize the creation of an H3 cell index, which
// expects all direction digits to be 7 beyond the cell's resolution.
pub const H3_INIT: u64 = 35184372088831u64;

/**
 * Gets the highest bit of the H3 index.
 */
pub fn H3_GET_HIGH_BIT(h3: u64) -> i8 {
    ((h3 & H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET)
        .try_into()
        .unwrap()
}

/**
 * Sets the highest bit of the h3 to v.
 */
pub fn H3_SET_HIGH_BIT(h3: u64, v: i8) -> u64 {
    (h3 & H3_HIGH_BIT_MASK_NEGATIVE) | ((v as u64) << H3_MAX_OFFSET)
}

/**
 * Gets the integer mode of h3.
 */
pub fn H3_GET_MODE(h3: u64) -> i8 {
    ((h3 & H3_MODE_MASK) >> H3_MODE_OFFSET).try_into().unwrap()
}

/**
 * Sets the integer mode of h3 to v.
 */
pub fn H3_SET_MODE(h3: u64, v: i8) -> u64 {
    (h3 & H3_MODE_MASK_NEGATIVE) | ((v as u64) << H3_MODE_OFFSET)
}

/**
 * Gets the integer base cell of h3.
 */
pub fn H3_GET_BASE_CELL(h3: u64) -> i8 {
    ((h3 & H3_BC_MASK) >> H3_BC_OFFSET).try_into().unwrap()
}

/**
 * Sets the integer base cell of h3 to bc.
 */
pub fn H3_SET_BASE_CELL(h3: u64, bc: i8) -> u64 {
    (h3 & H3_BC_MASK_NEGATIVE) | ((bc as u64) << H3_BC_OFFSET)
}

/**
 * Gets the integer resolution of h3.
 */
pub fn H3_GET_RESOLUTION(h3: u64) -> i8 {
    ((h3 & H3_RES_MASK) >> H3_RES_OFFSET).try_into().unwrap()
}

/**
 * Sets the integer resolution of h3.
 */
pub fn H3_SET_RESOLUTION(h3: u64, res: i8) -> u64 {
    (h3 & H3_RES_MASK_NEGATIVE) | ((res as u64) << H3_RES_OFFSET)
}

/**
 * Gets the resolution res integer digit (0-7) of h3.
 */
pub fn H3_GET_INDEX_DIGIT(h3: u64, res: i8) -> Direction {
    ((h3 >> ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)) & H3_DIGIT_MASK)
        .try_into()
        .unwrap()
}

/**
 * Sets a value in the reserved space. Setting to non-zero may produce invalid
 * indexes.
 */
pub fn H3_SET_RESERVED_BITS(h3: u64, v: i8) -> u64 {
    (h3 & H3_RESERVED_MASK_NEGATIVE) | ((v as u64) << H3_RESERVED_OFFSET)
}

/**
 * Gets a value in the reserved space. Should always be zero for valid indexes.
 */
pub fn H3_GET_RESERVED_BITS(h3: u64) -> u64 {
    ((h3 & H3_RESERVED_MASK) >> H3_RESERVED_OFFSET)
        .try_into()
        .unwrap()
}

/**
 * Sets the resolution res digit of h3 to the integer digit (0-7)
 */
pub fn H3_SET_INDEX_DIGIT(h3: u64, res: i8, digit: Direction) -> u64 {
    h3 & !(H3_DIGIT_MASK << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET))
        | ((digit as u64) << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET))
}

pub fn cellToParent(h: u64, parentRes: i8, out: &mut u64) -> H3Error {
    let childRes = H3_GET_RESOLUTION(h);
    if parentRes < 0 || parentRes > MAX_H3_RES {
        return H3Error::E_RES_DOMAIN;
    } else if parentRes > childRes {
        return H3Error::E_RES_MISMATCH;
    } else if parentRes == childRes {
        *out = h;
        return H3Error::E_SUCCESS;
    }
    let mut parentH = H3_SET_RESOLUTION(h, parentRes);
    for i in parentRes + 1..childRes + 1 {
        parentH = H3_SET_INDEX_DIGIT(parentH, i, Direction::INVALID_DIGIT);
    }
    *out = parentH;
    return H3Error::E_SUCCESS;
}

/**
 * Returns whether or not a resolution is a Class III grid. Note that odd
 * resolutions are Class III and even resolutions are Class II.
 * @param res The H3 resolution.
 * @return 1 if the resolution is a Class III grid, and 0 if the resolution is
 *         a Class II grid.
 */
pub fn isResolutionClassIII(res: i8) -> bool {
    res % 2 == 1
}

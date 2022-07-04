use crate::baseCells::{
    _baseCellIsCwOffset, _faceIjkToBaseCell, _faceIjkToBaseCellCCWrot60, _isBaseCellPentagon,
    MAX_FACE_COORD,
};
use crate::constants::{H3_CELL_MODE, MAX_H3_RES, NUM_BASE_CELLS};
use crate::coordijk::{
    CoordIJK, Direction, _downAp7, _downAp7r, _ijkNormalize, _ijkSub, _rotate60ccw, _rotate60cw,
    _unitIjkToDigit, _upAp7, _upAp7r,
};
use crate::faceijk::{FaceIJK, _geoToFaceIjk};
use crate::h3api::{H3Error, LatLng, H3_NULL};

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

/**
 * Returns the H3 resolution of an H3 index.
 * @param h The H3 index.
 * @return The resolution of the H3 index argument.
 */
pub fn getResolution(h: u64) -> i32 {
    H3_GET_RESOLUTION(h) as i32
}

/**
 * Returns the H3 base cell "number" of an H3 cell (hexagon or pentagon).
 *
 * Note: Technically works on H3 edges, but will return base cell of the
 * origin cell.
 *
 * @param h The H3 cell.
 * @return The base cell "number" of the H3 cell argument.
 */
pub fn getBaseCellNumber(h: u64) -> i32 {
    H3_GET_BASE_CELL(h) as i32
}

/**
 * Converts a string representation of an H3 index into an H3 index.
 * @param str The string representation of an H3 index.
 * @return The H3 index corresponding to the string argument, or H3_NULL if
 * invalid.
 */
//  H3Error H3_EXPORT(stringToH3)(const char *str, H3Index *out) {
//      H3Index h = H3_NULL;
//      // If failed, h will be unmodified and we should return H3_NULL anyways.
//      int read = sscanf(str, "%" PRIx64, &h);
//      if (read != 1) {
//          return E_FAILED;
//      }
//      *out = h;
//      return E_SUCCESS;
//  }

/**
 * Converts an H3 index into a string representation.
 * @param h The H3 index to convert.
 * @param str The string representation of the H3 index.
 * @param sz Size of the buffer `str`
 */
//  H3Error H3_EXPORT(h3ToString)(H3Index h, char *str, size_t sz) {
//      // An unsigned 64 bit integer will be expressed in at most
//      // 16 digits plus 1 for the null terminator.
//      if (sz < 17) {
//          // Buffer is potentially not large enough.
//          return E_MEMORY_BOUNDS;
//      }
//      sprintf(str, "%" PRIx64, h);
//      return E_SUCCESS;
//  }

/**
 * Returns whether or not an H3 index is a valid cell (hexagon or pentagon).
 * @param h The H3 index to validate.
 * @return 1 if the H3 index if valid, and 0 if it is not.
 */
pub fn isValidCell(h: u64) -> i32 {
    if H3_GET_HIGH_BIT(h) != 0 {
        return 0;
    }

    if H3_GET_MODE(h) != H3_CELL_MODE {
        return 0;
    }

    if H3_GET_RESERVED_BITS(h) != 0 {
        return 0;
    }

    let baseCell: i8 = H3_GET_BASE_CELL(h);
    if baseCell < 0 || baseCell >= NUM_BASE_CELLS {
        // Base cells less than zero can not be represented in an index
        return 0;
    }

    let res = H3_GET_RESOLUTION(h);
    if res < 0 || res > MAX_H3_RES {
        // Resolutions less than zero can not be represented in an index
        return 0;
    }

    let mut foundFirstNonZeroDigit = false;
    for r in 1..=res {
        let digit = H3_GET_INDEX_DIGIT(h, r);

        if !foundFirstNonZeroDigit && digit != Direction::CENTER_DIGIT {
            foundFirstNonZeroDigit = true;
            if _isBaseCellPentagon(baseCell) && digit == Direction::K_AXES_DIGIT {
                return 0;
            }
        }

        if digit < Direction::CENTER_DIGIT || digit >= Direction::INVALID_DIGIT {
            return 0;
        }
    }

    for r in res + 1..MAX_H3_RES {
        let digit = H3_GET_INDEX_DIGIT(h, r);
        if digit != Direction::INVALID_DIGIT {
            return 0;
        }
    }

    1
}

/**
 * Initializes an H3 index.
 * @param hp The H3 index to initialize.
 * @param res The H3 resolution to initialize the index to.
 * @param baseCell The H3 base cell to initialize the index to.
 * @param initDigit The H3 digit (0-7) to initialize all of the index digits to.
 */
fn setH3Index(hp: &mut u64, res: i8, baseCell: i8, initDigit: Direction) {
    let mut h = H3_INIT;
    h = H3_SET_MODE(h, H3_CELL_MODE);
    h = H3_SET_RESOLUTION(h, res);
    h = H3_SET_BASE_CELL(h, baseCell);
    for r in 1..=res {
        H3_SET_INDEX_DIGIT(h, r, initDigit);
    }
    *hp = h;
}

/**
 * cellToParent produces the parent index for a given H3 index
 *
 * @param h H3Index to find parent of
 * @param parentRes The resolution to switch to (parent, grandparent, etc)
 *
 * @return H3Index of the parent, or H3_NULL if you actually asked for a child
 */
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
    for i in parentRes + 1..=childRes {
        parentH = H3_SET_INDEX_DIGIT(parentH, i, Direction::INVALID_DIGIT);
    }
    *out = parentH;
    return H3Error::E_SUCCESS;
}

/**
 * Determines whether one resolution is a valid child resolution for a cell.
 * Each resolution is considered a valid child resolution of itself.
 *
 * @param h         h3Index  parent cell
 * @param childRes  int      resolution of the child
 *
 * @return The validity of the child resolution
 */
fn _hasChildAtRes(h: u64, childRes: i8) -> bool {
    let parentRes = H3_GET_RESOLUTION(h);
    if childRes < parentRes || childRes > MAX_H3_RES {
        return false;
    }
    true
}

/**
 * cellToChildrenSize returns the exact number of children for a cell at a
 * given child resolution.
 *
 * @param h         H3Index to find the number of children of
 * @param childRes  The child resolution you're interested in
 *
 * @return int      Exact number of children (handles hexagons and pentagons
 *                  correctly)
 */
pub fn cellToChildrenSize(h: u64, childRes: i32, out: &mut u64) -> H3Error {
    if !_hasChildAtRes(h, childRes.try_into().unwrap()) {
        return H3Error::E_RES_DOMAIN;
    }

    let childResU64: u64 = childRes.try_into().unwrap();
    let n: u64 = childResU64 - H3_GET_RESOLUTION(h) as u64;

    if isPentagon(h) == 1 {
        *out = 1u64 + 5u64 * (7u64.pow(n.try_into().unwrap()) - 1u64) / 6u64;
    } else {
        *out = 7u64.pow(n.try_into().unwrap());
    }
    H3Error::E_SUCCESS
}

/**
 * makeDirectChild takes an index and immediately returns the immediate child
 * index based on the specified cell number. Bit operations only, could generate
 * invalid indexes if not careful (deleted cell under a pentagon).
 *
 * @param h H3Index to find the direct child of
 * @param cellNumber int id of the direct child (0-6)
 *
 * @return The new H3Index for the child
 */
fn makeDirectChild(h: u64, cellNumber: i8) -> u64 {
    let childRes = H3_GET_RESOLUTION(h) + 1;
    let mut childH = H3_SET_RESOLUTION(h, childRes);
    childH = H3_SET_INDEX_DIGIT(childH, childRes, cellNumber.try_into().unwrap());
    childH
}

/**
 * cellToChildren takes the given hexagon id and generates all of the children
 * at the specified resolution storing them into the provided memory pointer.
 * It's assumed that cellToChildrenSize was used to determine the allocation.
 *
 * @param h H3Index to find the children of
 * @param childRes int the child level to produce
 * @param children H3Index* the memory to store the resulting addresses in
 */
//  H3Error H3_EXPORT(cellToChildren)(H3Index h, int childRes, H3Index *children) {
//      int64_t i = 0;
//      for (IterCellsChildren iter = iterInitParent(h, childRes); iter.h;
//           iterStepChild(&iter)) {
//          children[i] = iter.h;
//          i++;
//      }
//      return E_SUCCESS;
//  }

/**
 * Zero out index digits from start to end, inclusive.
 * No-op if start > end.
 */
fn _zeroIndexDigits(h: u64, start: i8, end: i8) -> u64 {
    if start > end {
        return h;
    }

    let mut m = 0u64;

    m = !m;
    m <<= H3_PER_DIGIT_OFFSET * (end - start + 1);
    m = !m;
    m <<= H3_PER_DIGIT_OFFSET * (MAX_H3_RES - end);
    m = !m;

    h & m
}

/**
 * cellToCenterChild produces the center child index for a given H3 index at
 * the specified resolution
 *
 * @param h H3Index to find center child of
 * @param childRes The resolution to switch to
 * @param child H3Index of the center child
 * @return 0 (E_SUCCESS) on success
 */
pub fn cellToCenterChild(h: u64, childRes: i32, child: &mut u64) -> H3Error {
    if !_hasChildAtRes(h, childRes.try_into().unwrap()) {
        return H3Error::E_RES_DOMAIN;
    }

    let mut childH = _zeroIndexDigits(h, H3_GET_RESOLUTION(h) + 1, childRes.try_into().unwrap());
    childH = H3_SET_RESOLUTION(h, childRes.try_into().unwrap());
    *child = childH;
    H3Error::E_SUCCESS
}

/**
 * compactCells takes a set of hexagons all at the same resolution and
 * compresses them by pruning full child branches to the parent level. This is
 * also done for all parents recursively to get the minimum number of hex
 * addresses that perfectly cover the defined space.
 * @param h3Set Set of hexagons
 * @param compactedSet The output array of compressed hexagons (preallocated)
 * @param numHexes The size of the input and output arrays (possible that no
 * contiguous regions exist in the set at all and no compression possible)
 * @return an error code on bad input data
 */
// todo: update internal implementation for int64_t
//  H3Error H3_EXPORT(compactCells)(const H3Index *h3Set, H3Index *compactedSet,
//                                  const int64_t numHexes) {
//      if (numHexes == 0) {
//          return E_SUCCESS;
//      }
//      int res = H3_GET_RESOLUTION(h3Set[0]);
//      if (res == 0) {
//          // No compaction possible, just copy the set to output
//          for (int i = 0; i < numHexes; i++) {
//              compactedSet[i] = h3Set[i];
//          }
//          return E_SUCCESS;
//      }
//      H3Index *remainingHexes = H3_MEMORY(malloc)(numHexes * sizeof(H3Index));
//      if (!remainingHexes) {
//          return E_MEMORY;
//      }
//      memcpy(remainingHexes, h3Set, numHexes * sizeof(H3Index));
//      H3Index *hashSetArray = H3_MEMORY(calloc)(numHexes, sizeof(H3Index));
//      if (!hashSetArray) {
//          H3_MEMORY(free)(remainingHexes);
//          return E_MEMORY;
//      }
//      H3Index *compactedSetOffset = compactedSet;
//      int numRemainingHexes = numHexes;
//      while (numRemainingHexes) {
//          res = H3_GET_RESOLUTION(remainingHexes[0]);
//          int parentRes = res - 1;
//          // Put the parents of the hexagons into the temp array
//          // via a hashing mechanism, and use the reserved bits
//          // to track how many times a parent is duplicated
//          for (int i = 0; i < numRemainingHexes; i++) {
//              H3Index currIndex = remainingHexes[i];
//              if (currIndex != 0) {
//                  // If the reserved bits were set by the caller, the
//                  // algorithm below may encounter undefined behavior
//                  // because it expects to have set the reserved bits
//                  // itself.
//                  if (H3_GET_RESERVED_BITS(currIndex) != 0) {
//                      H3_MEMORY(free)(remainingHexes);
//                      H3_MEMORY(free)(hashSetArray);
//                      return E_CELL_INVALID;
//                  }

//                  H3Index parent;
//                  H3Error parentError =
//                      H3_EXPORT(cellToParent)(currIndex, parentRes, &parent);
//                  // Should never be reachable as a result of the compact
//                  // algorithm. Can happen if cellToParent errors e.g.
//                  // because of incompatible resolutions.
//                  if (parentError) {
//                      H3_MEMORY(free)(remainingHexes);
//                      H3_MEMORY(free)(hashSetArray);
//                      return parentError;
//                  }
//                  // Modulus hash the parent into the temp array
//                  int loc = (int)(parent % numRemainingHexes);
//                  int loopCount = 0;
//                  while (hashSetArray[loc] != 0) {
//                      if (loopCount > numRemainingHexes) {  // LCOV_EXCL_BR_LINE
//                          // LCOV_EXCL_START
//                          // This case should not be possible because at most one
//                          // index is placed into hashSetArray per
//                          // numRemainingHexes.
//                          H3_MEMORY(free)(remainingHexes);
//                          H3_MEMORY(free)(hashSetArray);
//                          return E_FAILED;
//                          // LCOV_EXCL_STOP
//                      }
//                      H3Index tempIndex =
//                          hashSetArray[loc] & H3_RESERVED_MASK_NEGATIVE;
//                      if (tempIndex == parent) {
//                          int count = H3_GET_RESERVED_BITS(hashSetArray[loc]) + 1;
//                          int limitCount = 7;
//                          if (H3_EXPORT(isPentagon)(tempIndex &
//                                                    H3_RESERVED_MASK_NEGATIVE)) {
//                              limitCount--;
//                          }
//                          // One is added to count for this check to match one
//                          // being added to count later in this function when
//                          // checking for all children being present.
//                          if (count + 1 > limitCount) {
//                              // Only possible on duplicate input
//                              H3_MEMORY(free)(remainingHexes);
//                              H3_MEMORY(free)(hashSetArray);
//                              return E_DUPLICATE_INPUT;
//                          }
//                          H3_SET_RESERVED_BITS(parent, count);
//                          hashSetArray[loc] = H3_NULL;
//                      } else {
//                          loc = (loc + 1) % numRemainingHexes;
//                      }
//                      loopCount++;
//                  }
//                  hashSetArray[loc] = parent;
//              }
//          }
//          // Determine which parent hexagons have a complete set
//          // of children and put them in the compactableHexes array
//          int compactableCount = 0;
//          int maxCompactableCount =
//              numRemainingHexes / 6;  // Somehow all pentagons; conservative
//          if (maxCompactableCount == 0) {
//              memcpy(compactedSetOffset, remainingHexes,
//                     numRemainingHexes * sizeof(remainingHexes[0]));
//              break;
//          }
//          H3Index *compactableHexes =
//              H3_MEMORY(calloc)(maxCompactableCount, sizeof(H3Index));
//          if (!compactableHexes) {
//              H3_MEMORY(free)(remainingHexes);
//              H3_MEMORY(free)(hashSetArray);
//              return E_MEMORY;
//          }
//          for (int i = 0; i < numRemainingHexes; i++) {
//              if (hashSetArray[i] == 0) continue;
//              int count = H3_GET_RESERVED_BITS(hashSetArray[i]) + 1;
//              // Include the deleted direction for pentagons as implicitly "there"
//              if (H3_EXPORT(isPentagon)(hashSetArray[i] &
//                                        H3_RESERVED_MASK_NEGATIVE)) {
//                  // We need this later on, no need to recalculate
//                  H3_SET_RESERVED_BITS(hashSetArray[i], count);
//                  // Increment count after setting the reserved bits,
//                  // since count is already incremented above, so it
//                  // will be the expected value for a complete hexagon.
//                  count++;
//              }
//              if (count == 7) {
//                  // Bingo! Full set!
//                  compactableHexes[compactableCount] =
//                      hashSetArray[i] & H3_RESERVED_MASK_NEGATIVE;
//                  compactableCount++;
//              }
//          }
//          // Uncompactable hexes are immediately copied into the
//          // output compactedSetOffset
//          int uncompactableCount = 0;
//          for (int i = 0; i < numRemainingHexes; i++) {
//              H3Index currIndex = remainingHexes[i];
//              if (currIndex != H3_NULL) {
//                  H3Index parent;
//                  H3Error parentError =
//                      H3_EXPORT(cellToParent)(currIndex, parentRes, &parent);
//                  // LCOV_EXCL_START
//                  // Should never be reachable as a result of the compact
//                  // algorithm.
//                  if (parentError) {
//                      // TODO: Determine if this is somehow reachable.
//                      H3_MEMORY(free)(remainingHexes);
//                      H3_MEMORY(free)(hashSetArray);
//                      return parentError;
//                  }
//                  // LCOV_EXCL_STOP
//                  // Modulus hash the parent into the temp array
//                  // to determine if this index was included in
//                  // the compactableHexes array
//                  int loc = (int)(parent % numRemainingHexes);
//                  int loopCount = 0;
//                  bool isUncompactable = true;
//                  do {
//                      if (loopCount > numRemainingHexes) {  // LCOV_EXCL_BR_LINE
//                          // LCOV_EXCL_START
//                          // This case should not be possible because at most one
//                          // index is placed into hashSetArray per input hexagon.
//                          H3_MEMORY(free)(compactableHexes);
//                          H3_MEMORY(free)(remainingHexes);
//                          H3_MEMORY(free)(hashSetArray);
//                          return E_FAILED;
//                          // LCOV_EXCL_STOP
//                      }
//                      H3Index tempIndex =
//                          hashSetArray[loc] & H3_RESERVED_MASK_NEGATIVE;
//                      if (tempIndex == parent) {
//                          int count = H3_GET_RESERVED_BITS(hashSetArray[loc]) + 1;
//                          if (count == 7) {
//                              isUncompactable = false;
//                          }
//                          break;
//                      } else {
//                          loc = (loc + 1) % numRemainingHexes;
//                      }
//                      loopCount++;
//                  } while (hashSetArray[loc] != parent);
//                  if (isUncompactable) {
//                      compactedSetOffset[uncompactableCount] = remainingHexes[i];
//                      uncompactableCount++;
//                  }
//              }
//          }
//          // Set up for the next loop
//          memset(hashSetArray, 0, numHexes * sizeof(H3Index));
//          compactedSetOffset += uncompactableCount;
//          memcpy(remainingHexes, compactableHexes,
//                 compactableCount * sizeof(H3Index));
//          numRemainingHexes = compactableCount;
//          H3_MEMORY(free)(compactableHexes);
//      }
//      H3_MEMORY(free)(remainingHexes);
//      H3_MEMORY(free)(hashSetArray);
//      return E_SUCCESS;
//  }

/**
 * uncompactCells takes a compressed set of cells and expands back to the
 * original set of cells.
 *
 * Skips elements that are H3_NULL (i.e., 0).
 *
 * @param   compactSet  Set of compacted cells
 * @param   numCompact  The number of cells in the input compacted set
 * @param   outSet      Output array for decompressed cells (preallocated)
 * @param   numOut      The size of the output array to bound check against
 * @param   res         The H3 resolution to decompress to
 * @return              An error code if output array is too small or any cell
 *                      is smaller than the output resolution.
 */
//  H3Error H3_EXPORT(uncompactCells)(const H3Index *compactedSet,
//                                    const int64_t numCompacted, H3Index *outSet,
//                                    const int64_t numOut, const int res) {
//      int64_t i = 0;

//      for (int64_t j = 0; j < numCompacted; j++) {
//          if (!_hasChildAtRes(compactedSet[j], res)) return E_RES_MISMATCH;

//          for (IterCellsChildren iter = iterInitParent(compactedSet[j], res);
//               iter.h; i++, iterStepChild(&iter)) {
//              if (i >= numOut) return E_MEMORY_BOUNDS;  // went too far; abort!
//              outSet[i] = iter.h;
//          }
//      }
//      return E_SUCCESS;
//  }

/**
 * uncompactCellsSize takes a compacted set of hexagons and provides
 * the exact size of the uncompacted set of hexagons.
 *
 * @param   compactedSet  Set of hexagons
 * @param   numHexes      The number of hexes in the input set
 * @param   res           The hexagon resolution to decompress to
 * @param   out           The number of hexagons to allocate memory for
 * @returns E_SUCCESS on success, or another value on error
 */
//  H3Error H3_EXPORT(uncompactCellsSize)(const H3Index *compactedSet,
//                                        const int64_t numCompacted, const int res,
//                                        int64_t *out) {
//      int64_t numOut = 0;
//      for (int64_t i = 0; i < numCompacted; i++) {
//          if (compactedSet[i] == H3_NULL) continue;

//          int64_t childrenSize;
//          H3Error childrenError =
//              H3_EXPORT(cellToChildrenSize)(compactedSet[i], res, &childrenSize);
//          if (childrenError) {
//              // The parent res does not contain `res`.
//              return E_RES_MISMATCH;
//          }
//          numOut += childrenSize;
//      }
//      *out = numOut;
//      return E_SUCCESS;
//  }

/**
 * isResClassIII takes a hexagon ID and determines if it is in a
 * Class III resolution (rotated versus the icosahedron and subject
 * to shape distortion adding extra points on icosahedron edges, making
 * them not true hexagons).
 * @param h The H3Index to check.
 * @return Returns 1 if the hexagon is class III, otherwise 0.
 */
pub fn isResClassIII(h: u64) -> i32 {
    if H3_GET_RESOLUTION(h) % 2 == 1 {
        1
    } else {
        0
    }
}

/**
 * isPentagon takes an H3Index and determines if it is actually a pentagon.
 * @param h The H3Index to check.
 * @return Returns 1 if it is a pentagon, otherwise 0.
 */
pub fn isPentagon(h: u64) -> i32 {
    if _isBaseCellPentagon(H3_GET_BASE_CELL(h))
        && _h3LeadingNonZeroDigit(h) == Direction::CENTER_DIGIT
    {
        1
    } else {
        0
    }
}

/**
 * Returns the highest resolution non-zero digit in an H3Index.
 * @param h The H3Index.
 * @return The highest resolution non-zero digit in the H3Index.
 */
fn _h3LeadingNonZeroDigit(h: u64) -> Direction {
    for r in 1..=H3_GET_RESOLUTION(h) {
        if H3_GET_INDEX_DIGIT(h, r) != Direction::CENTER_DIGIT {
            return H3_GET_INDEX_DIGIT(h, r);
        }
    }

    // if we're here it's all 0's
    Direction::CENTER_DIGIT
}

/**
 * Rotate an H3Index 60 degrees counter-clockwise about a pentagonal center.
 * @param h The H3Index.
 */
fn _h3RotatePent60ccw(h: u64) -> u64 {
    // rotate in place; skips any leading 1 digits (k-axis)

    let mut mutH = h;
    let mut foundFirstNonZeroDigit = false;
    let res = H3_GET_RESOLUTION(h);
    for r in 1..=res {
        // rotate this digit
        mutH = H3_SET_INDEX_DIGIT(mutH, r, _rotate60ccw(H3_GET_INDEX_DIGIT(mutH, r)));

        // look for the first non-zero digit so we
        // can adjust for deleted k-axes sequence
        // if necessary
        if !foundFirstNonZeroDigit && H3_GET_INDEX_DIGIT(mutH, r) != Direction::CENTER_DIGIT {
            foundFirstNonZeroDigit = true;

            // adjust for deleted k-axes sequence
            if _h3LeadingNonZeroDigit(mutH) == Direction::K_AXES_DIGIT {
                mutH = _h3Rotate60ccw(mutH);
            }
        }
    }
    mutH
}

/**
 * Rotate an H3Index 60 degrees clockwise about a pentagonal center.
 * @param h The H3Index.
 */
fn _h3RotatePent60cw(h: u64) -> u64 {
    // rotate in place; skips any leading 1 digits (k-axis)

    let mut mutH = h;
    let res = H3_GET_RESOLUTION(h);
    let mut foundFirstNonZeroDigit = false;
    for r in 1..=res {
        // rotate this digit
        mutH = H3_SET_INDEX_DIGIT(mutH, r, _rotate60cw(H3_GET_INDEX_DIGIT(mutH, r)));

        // look for the first non-zero digit so we
        // can adjust for deleted k-axes sequence
        // if necessary
        if !foundFirstNonZeroDigit && H3_GET_INDEX_DIGIT(mutH, r) != Direction::CENTER_DIGIT {
            foundFirstNonZeroDigit = true;

            // adjust for deleted k-axes sequence
            if _h3LeadingNonZeroDigit(mutH) == Direction::K_AXES_DIGIT {
                mutH = _h3Rotate60cw(mutH);
            }
        }
    }
    mutH
}

/**
 * Rotate an H3Index 60 degrees counter-clockwise.
 * @param h The H3Index.
 */
fn _h3Rotate60ccw(h: u64) -> u64 {
    let mut mutH = h;
    let res = H3_GET_RESOLUTION(h);
    for r in 1..=res {
        let oldDigit = H3_GET_INDEX_DIGIT(mutH, r);
        mutH = H3_SET_INDEX_DIGIT(mutH, r, _rotate60ccw(oldDigit));
    }

    mutH
}

/**
 * Rotate an H3Index 60 degrees clockwise.
 * @param h The H3Index.
 */
fn _h3Rotate60cw(h: u64) -> u64 {
    let mut mutH = h;
    let res = H3_GET_RESOLUTION(h);
    for r in 1..=res {
        let oldDigit = H3_GET_INDEX_DIGIT(mutH, r);
        mutH = H3_SET_INDEX_DIGIT(mutH, r, _rotate60cw(oldDigit));
    }

    mutH
}

/**
 * Convert an FaceIJK address to the corresponding H3Index.
 * @param fijk The FaceIJK address.
 * @param res The cell resolution.
 * @return The encoded H3Index (or H3_NULL on failure).
 */
fn _faceIjkToH3(fijk: FaceIJK, res: i8) -> u64 {
    // initialize the index
    let mut h = H3_INIT;
    h = H3_SET_MODE(h, H3_CELL_MODE);
    h = H3_SET_RESOLUTION(h, res);

    // check for res 0/base cell
    if res == 0 {
        if fijk.coord.i > MAX_FACE_COORD as i32
            || fijk.coord.j > MAX_FACE_COORD as i32
            || fijk.coord.k > MAX_FACE_COORD as i32
        {
            // out of range input
            return H3_NULL;
        }

        h = H3_SET_BASE_CELL(h, _faceIjkToBaseCell(fijk));
        return h;
    }

    // we need to find the correct base cell FaceIJK for this H3 index;
    // start with the passed in face and resolution res ijk coordinates
    // in that face's coordinate system
    let mut fijkBC = fijk;

    // build the H3Index from finest res up
    // adjust r for the fact that the res 0 base cell offsets the indexing
    // digits
    for r in (0..=res - 1).rev() {
        let lastIJK = fijkBC.coord;
        let mut lastCenter: CoordIJK;
        if isResolutionClassIII(r + 1) {
            // rotate ccw
            _upAp7(&mut fijkBC.coord);
            lastCenter = fijkBC.coord;
            _downAp7(&mut lastCenter);
        } else {
            // rotate cw
            _upAp7r(&mut fijkBC.coord);
            lastCenter = fijkBC.coord;
            _downAp7r(&mut lastCenter);
        }

        let mut diff = CoordIJK { i: 0, j: 0, k: 0 };
        _ijkSub(lastIJK, lastCenter, &mut diff);
        _ijkNormalize(&mut diff);

        h = H3_SET_INDEX_DIGIT(h, r + 1, _unitIjkToDigit(diff));
    }

    // fijkBC should now hold the IJK of the base cell in the
    // coordinate system of the current face

    if fijkBC.coord.i > MAX_FACE_COORD as i32
        || fijkBC.coord.j > MAX_FACE_COORD as i32
        || fijkBC.coord.k > MAX_FACE_COORD as i32
    {
        // out of range input
        return H3_NULL;
    }

    // lookup the correct base cell
    let baseCell = _faceIjkToBaseCell(fijkBC);
    h = H3_SET_BASE_CELL(h, baseCell);

    // rotate if necessary to get canonical base cell orientation
    // for this base cell
    let numRots = _faceIjkToBaseCellCCWrot60(fijkBC);
    if _isBaseCellPentagon(baseCell) {
        // force rotation out of missing k-axes sub-sequence
        if _h3LeadingNonZeroDigit(h) == Direction::K_AXES_DIGIT {
            // check for a cw/ccw offset face; default is ccw
            if _baseCellIsCwOffset(baseCell, fijkBC.face) {
                h = _h3Rotate60cw(h);
            } else {
                h = _h3Rotate60ccw(h);
            }
        }

        for i in 0..numRots {
            h = _h3RotatePent60ccw(h);
        }
    } else {
        for i in 0..numRots {
            h = _h3Rotate60ccw(h);
        }
    }

    h
}

/**
 * Encodes a coordinate on the sphere to the H3 index of the containing cell at
 * the specified resolution.
 *
 * Returns 0 on invalid input.
 *
 * @param g The spherical coordinates to encode.
 * @param res The desired H3 resolution for the encoding.
 * @param out The encoded H3Index.
 * @returns E_SUCCESS (0) on success, another value otherwise
 */
pub fn latLngToCell(g: LatLng, res: i32, out: &mut u64) -> H3Error {
    if res < 0 || res > MAX_H3_RES.into() {
        return H3Error::E_RES_DOMAIN;
    }
    // TODO:
    //  if (!isfinite(g->lat) || !isfinite(g->lng)) {
    //      return E_LATLNG_DOMAIN;
    //  }

    let mut fijk = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    _geoToFaceIjk(g, res.try_into().unwrap(), &mut fijk);
    *out = _faceIjkToH3(fijk, res.try_into().unwrap());
    if *out != 0u64 {
        return H3Error::E_SUCCESS;
    } else {
        return H3Error::E_FAILED;
    }
}

//  /**
//   * Convert an H3Index to the FaceIJK address on a specified icosahedral face.
//   * @param h The H3Index.
//   * @param fijk The FaceIJK address, initialized with the desired face
//   *        and normalized base cell coordinates.
//   * @return Returns 1 if the possibility of overage exists, otherwise 0.
//   */
//  int _h3ToFaceIjkWithInitializedFijk(H3Index h, FaceIJK *fijk) {
//      CoordIJK *ijk = &fijk->coord;
//      int res = H3_GET_RESOLUTION(h);

//      // center base cell hierarchy is entirely on this face
//      int possibleOverage = 1;
//      if (!_isBaseCellPentagon(H3_GET_BASE_CELL(h)) &&
//          (res == 0 ||
//           (fijk->coord.i == 0 && fijk->coord.j == 0 && fijk->coord.k == 0)))
//          possibleOverage = 0;

//      for (int r = 1; r <= res; r++) {
//          if (isResolutionClassIII(r)) {
//              // Class III == rotate ccw
//              _downAp7(ijk);
//          } else {
//              // Class II == rotate cw
//              _downAp7r(ijk);
//          }

//          _neighbor(ijk, H3_GET_INDEX_DIGIT(h, r));
//      }

//      return possibleOverage;
//  }

//  /**
//   * Convert an H3Index to a FaceIJK address.
//   * @param h The H3Index.
//   * @param fijk The corresponding FaceIJK address.
//   */
//  H3Error _h3ToFaceIjk(H3Index h, FaceIJK *fijk) {
//      int baseCell = H3_GET_BASE_CELL(h);
//      if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {  // LCOV_EXCL_BR_LINE
//          // Base cells less than zero can not be represented in an index
//          // To prevent reading uninitialized memory, we zero the output.
//          fijk->face = 0;
//          fijk->coord.i = fijk->coord.j = fijk->coord.k = 0;
//          return E_CELL_INVALID;
//      }
//      // adjust for the pentagonal missing sequence; all of sub-sequence 5 needs
//      // to be adjusted (and some of sub-sequence 4 below)
//      if (_isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) == 5)
//          h = _h3Rotate60cw(h);

//      // start with the "home" face and ijk+ coordinates for the base cell of c
//      *fijk = baseCellData[baseCell].homeFijk;
//      if (!_h3ToFaceIjkWithInitializedFijk(h, fijk))
//          return E_SUCCESS;  // no overage is possible; h lies on this face

//      // if we're here we have the potential for an "overage"; i.e., it is
//      // possible that c lies on an adjacent face

//      CoordIJK origIJK = fijk->coord;

//      // if we're in Class III, drop into the next finer Class II grid
//      int res = H3_GET_RESOLUTION(h);
//      if (isResolutionClassIII(res)) {
//          // Class III
//          _downAp7r(&fijk->coord);
//          res++;
//      }

//      // adjust for overage if needed
//      // a pentagon base cell with a leading 4 digit requires special handling
//      int pentLeading4 =
//          (_isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) == 4);
//      if (_adjustOverageClassII(fijk, res, pentLeading4, 0) != NO_OVERAGE) {
//          // if the base cell is a pentagon we have the potential for secondary
//          // overages
//          if (_isBaseCellPentagon(baseCell)) {
//              while (_adjustOverageClassII(fijk, res, 0, 0) != NO_OVERAGE)
//                  continue;
//          }

//          if (res != H3_GET_RESOLUTION(h)) _upAp7r(&fijk->coord);
//      } else if (res != H3_GET_RESOLUTION(h)) {
//          fijk->coord = origIJK;
//      }
//      return E_SUCCESS;
//  }

//  /**
//   * Determines the spherical coordinates of the center point of an H3 index.
//   *
//   * @param h3 The H3 index.
//   * @param g The spherical coordinates of the H3 cell center.
//   */
//  H3Error H3_EXPORT(cellToLatLng)(H3Index h3, LatLng *g) {
//      FaceIJK fijk;
//      H3Error e = _h3ToFaceIjk(h3, &fijk);
//      if (e) {
//          return e;
//      }
//      _faceIjkToGeo(&fijk, H3_GET_RESOLUTION(h3), g);
//      return E_SUCCESS;
//  }

//  /**
//   * Determines the cell boundary in spherical coordinates for an H3 index.
//   *
//   * @param h3 The H3 index.
//   * @param cb The boundary of the H3 cell in spherical coordinates.
//   */
//  H3Error H3_EXPORT(cellToBoundary)(H3Index h3, CellBoundary *cb) {
//      FaceIJK fijk;
//      H3Error e = _h3ToFaceIjk(h3, &fijk);
//      if (e) {
//          return e;
//      }
//      if (H3_EXPORT(isPentagon)(h3)) {
//          _faceIjkPentToCellBoundary(&fijk, H3_GET_RESOLUTION(h3), 0,
//                                     NUM_PENT_VERTS, cb);
//      } else {
//          _faceIjkToCellBoundary(&fijk, H3_GET_RESOLUTION(h3), 0, NUM_HEX_VERTS,
//                                 cb);
//      }
//      return E_SUCCESS;
//  }

//  /**
//   * Returns the max number of possible icosahedron faces an H3 index
//   * may intersect.
//   *
//   * @return int count of faces
//   */
//  H3Error H3_EXPORT(maxFaceCount)(H3Index h3, int *out) {
//      // a pentagon always intersects 5 faces, a hexagon never intersects more
//      // than 2 (but may only intersect 1)
//      *out = H3_EXPORT(isPentagon)(h3) ? 5 : 2;
//      return E_SUCCESS;
//  }

//  /**
//   * Find all icosahedron faces intersected by a given H3 index, represented
//   * as integers from 0-19. The array is sparse; since 0 is a valid value,
//   * invalid array values are represented as -1. It is the responsibility of
//   * the caller to filter out invalid values.
//   *
//   * @param h3 The H3 index
//   * @param out Output array. Must be of size maxFaceCount(h3).
//   */
//  H3Error H3_EXPORT(getIcosahedronFaces)(H3Index h3, int *out) {
//      int res = H3_GET_RESOLUTION(h3);
//      int isPent = H3_EXPORT(isPentagon)(h3);

//      // We can't use the vertex-based approach here for class II pentagons,
//      // because all their vertices are on the icosahedron edges. Their
//      // direct child pentagons cross the same faces, so use those instead.
//      if (isPent && !isResolutionClassIII(res)) {
//          // Note that this would not work for res 15, but this is only run on
//          // Class II pentagons, it should never be invoked for a res 15 index.
//          H3Index childPentagon = makeDirectChild(h3, 0);
//          return H3_EXPORT(getIcosahedronFaces)(childPentagon, out);
//      }

//      // convert to FaceIJK
//      FaceIJK fijk;
//      H3Error err = _h3ToFaceIjk(h3, &fijk);
//      if (err) {
//          return err;
//      }

//      // Get all vertices as FaceIJK addresses. For simplicity, always
//      // initialize the array with 6 verts, ignoring the last one for pentagons
//      FaceIJK fijkVerts[NUM_HEX_VERTS];
//      int vertexCount;

//      if (isPent) {
//          vertexCount = NUM_PENT_VERTS;
//          _faceIjkPentToVerts(&fijk, &res, fijkVerts);
//      } else {
//          vertexCount = NUM_HEX_VERTS;
//          _faceIjkToVerts(&fijk, &res, fijkVerts);
//      }

//      // We may not use all of the slots in the output array,
//      // so fill with invalid values to indicate unused slots
//      int faceCount;
//      H3Error maxFaceCountError = H3_EXPORT(maxFaceCount)(h3, &faceCount);
//      if (maxFaceCountError != E_SUCCESS) {
//          return maxFaceCountError;
//      }
//      for (int i = 0; i < faceCount; i++) {
//          out[i] = INVALID_FACE;
//      }

//      // add each vertex face, using the output array as a hash set
//      for (int i = 0; i < vertexCount; i++) {
//          FaceIJK *vert = &fijkVerts[i];

//          // Adjust overage, determining whether this vertex is
//          // on another face
//          if (isPent) {
//              _adjustPentVertOverage(vert, res);
//          } else {
//              _adjustOverageClassII(vert, res, 0, 1);
//          }

//          // Save the face to the output array
//          int face = vert->face;
//          int pos = 0;
//          // Find the first empty output position, or the first position
//          // matching the current face
//          while (out[pos] != INVALID_FACE && out[pos] != face) {
//              pos++;
//              if (pos >= faceCount) {
//                  // Mismatch between the heuristic used in maxFaceCount and
//                  // calculation here - indicates an invalid index.
//                  return E_FAILED;
//              }
//          }
//          out[pos] = face;
//      }
//      return E_SUCCESS;
//  }

//  /**
//   * pentagonCount returns the number of pentagons (same at any resolution)
//   *
//   * @return int count of pentagon indexes
//   */
//  int H3_EXPORT(pentagonCount)() { return NUM_PENTAGONS; }

//  /**
//   * Generates all pentagons at the specified resolution
//   *
//   * @param res The resolution to produce pentagons at.
//   * @param out Output array. Must be of size pentagonCount().
//   */
//  H3Error H3_EXPORT(getPentagons)(int res, H3Index *out) {
//      if (res < 0 || res > MAX_H3_RES) {
//          return E_RES_DOMAIN;
//      }
//      int i = 0;
//      for (int bc = 0; bc < NUM_BASE_CELLS; bc++) {
//          if (_isBaseCellPentagon(bc)) {
//              H3Index pentagon;
//              setH3Index(&pentagon, res, bc, 0);
//              out[i++] = pentagon;
//          }
//      }
//      return E_SUCCESS;
//  }

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

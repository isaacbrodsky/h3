/** @struct FaceIJK
 * @brief Face number and ijk coordinates on that face-centered coordinate
 * system
 */
pub struct FaceIJK {
    // face number
    pub face: i8,
    // ijk coordinates on that face
    pub coord: CoordIJK,
}

/** @struct FaceOrientIJK
 * @brief Information to transform into an adjacent face IJK system
 */
 struct FaceOrientIJK {
    // face number
    face: i8,
    // res 0 translation relative to primary face
    translate: CoordIJK,
    // number of 60 degree ccw rotations relative to primary face
    ccwRot60: i8,
 }

// indexes for faceNeighbors table
/** IJ quadrant faceNeighbors table direction */
const IJ: i8 = 1;
/** KI quadrant faceNeighbors table direction */
const KI: i8 = 2;
/** JK quadrant faceNeighbors table direction */
const JK: i8 = 3;

/** Invalid face index */
const INVALID_FACE: i8 = -1;

/** Digit representing overage type */
enum Overage {
    /** No overage (on original face) */
    NO_OVERAGE = 0,
    /** On face edge (only occurs on substrate grids) */
    FACE_EDGE = 1,
    /** Overage on new face interior */
    NEW_FACE = 2
}
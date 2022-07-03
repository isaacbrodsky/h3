use crate::constants::{
    EPSILON, M_AP7_ROT_RADS, M_SQRT3_2, NUM_HEX_VERTS, NUM_ICOSA_FACES, NUM_PENT_VERTS,
    RES0_U_GNOMONIC,
};
use crate::coordijk::{
    CoordIJK, _downAp3, _downAp3r, _downAp7, _downAp7r, _hex2dToCoordIJK, _ijkAdd, _ijkNormalize,
    _ijkRotate60ccw, _ijkRotate60cw, _ijkScale, _ijkSub, _ijkToHex2d,
};
use crate::h3Index::isResolutionClassIII;
use crate::h3api::{CellBoundary, LatLng};
use crate::latLng::{_geoAzDistanceRads, _geoAzimuthRads, _posAngleRads};
use crate::vec2d::{Vec2d, _v2dEquals, _v2dIntersect, _v2dMag};
use crate::vec3d::{Vec3d, _geoToVec3d, _pointSquareDist};

/** @struct FaceIJK
 * @brief Face number and ijk coordinates on that face-centered coordinate
 * system
 */
#[derive(Copy, Clone)]
pub struct FaceIJK {
    // face number
    pub face: i8,
    // ijk coordinates on that face
    pub coord: CoordIJK,
}

/** @struct FaceOrientIJK
 * @brief Information to transform into an adjacent face IJK system
 */
#[derive(Copy, Clone)]
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
#[derive(PartialEq)]
enum Overage {
    /** No overage (on original face) */
    NO_OVERAGE = 0,
    /** On face edge (only occurs on substrate grids) */
    FACE_EDGE = 1,
    /** Overage on new face interior */
    NEW_FACE = 2,
}

/** square root of 7 */
const M_SQRT7: f64 = 2.6457513110645905905016157536392604257102;

/** @brief icosahedron face centers in lat/lng radians */
const faceCenterGeo: [LatLng; NUM_ICOSA_FACES] = [
    LatLng {
        lat: 0.803582649718989942,
        lng: 1.248397419617396099,
    }, // face  0
    LatLng {
        lat: 1.307747883455638156,
        lng: 2.536945009877921159,
    }, // face  1
    LatLng {
        lat: 1.054751253523952054,
        lng: -1.347517358900396623,
    }, // face  2
    LatLng {
        lat: 0.600191595538186799,
        lng: -0.450603909469755746,
    }, // face  3
    LatLng {
        lat: 0.491715428198773866,
        lng: 0.401988202911306943,
    }, // face  4
    LatLng {
        lat: 0.172745327415618701,
        lng: 1.678146885280433686,
    }, // face  5
    LatLng {
        lat: 0.605929321571350690,
        lng: 2.953923329812411617,
    }, // face  6
    LatLng {
        lat: 0.427370518328979641,
        lng: -1.888876200336285401,
    }, // face  7
    LatLng {
        lat: -0.079066118549212831,
        lng: -0.733429513380867741,
    }, // face  8
    LatLng {
        lat: -0.230961644455383637,
        lng: 0.506495587332349035,
    }, // face  9
    LatLng {
        lat: 0.079066118549212831,
        lng: 2.408163140208925497,
    }, // face 10
    LatLng {
        lat: 0.230961644455383637,
        lng: -2.635097066257444203,
    }, // face 11
    LatLng {
        lat: -0.172745327415618701,
        lng: -1.463445768309359553,
    }, // face 12
    LatLng {
        lat: -0.605929321571350690,
        lng: -0.187669323777381622,
    }, // face 13
    LatLng {
        lat: -0.427370518328979641,
        lng: 1.252716453253507838,
    }, // face 14
    LatLng {
        lat: -0.600191595538186799,
        lng: 2.690988744120037492,
    }, // face 15
    LatLng {
        lat: -0.491715428198773866,
        lng: -2.739604450678486295,
    }, // face 16
    LatLng {
        lat: -0.803582649718989942,
        lng: -1.893195233972397139,
    }, // face 17
    LatLng {
        lat: -1.307747883455638156,
        lng: -0.604647643711872080,
    }, // face 18
    LatLng {
        lat: -1.054751253523952054,
        lng: 1.794075294689396615,
    }, // face 19
];

/** @brief icosahedron face centers in x/y/z on the unit sphere */
const faceCenterPoint: [Vec3d; NUM_ICOSA_FACES] = [
    Vec3d {
        x: 0.2199307791404606,
        y: 0.6583691780274996,
        z: 0.7198475378926182,
    }, // face  0
    Vec3d {
        x: -0.2139234834501421,
        y: 0.1478171829550703,
        z: 0.9656017935214205,
    }, // face  1
    Vec3d {
        x: 0.1092625278784797,
        y: -0.4811951572873210,
        z: 0.8697775121287253,
    }, // face  2
    Vec3d {
        x: 0.7428567301586791,
        y: -0.3593941678278028,
        z: 0.5648005936517033,
    }, // face  3
    Vec3d {
        x: 0.8112534709140969,
        y: 0.3448953237639384,
        z: 0.4721387736413930,
    }, // face  4
    Vec3d {
        x: -0.1055498149613921,
        y: 0.9794457296411413,
        z: 0.1718874610009365,
    }, // face  5
    Vec3d {
        x: -0.8075407579970092,
        y: 0.1533552485898818,
        z: 0.5695261994882688,
    }, // face  6
    Vec3d {
        x: -0.2846148069787907,
        y: -0.8644080972654206,
        z: 0.4144792552473539,
    }, // face  7
    Vec3d {
        x: 0.7405621473854482,
        y: -0.6673299564565524,
        z: -0.0789837646326737,
    }, // face  8
    Vec3d {
        x: 0.8512303986474293,
        y: 0.4722343788582681,
        z: -0.2289137388687808,
    }, // face  9
    Vec3d {
        x: -0.7405621473854481,
        y: 0.6673299564565524,
        z: 0.0789837646326737,
    }, // face 10
    Vec3d {
        x: -0.8512303986474292,
        y: -0.4722343788582682,
        z: 0.2289137388687808,
    }, // face 11
    Vec3d {
        x: 0.1055498149613919,
        y: -0.9794457296411413,
        z: -0.1718874610009365,
    }, // face 12
    Vec3d {
        x: 0.8075407579970092,
        y: -0.1533552485898819,
        z: -0.5695261994882688,
    }, // face 13
    Vec3d {
        x: 0.2846148069787908,
        y: 0.8644080972654204,
        z: -0.4144792552473539,
    }, // face 14
    Vec3d {
        x: -0.7428567301586791,
        y: 0.3593941678278027,
        z: -0.5648005936517033,
    }, // face 15
    Vec3d {
        x: -0.8112534709140971,
        y: -0.3448953237639382,
        z: -0.4721387736413930,
    }, // face 16
    Vec3d {
        x: -0.2199307791404607,
        y: -0.6583691780274996,
        z: -0.7198475378926182,
    }, // face 17
    Vec3d {
        x: 0.2139234834501420,
        y: -0.1478171829550704,
        z: -0.9656017935214205,
    }, // face 18
    Vec3d {
        x: -0.1092625278784796,
        y: 0.4811951572873210,
        z: -0.8697775121287253,
    }, // face 19
];

/** @brief icosahedron face ijk axes as azimuth in radians from face center to
 * vertex 0/1/2 respectively
 */
const faceAxesAzRadsCII: [[f64; 3]; NUM_ICOSA_FACES] = [
    [
        5.619958268523939882,
        3.525563166130744542,
        1.431168063737548730,
    ], // face  0
    [
        5.760339081714187279,
        3.665943979320991689,
        1.571548876927796127,
    ], // face  1
    [
        0.780213654393430055,
        4.969003859179821079,
        2.874608756786625655,
    ], // face  2
    [
        0.430469363979999913,
        4.619259568766391033,
        2.524864466373195467,
    ], // face  3
    [
        6.130269123335111400,
        4.035874020941915804,
        1.941478918548720291,
    ], // face  4
    [
        2.692877706530642877,
        0.598482604137447119,
        4.787272808923838195,
    ], // face  5
    [
        2.982963003477243874,
        0.888567901084048369,
        5.077358105870439581,
    ], // face  6
    [
        3.532912002790141181,
        1.438516900396945656,
        5.627307105183336758,
    ], // face  7
    [
        3.494305004259568154,
        1.399909901866372864,
        5.588700106652763840,
    ], // face  8
    [
        3.003214169499538391,
        0.908819067106342928,
        5.097609271892733906,
    ], // face  9
    [
        5.930472956509811562,
        3.836077854116615875,
        1.741682751723420374,
    ], // face 10
    [
        0.138378484090254847,
        4.327168688876645809,
        2.232773586483450311,
    ], // face 11
    [
        0.448714947059150361,
        4.637505151845541521,
        2.543110049452346120,
    ], // face 12
    [
        0.158629650112549365,
        4.347419854898940135,
        2.253024752505744869,
    ], // face 13
    [
        5.891865957979238535,
        3.797470855586042958,
        1.703075753192847583,
    ], // face 14
    [
        2.711123289609793325,
        0.616728187216597771,
        4.805518392002988683,
    ], // face 15
    [
        3.294508837434268316,
        1.200113735041072948,
        5.388903939827463911,
    ], // face 16
    [
        3.804819692245439833,
        1.710424589852244509,
        5.899214794638635174,
    ], // face 17
    [
        3.664438879055192436,
        1.570043776661997111,
        5.758833981448388027,
    ], // face 18
    [
        2.361378999196363184,
        0.266983896803167583,
        4.455774101589558636,
    ], // face 19
];

/** @brief Definition of which faces neighbor each other. */
const faceNeighbors: [[FaceOrientIJK; 4]; NUM_ICOSA_FACES] = [
    [
        // face 0
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 1
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 2
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 3
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 4
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 5
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 0,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 6
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 1,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 7
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 2,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 8
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 3,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 9
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 4,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 10
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 11
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 6,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 12
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 7,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 13
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 8,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 14
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 9,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 3,
        }, // ij quadrant
        FaceOrientIJK {
            face: 5,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 3,
        }, // ki quadrant
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 15
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 10,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 16
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 11,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 17
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 16,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 12,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 18
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 17,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 13,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
    [
        // face 19
        FaceOrientIJK {
            face: 19,
            translate: CoordIJK { i: 0, j: 0, k: 0 },
            ccwRot60: 0,
        }, // central face
        FaceOrientIJK {
            face: 15,
            translate: CoordIJK { i: 2, j: 0, k: 2 },
            ccwRot60: 1,
        }, // ij quadrant
        FaceOrientIJK {
            face: 18,
            translate: CoordIJK { i: 2, j: 2, k: 0 },
            ccwRot60: 5,
        }, // ki quadrant
        FaceOrientIJK {
            face: 14,
            translate: CoordIJK { i: 0, j: 2, k: 2 },
            ccwRot60: 3,
        }, // jk quadrant
    ],
];

/** @brief direction from the origin face to the destination face, relative to
 * the origin face's coordinate system, or -1 if not adjacent.
 */
const adjacentFaceDir: [[i8; NUM_ICOSA_FACES]; NUM_ICOSA_FACES] = [
    [
        0, KI, -1, -1, IJ, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 0
    [
        IJ, 0, KI, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 1
    [
        -1, IJ, 0, KI, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 2
    [
        -1, -1, IJ, 0, KI, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 3
    [
        KI, -1, -1, IJ, 0, -1, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 4
    [
        JK, -1, -1, -1, -1, 0, -1, -1, -1, -1, IJ, -1, -1, -1, KI, -1, -1, -1, -1, -1,
    ], // face 5
    [
        -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 6
    [
        -1, -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1, -1, -1,
    ], // face 7
    [
        -1, -1, -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1, -1,
    ], // face 8
    [
        -1, -1, -1, -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1,
    ], // face 9
    [
        -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1, -1, -1, -1,
    ], // face 10
    [
        -1, -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1, -1, -1,
    ], // face 11
    [
        -1, -1, -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1, -1,
    ], // face 12
    [
        -1, -1, -1, -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1,
    ], // face 13
    [
        -1, -1, -1, -1, -1, KI, -1, -1, -1, IJ, -1, -1, -1, -1, 0, -1, -1, -1, -1, JK,
    ], // face 14
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, -1, 0, IJ, -1, -1, KI,
    ], // face 15
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, KI, 0, IJ, -1, -1,
    ], // face 16
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, KI, 0, IJ, -1,
    ], // face 17
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, KI, 0, IJ,
    ], // face 18
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, IJ, -1, -1, KI, 0,
    ], // face 19
];

/** @brief overage distance table */
const maxDimByCIIres: [i32; 17] = [
    2,        // res  0
    -1,       // res  1
    14,       // res  2
    -1,       // res  3
    98,       // res  4
    -1,       // res  5
    686,      // res  6
    -1,       // res  7
    4802,     // res  8
    -1,       // res  9
    33614,    // res 10
    -1,       // res 11
    235298,   // res 12
    -1,       // res 13
    1647086,  // res 14
    -1,       // res 15
    11529602, // res 16
];

/** @brief unit scale distance table */
const unitScaleByCIIres: [i32; 17] = [
    1,       // res  0
    -1,      // res  1
    7,       // res  2
    -1,      // res  3
    49,      // res  4
    -1,      // res  5
    343,     // res  6
    -1,      // res  7
    2401,    // res  8
    -1,      // res  9
    16807,   // res 10
    -1,      // res 11
    117649,  // res 12
    -1,      // res 13
    823543,  // res 14
    -1,      // res 15
    5764801, // res 16
];

/**
 * Encodes a coordinate on the sphere to the FaceIJK address of the containing
 * cell at the specified resolution.
 *
 * @param g The spherical coordinates to encode.
 * @param res The desired H3 resolution for the encoding.
 * @param h The FaceIJK address of the containing cell at resolution res.
 */
fn _geoToFaceIjk(g: LatLng, res: i8, h: &mut FaceIJK) {
    // first convert to hex2d
    let mut v = Vec2d { x: 0.0, y: 0.0 };
    _geoToHex2d(g, res, &mut h.face, &mut v);

    // then convert to ijk+
    _hex2dToCoordIJK(v, &mut h.coord);
}

/**
 * Encodes a coordinate on the sphere to the corresponding icosahedral face and
 * containing 2D hex coordinates relative to that face center.
 *
 * @param g The spherical coordinates to encode.
 * @param res The desired H3 resolution for the encoding.
 * @param face The icosahedral face containing the spherical coordinates.
 * @param v The 2D hex coordinates of the cell containing the point.
 */
fn _geoToHex2d(g: LatLng, res: i8, face: &mut i8, v: &mut Vec2d) {
    // determine the icosahedron face
    let mut sqd: f64 = 0.0;
    _geoToClosestFace(g, face, &mut sqd);

    // cos(r) = 1 - 2 * sin^2(r/2) = 1 - 2 * (sqd / 4) = 1 - sqd/2
    let mut r = (1.0 - sqd / 2.0).acos();

    if (r < EPSILON) {
        v.x = 0.0;
        v.y = 0.0;
        return;
    }

    // now have face and r, now find CCW theta from CII i-axis
    let mut theta = _posAngleRads(
        faceAxesAzRadsCII[*face as usize][0]
            - _posAngleRads(_geoAzimuthRads(faceCenterGeo[*face as usize], g)),
    );

    // adjust theta for Class III (odd resolutions)
    if (isResolutionClassIII(res)) {
        theta = _posAngleRads(theta - M_AP7_ROT_RADS);
    }

    // perform gnomonic scaling of r
    r = r.tan();

    // scale for current resolution length u
    r /= RES0_U_GNOMONIC;
    for i in 0..res {
        r *= M_SQRT7;
    }

    // we now have (r, theta) in hex2d with theta ccw from x-axes

    // convert to local x,y
    v.x = r * theta.cos();
    v.y = r * theta.sin();
}

/**
 * Determines the center point in spherical coordinates of a cell given by 2D
 * hex coordinates on a particular icosahedral face.
 *
 * @param v The 2D hex coordinates of the cell.
 * @param face The icosahedral face upon which the 2D hex coordinate system is
 *             centered.
 * @param res The H3 resolution of the cell.
 * @param substrate Indicates whether or not this grid is actually a substrate
 *        grid relative to the specified resolution.
 * @param g The spherical coordinates of the cell center point.
 */
fn _hex2dToGeo(v: Vec2d, face: i8, res: i8, substrate: bool, g: &mut LatLng) {
    // calculate (r, theta) in hex2d
    let mut r = _v2dMag(v);

    if (r < EPSILON) {
        *g = faceCenterGeo[face as usize];
        return;
    }

    let mut theta = v.y.atan2(v.x);

    // scale for current resolution length u
    for i in 0..res {
        r /= M_SQRT7;
    }

    // scale accordingly if this is a substrate grid
    if (substrate) {
        r /= 3.0;
        if (isResolutionClassIII(res)) {
            r /= M_SQRT7;
        }
    }

    r *= RES0_U_GNOMONIC;

    // perform inverse gnomonic scaling of r
    r = r.atan();

    // adjust theta for Class III
    // if a substrate grid, then it's already been adjusted for Class III
    if (!substrate && isResolutionClassIII(res)) {
        theta = _posAngleRads(theta + M_AP7_ROT_RADS);
    }

    // find theta as an azimuth
    theta = _posAngleRads(faceAxesAzRadsCII[face as usize][0] - theta);

    // now find the point at (r,theta) from the face center
    _geoAzDistanceRads(faceCenterGeo[face as usize], theta, r, g);
}

/**
 * Determines the center point in spherical coordinates of a cell given by
 * a FaceIJK address at a specified resolution.
 *
 * @param h The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 * @param g The spherical coordinates of the cell center point.
 */
fn _faceIjkToGeo(h: FaceIJK, res: i8, g: &mut LatLng) {
    let mut v = Vec2d { x: 0.0, y: 0.0 };
    _ijkToHex2d(h.coord, &mut v);
    _hex2dToGeo(v, h.face, res, false, g);
}

/**
 * Generates the cell boundary in spherical coordinates for a pentagonal cell
 * given by a FaceIJK address at a specified resolution.
 *
 * @param h The FaceIJK address of the pentagonal cell.
 * @param res The H3 resolution of the cell.
 * @param start The first topological vertex to return.
 * @param length The number of topological vertexes to return.
 * @param g The spherical coordinates of the cell boundary.
 */
fn _faceIjkPentToCellBoundary(h: FaceIJK, res: i8, start: i8, length: i8, g: &mut CellBoundary) {
    let mut adjRes = res;
    let mut centerIJK = h;
    let mut fijkVerts: [FaceIJK; NUM_PENT_VERTS as usize] = [
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
    ];
    _faceIjkPentToVerts(&mut centerIJK, &mut adjRes, &mut fijkVerts);

    // If we're returning the entire loop, we need one more iteration in case
    // of a distortion vertex on the last edge
    let additionalIteration = if length == NUM_PENT_VERTS { 1 } else { 0 };

    // convert each vertex to lat/lng
    // adjust the face of each vertex as appropriate and introduce
    // edge-crossing vertices as needed
    g.numVerts = 0;
    let mut lastFijk = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    for vert in start..start + length + additionalIteration {
        let v = vert % NUM_PENT_VERTS;

        let mut fijk = fijkVerts[v as usize];

        _adjustPentVertOverage(&mut fijk, adjRes);

        // all Class III pentagon edges cross icosa edges
        // note that Class II pentagons have vertices on the edge,
        // not edge intersections
        if (isResolutionClassIII(res) && vert > start) {
            // find hex2d of the two vertexes on the last face

            let mut tmpFijk = fijk;

            let mut orig2d0 = Vec2d { x: 0.0, y: 0.0 };
            _ijkToHex2d(lastFijk.coord, &mut orig2d0);

            let currentToLastDir = adjacentFaceDir[tmpFijk.face as usize][lastFijk.face as usize];

            let fijkOrient = faceNeighbors[tmpFijk.face as usize][currentToLastDir as usize];

            tmpFijk.face = fijkOrient.face;

            // rotate and translate for adjacent face
            for i in 0..fijkOrient.ccwRot60 {
                _ijkRotate60ccw(&mut tmpFijk.coord);
            }

            let mut transVec = fijkOrient.translate;
            _ijkScale(&mut transVec, unitScaleByCIIres[adjRes as usize] * 3);
            _ijkAdd(tmpFijk.coord, transVec, &mut tmpFijk.coord);
            _ijkNormalize(&mut tmpFijk.coord);

            let mut orig2d1 = Vec2d { x: 0.0, y: 0.0 };
            _ijkToHex2d(tmpFijk.coord, &mut orig2d1);

            // find the appropriate icosa face edge vertexes
            let maxDim = maxDimByCIIres[adjRes as usize] as f64;
            let v0 = Vec2d {
                x: 3.0 * maxDim,
                y: 0.0,
            };
            let v1 = Vec2d {
                x: -1.5 * maxDim,
                y: 3.0 * M_SQRT3_2 * maxDim,
            };
            let v2 = Vec2d {
                x: -1.5 * maxDim,
                y: -3.0 * M_SQRT3_2 * maxDim,
            };

            let mut edge0 = Vec2d { x: 0., y: 0. };
            let mut edge1 = Vec2d { x: 0., y: 0. };
            match adjacentFaceDir[tmpFijk.face as usize][fijk.face as usize] {
                IJ => {
                    edge0 = v0;
                    edge1 = v1;
                }
                JK => {
                    edge0 = v1;
                    edge1 = v2;
                }
                KI => {
                    edge0 = v2;
                    edge1 = v0;
                }
                _ => panic!("unexpected adjacent face dir"),
            }

            // find the intersection and add the lat/lng point to the result
            let mut inter = Vec2d { x: 0., y: 0. };
            _v2dIntersect(orig2d0, orig2d1, edge0, edge1, &mut inter);
            _hex2dToGeo(
                inter,
                tmpFijk.face,
                adjRes,
                true,
                &mut g.verts[g.numVerts as usize],
            );
            g.numVerts += 1;
        }

        // convert vertex to lat/lng and add to the result
        // vert == start + NUM_PENT_VERTS is only used to test for possible
        // intersection on last edge
        if (vert < start + NUM_PENT_VERTS) {
            let mut vec = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(fijk.coord, &mut vec);
            _hex2dToGeo(
                vec,
                fijk.face,
                adjRes,
                true,
                &mut g.verts[g.numVerts as usize],
            );
            g.numVerts += 1;
        }

        lastFijk = fijk;
    }
}

/**
 * Get the vertices of a pentagon cell as substrate FaceIJK addresses
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell. This may be adjusted if
 *            necessary for the substrate grid resolution.
 * @param fijkVerts Output array for the vertices
 */
fn _faceIjkPentToVerts(fijk: &mut FaceIJK, res: &mut i8, fijkVerts: &mut [FaceIJK]) {
    // the vertexes of an origin-centered pentagon in a Class II resolution on a
    // substrate grid with aperture sequence 33r. The aperture 3 gets us the
    // vertices, and the 3r gets us back to Class II.
    // vertices listed ccw from the i-axes
    const vertsCII: [CoordIJK; NUM_PENT_VERTS as usize] = [
        CoordIJK { i: 2, j: 1, k: 0 }, // 0
        CoordIJK { i: 1, j: 2, k: 0 }, // 1
        CoordIJK { i: 0, j: 2, k: 1 }, // 2
        CoordIJK { i: 0, j: 1, k: 2 }, // 3
        CoordIJK { i: 1, j: 0, k: 2 }, // 4
    ];

    // the vertexes of an origin-centered pentagon in a Class III resolution on
    // a substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
    // vertices, and the 3r7r gets us to Class II. vertices listed ccw from the
    // i-axes
    const vertsCIII: [CoordIJK; NUM_PENT_VERTS as usize] = [
        CoordIJK { i: 5, j: 4, k: 0 }, // 0
        CoordIJK { i: 1, j: 5, k: 0 }, // 1
        CoordIJK { i: 0, j: 5, k: 4 }, // 2
        CoordIJK { i: 0, j: 1, k: 5 }, // 3
        CoordIJK { i: 4, j: 0, k: 5 }, // 4
    ];

    // get the correct set of substrate vertices for this resolution
    let verts: [CoordIJK; NUM_PENT_VERTS as usize];
    if (isResolutionClassIII(*res)) {
        verts = vertsCIII;
    } else {
        verts = vertsCII;
    }

    // adjust the center point to be in an aperture 33r substrate grid
    // these should be composed for speed
    _downAp3(&mut fijk.coord);
    _downAp3r(&mut fijk.coord);

    // if res is Class III we need to add a cw aperture 7 to get to
    // icosahedral Class II
    if (isResolutionClassIII(*res)) {
        _downAp7r(&mut fijk.coord);
        *res += 1;
    }

    // The center point is now in the same substrate grid as the origin
    // cell vertices. Add the center point substate coordinates
    // to each vertex to translate the vertices to that cell.
    for v in 0..NUM_PENT_VERTS {
        fijkVerts[v as usize].face = fijk.face;
        _ijkAdd(
            fijk.coord,
            verts[v as usize],
            &mut fijkVerts[v as usize].coord,
        );
        _ijkNormalize(&mut fijkVerts[v as usize].coord);
    }
}

/**
 * Generates the cell boundary in spherical coordinates for a cell given by a
 * FaceIJK address at a specified resolution.
 *
 * @param h The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 * @param start The first topological vertex to return.
 * @param length The number of topological vertexes to return.
 * @param g The spherical coordinates of the cell boundary.
 */
fn _faceIjkToCellBoundary(h: FaceIJK, res: i8, start: i8, length: i8, g: &mut CellBoundary) {
    let mut adjRes = res;
    let mut centerIJK = h;
    let mut fijkVerts: [FaceIJK; NUM_HEX_VERTS as usize] = [
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
        FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        },
    ];
    _faceIjkToVerts(&mut centerIJK, &mut adjRes, &mut fijkVerts);

    // If we're returning the entire loop, we need one more iteration in case
    // of a distortion vertex on the last edge
    let additionalIteration = if length == NUM_HEX_VERTS { 1 } else { 0 };

    // convert each vertex to lat/lng
    // adjust the face of each vertex as appropriate and introduce
    // edge-crossing vertices as needed
    g.numVerts = 0;
    let mut lastFace = -1;
    let mut lastOverage = Overage::NO_OVERAGE;
    for vert in start..start + length + additionalIteration {
        let v = vert % NUM_HEX_VERTS;

        let mut fijk = fijkVerts[v as usize];

        let overage = _adjustOverageClassII(&mut fijk, adjRes, false, true);

        /*
        Check for edge-crossing. Each face of the underlying icosahedron is a
        different projection plane. So if an edge of the hexagon crosses an
        icosahedron edge, an additional vertex must be introduced at that
        intersection point. Then each half of the cell edge can be projected
        to geographic coordinates using the appropriate icosahedron face
        projection. Note that Class II cell edges have vertices on the face
        edge, with no edge line intersections.
        */
        if (isResolutionClassIII(res)
            && vert > start
            && fijk.face != lastFace
            && lastOverage != Overage::FACE_EDGE)
        {
            // find hex2d of the two vertexes on original face
            let lastV = (v + 5) % NUM_HEX_VERTS;
            let mut orig2d0 = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(fijkVerts[lastV as usize].coord, &mut orig2d0);

            let mut orig2d1 = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(fijkVerts[v as usize].coord, &mut orig2d1);

            // find the appropriate icosa face edge vertexes
            let maxDim = maxDimByCIIres[adjRes as usize] as f64;
            let v0 = Vec2d {
                x: 3.0 * maxDim,
                y: 0.0,
            };
            let v1 = Vec2d {
                x: -1.5 * maxDim,
                y: 3.0 * M_SQRT3_2 * maxDim,
            };
            let v2 = Vec2d {
                x: -1.5 * maxDim,
                y: -3.0 * M_SQRT3_2 * maxDim,
            };

            let face2 = if lastFace == centerIJK.face {
                fijk.face
            } else {
                lastFace
            };
            let mut edge0 = Vec2d { x: 0., y: 0. };
            let mut edge1 = Vec2d { x: 0., y: 0. };
            match adjacentFaceDir[centerIJK.face as usize][face2 as usize] {
                IJ => {
                    edge0 = v0;
                    edge1 = v1;
                }
                JK => {
                    edge0 = v1;
                    edge1 = v2;
                }
                KI => {
                    edge0 = v2;
                    edge1 = v0;
                }
                _ => panic!("Unexpected adjacent face dir"),
            }

            // find the intersection and add the lat/lng point to the result
            let mut inter = Vec2d { x: 0., y: 0. };
            _v2dIntersect(orig2d0, orig2d1, edge0, edge1, &mut inter);
            /*
            If a point of intersection occurs at a hexagon vertex, then each
            adjacent hexagon edge will lie completely on a single icosahedron
            face, and no additional vertex is required.
            */
            let isIntersectionAtVertex = _v2dEquals(orig2d0, inter) || _v2dEquals(orig2d1, inter);
            if !isIntersectionAtVertex {
                _hex2dToGeo(
                    inter,
                    centerIJK.face,
                    adjRes,
                    true,
                    &mut g.verts[g.numVerts as usize],
                );
                g.numVerts += 1;
            }
        }

        // convert vertex to lat/lng and add to the result
        // vert == start + NUM_HEX_VERTS is only used to test for possible
        // intersection on last edge
        if (vert < start + NUM_HEX_VERTS) {
            let mut vec = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(fijk.coord, &mut vec);
            _hex2dToGeo(
                vec,
                fijk.face,
                adjRes,
                true,
                &mut g.verts[g.numVerts as usize],
            );
            g.numVerts += 1;
        }

        lastFace = fijk.face;
        lastOverage = overage;
    }
}

/**
 * Get the vertices of a cell as substrate FaceIJK addresses
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell. This may be adjusted if
 *            necessary for the substrate grid resolution.
 * @param fijkVerts Output array for the vertices
 */
fn _faceIjkToVerts(
    fijk: &mut FaceIJK,
    res: &mut i8,
    fijkVerts: &mut [FaceIJK; NUM_HEX_VERTS as usize],
) {
    // the vertexes of an origin-centered cell in a Class II resolution on a
    // substrate grid with aperture sequence 33r. The aperture 3 gets us the
    // vertices, and the 3r gets us back to Class II.
    // vertices listed ccw from the i-axes
    const vertsCII: [CoordIJK; NUM_HEX_VERTS as usize] = [
        CoordIJK { i: 2, j: 1, k: 0 }, // 0
        CoordIJK { i: 1, j: 2, k: 0 }, // 1
        CoordIJK { i: 0, j: 2, k: 1 }, // 2
        CoordIJK { i: 0, j: 1, k: 2 }, // 3
        CoordIJK { i: 1, j: 0, k: 2 }, // 4
        CoordIJK { i: 2, j: 0, k: 1 }, // 5
    ];

    // the vertexes of an origin-centered cell in a Class III resolution on a
    // substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
    // vertices, and the 3r7r gets us to Class II.
    // vertices listed ccw from the i-axes
    const vertsCIII: [CoordIJK; NUM_HEX_VERTS as usize] = [
        CoordIJK { i: 5, j: 4, k: 0 }, // 0
        CoordIJK { i: 1, j: 5, k: 0 }, // 1
        CoordIJK { i: 0, j: 5, k: 4 }, // 2
        CoordIJK { i: 0, j: 1, k: 5 }, // 3
        CoordIJK { i: 4, j: 0, k: 5 }, // 4
        CoordIJK { i: 5, j: 0, k: 1 }, // 5
    ];

    // get the correct set of substrate vertices for this resolution
    let verts: [CoordIJK; NUM_HEX_VERTS as usize];
    if (isResolutionClassIII(*res)) {
        verts = vertsCIII;
    } else {
        verts = vertsCII;
    }

    // adjust the center point to be in an aperture 33r substrate grid
    // these should be composed for speed
    _downAp3(&mut fijk.coord);
    _downAp3r(&mut fijk.coord);

    // if res is Class III we need to add a cw aperture 7 to get to
    // icosahedral Class II
    if (isResolutionClassIII(*res)) {
        _downAp7r(&mut fijk.coord);
        *res += 1;
    }

    // The center point is now in the same substrate grid as the origin
    // cell vertices. Add the center point substate coordinates
    // to each vertex to translate the vertices to that cell.
    for v in 0..NUM_HEX_VERTS {
        fijkVerts[v as usize].face = fijk.face;
        _ijkAdd(
            fijk.coord,
            verts[v as usize],
            &mut fijkVerts[v as usize].coord,
        );
        _ijkNormalize(&mut fijkVerts[v as usize].coord);
    }
}

/**
 * Adjusts a FaceIJK address in place so that the resulting cell address is
 * relative to the correct icosahedral face.
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 * @param pentLeading4 Whether or not the cell is a pentagon with a leading
 *        digit 4.
 * @param substrate Whether or not the cell is in a substrate grid.
 * @return 0 if on original face (no overage); 1 if on face edge (only occurs
 *         on substrate grids); 2 if overage on new face interior
 */
fn _adjustOverageClassII(
    fijk: &mut FaceIJK,
    res: i8,
    pentLeading4: bool,
    substrate: bool,
) -> Overage {
    let mut overage = Overage::NO_OVERAGE;

    let mut ijk = fijk.coord;

    // get the maximum dimension value; scale if a substrate grid
    let mut maxDim = maxDimByCIIres[res as usize];
    if substrate {
        maxDim *= 3;
    }

    // check for overage
    if (substrate && ijk.i + ijk.j + ijk.k == maxDim) {
        // on edge
        overage = Overage::FACE_EDGE;
    } else if (ijk.i + ijk.j + ijk.k > maxDim)
    // overage
    {
        overage = Overage::NEW_FACE;

        let fijkOrient: FaceOrientIJK;
        if (ijk.k > 0) {
            if (ijk.j > 0) {
                // jk "quadrant"
                fijkOrient = faceNeighbors[fijk.face as usize][JK as usize];
            } else
            // ik "quadrant"
            {
                fijkOrient = faceNeighbors[fijk.face as usize][KI as usize];

                // adjust for the pentagonal missing sequence
                if pentLeading4 {
                    // translate origin to center of pentagon
                    let origin = CoordIJK {
                        i: maxDim,
                        j: 0,
                        k: 0,
                    };
                    let mut tmp = CoordIJK { i: 0, j: 0, k: 0 };
                    _ijkSub(ijk, origin, &mut tmp);
                    // rotate to adjust for the missing sequence
                    _ijkRotate60cw(&mut tmp);
                    // translate the origin back to the center of the triangle
                    _ijkAdd(tmp, origin, &mut ijk);
                }
            }
        } else {
            // ij "quadrant"
            fijkOrient = faceNeighbors[fijk.face as usize][IJ as usize];
        }

        fijk.face = fijkOrient.face;

        // rotate and translate for adjacent face
        for i in 0..fijkOrient.ccwRot60 {
            _ijkRotate60ccw(&mut ijk);
        }

        let mut transVec = fijkOrient.translate;
        let mut unitScale = unitScaleByCIIres[res as usize];
        if (substrate) {
            unitScale *= 3;
        }
        _ijkScale(&mut transVec, unitScale);
        _ijkAdd(ijk, transVec, &mut ijk);
        _ijkNormalize(&mut ijk);

        // overage points on pentagon boundaries can end up on edges
        if (substrate && ijk.i + ijk.j + ijk.k == maxDim) {
            // on edge
            overage = Overage::FACE_EDGE;
        }
    }

    return overage;
}

/**
 * Adjusts a FaceIJK address for a pentagon vertex in a substrate grid in
 * place so that the resulting cell address is relative to the correct
 * icosahedral face.
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 */
fn _adjustPentVertOverage(fijk: &mut FaceIJK, res: i8) -> Overage {
    let mut overage: Overage;
    loop {
        overage = _adjustOverageClassII(fijk, res, false, true);
        if (overage != Overage::NEW_FACE) {
            break;
        }
    }
    overage
}

/**
 * Encodes a coordinate on the sphere to the corresponding icosahedral face and
 * containing the squared euclidean distance to that face center.
 *
 * @param g The spherical coordinates to encode.
 * @param face The icosahedral face containing the spherical coordinates.
 * @param sqd The squared euclidean distance to its icosahedral face center.
 */
fn _geoToClosestFace(g: LatLng, face: &mut i8, sqd: &mut f64) {
    let mut v3d = Vec3d {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    _geoToVec3d(g, &mut v3d);

    // determine the icosahedron face
    *face = 0;
    // The distance between two farthest points is 2.0, therefore the square of
    // the distance between two points should always be less or equal than 4.0 .
    *sqd = 5.0;
    for f in 0..NUM_ICOSA_FACES {
        let sqdT = _pointSquareDist(faceCenterPoint[f as usize], v3d);
        if sqdT < *sqd {
            *face = f as i8;
            *sqd = sqdT;
        }
    }
}

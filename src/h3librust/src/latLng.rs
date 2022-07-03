use crate::constants::{
    EARTH_RADIUS_KM, EPSILON, MAX_H3_RES, M_180_PI, M_2PI, M_PI, M_PI_180, M_PI_2,
};
use crate::h3api::{H3Error, LatLng};

/** epsilon of ~0.1mm in degrees */
const EPSILON_DEG: f64 = 0.000000001;
/** epsilon of ~0.1mm in radians */
const EPSILON_RAD: f64 = (EPSILON_DEG * M_PI_180);

/**
 * Normalizes radians to a value between 0.0 and two PI.
 *
 * @param rads The input radians value.
 * @return The normalized radians value.
 */
pub fn _posAngleRads(rads: f64) -> f64 {
    let mut tmp = if rads < 0.0 { rads + M_2PI } else { rads };
    if (rads >= M_2PI) {
        tmp -= M_2PI
    };
    tmp
}

/**
 * Determines if the components of two spherical coordinates are within some
 * threshold distance of each other.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @param threshold The threshold distance.
 * @return Whether or not the two coordinates are within the threshold distance
 *         of each other.
 */
fn geoAlmostEqualThreshold(p1: LatLng, p2: LatLng, threshold: f64) -> bool {
    (p1.lat - p2.lat).abs() < threshold && (p1.lng - p2.lng).abs() < threshold
}

/**
 * Determines if the components of two spherical coordinates are within our
 * standard epsilon distance of each other.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @return Whether or not the two coordinates are within the epsilon distance
 *         of each other.
 */
fn geoAlmostEqual(p1: LatLng, p2: LatLng) -> bool {
    geoAlmostEqualThreshold(p1, p2, EPSILON_RAD)
}

/**
 * Set the components of spherical coordinates in decimal degrees.
 *
 * @param p The spherical coordinates.
 * @param latDegs The desired latitude in decimal degrees.
 * @param lngDegs The desired longitude in decimal degrees.
 */
fn setGeoDegs(p: &mut LatLng, latDegs: f64, lngDegs: f64) {
    _setGeoRads(p, degsToRads(latDegs), degsToRads(lngDegs))
}

/**
 * Set the components of spherical coordinates in radians.
 *
 * @param p The spherical coordinates.
 * @param latRads The desired latitude in decimal radians.
 * @param lngRads The desired longitude in decimal radians.
 */
fn _setGeoRads(p: &mut LatLng, latRads: f64, lngRads: f64) {
    p.lat = latRads;
    p.lng = lngRads;
}

/**
 * Convert from decimal degrees to radians.
 *
 * @param degrees The decimal degrees.
 * @return The corresponding radians.
 */
pub fn degsToRads(degrees: f64) -> f64 {
    degrees * M_PI_180
}

/**
 * Convert from radians to decimal degrees.
 *
 * @param radians The radians.
 * @return The corresponding decimal degrees.
 */
pub fn radsToDegs(radians: f64) -> f64 {
    radians * M_180_PI
}

/**
 * constrainLat makes sure latitudes are in the proper bounds
 *
 * @param lat The original lat value
 * @return The corrected lat value
 */
fn constrainLat(lat: f64) -> f64 {
    let mut latOutput = lat;
    while latOutput > M_PI_2 {
        latOutput = latOutput - M_PI;
    }
    latOutput
}

/**
 * constrainLng makes sure longitudes are in the proper bounds
 *
 * @param lng The origin lng value
 * @return The corrected lng value
 */
fn constrainLng(lng: f64) -> f64 {
    let mut lngOutput = lng;
    while (lngOutput > M_PI) {
        lngOutput = lngOutput - (2. * M_PI);
    }
    while (lngOutput < -M_PI) {
        lngOutput = lngOutput + (2. * M_PI);
    }
    lngOutput
}

/**
 * The great circle distance in radians between two spherical coordinates.
 *
 * This function uses the Haversine formula.
 * For math details, see:
 *     https://en.wikipedia.org/wiki/Haversine_formula
 *     https://www.movable-type.co.uk/scripts/latlong.html
 *
 * @param  a  the first lat/lng pair (in radians)
 * @param  b  the second lat/lng pair (in radians)
 *
 * @return    the great circle distance in radians between a and b
 */
pub fn distanceRads(a: LatLng, b: LatLng) -> f64 {
    let sinLat = ((b.lat - a.lat) / 2.0).sin();
    let sinLng = ((b.lng - a.lng) / 2.0).sin();

    let A: f64 = sinLat * sinLat + a.lat.cos() * b.lat.cos() * sinLng * sinLng;

    return 2.0 * A.sqrt().atan2((1.0 - A).sqrt());
}

/**
 * The great circle distance in kilometers between two spherical coordinates.
 */
pub fn distanceKm(a: LatLng, b: LatLng) -> f64 {
    distanceRads(a, b) * EARTH_RADIUS_KM
}

/**
 * The great circle distance in meters between two spherical coordinates.
 */
pub fn distanceM(a: LatLng, b: LatLng) -> f64 {
    distanceKm(a, b) * 1000.0
}

/**
 * Determines the azimuth to p2 from p1 in radians.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @return The azimuth in radians from p1 to p2.
 */
pub fn _geoAzimuthRads(p1: LatLng, p2: LatLng) -> f64 {
    (p2.lat.cos() * (p2.lng - p1.lng).sin())
        .atan2(p1.lat.cos() * p2.lng.sin() - p1.lat.sin() * p2.lat.cos() * (p2.lng - p1.lng).cos())
}

/**
 * Computes the point on the sphere a specified azimuth and distance from
 * another point.
 *
 * @param p1 The first spherical coordinates.
 * @param az The desired azimuth from p1.
 * @param distance The desired distance from p1, must be non-negative.
 * @param p2 The spherical coordinates at the desired azimuth and distance from
 * p1.
 */
pub fn _geoAzDistanceRads(p1: LatLng, azInput: f64, distance: f64, p2: &mut LatLng) {
    if (distance < EPSILON) {
        *p2 = p1;
        return;
    }

    let az = _posAngleRads(azInput);

    // check for due north/south azimuth
    if (az < EPSILON || (az - M_PI).abs() < EPSILON) {
        if (az < EPSILON) {
            // due north
            p2.lat = p1.lat + distance;
        } else {
            // due south
            p2.lat = p1.lat - distance;
        }

        if ((p2.lat - M_PI_2).abs() < EPSILON)
        // north pole
        {
            p2.lat = M_PI_2;
            p2.lng = 0.0;
        } else if ((p2.lat + M_PI_2).abs() < EPSILON)
        // south pole
        {
            p2.lat = -M_PI_2;
            p2.lng = 0.0;
        } else {
            p2.lng = constrainLng(p1.lng);
        }
    } else
    // not due north or south
    {
        let mut sinlat: f64 =
            p1.lat.sin() * distance.cos() + p1.lat.cos() * distance.sin() * az.cos();
        if (sinlat > 1.0) {
            sinlat = 1.0
        };
        if (sinlat < -1.0) {
            sinlat = -1.0
        };
        p2.lat = sinlat.asin();
        if ((p2.lat - M_PI_2).abs() < EPSILON)
        // north pole
        {
            p2.lat = M_PI_2;
            p2.lng = 0.0;
        } else if ((p2.lat + M_PI_2).abs() < EPSILON)
        // south pole
        {
            p2.lat = -M_PI_2;
            p2.lng = 0.0;
        } else {
            let mut sinlng = az.sin() * distance.sin() / p2.lat.cos();
            let mut coslng =
                (distance.cos() - p1.lat.sin() * p2.lat.sin()) / p1.lat.cos() / p2.lat.cos();
            if (sinlng > 1.0) {
                sinlng = 1.0;
            }
            if (sinlng < -1.0) {
                sinlng = -1.0;
            }
            if (coslng > 1.0) {
                coslng = 1.0;
            }
            if (coslng < -1.0) {
                coslng = -1.0;
            }
            p2.lng = constrainLng(p1.lng + sinlng.atan2(coslng));
        }
    }
}

/*
 * The following functions provide meta information about the H3 hexagons at
 * each zoom level. Since there are only 16 total levels, these are current
 * handled with hardwired static values, but it may be worthwhile to put these
 * static values into another file that can be autogenerated by source code in
 * the future.
 */
pub fn getHexagonAreaAvgKm2(res: i32, out: &mut f64) -> H3Error {
    const areas: [f64; 16] = [
        4.357449416078383e+06,
        6.097884417941332e+05,
        8.680178039899720e+04,
        1.239343465508816e+04,
        1.770347654491307e+03,
        2.529038581819449e+02,
        3.612906216441245e+01,
        5.161293359717191e+00,
        7.373275975944177e-01,
        1.053325134272067e-01,
        1.504750190766435e-02,
        2.149643129451879e-03,
        3.070918756316060e-04,
        4.387026794728296e-05,
        6.267181135324313e-06,
        8.953115907605790e-07,
    ];
    if res < 0 || res > MAX_H3_RES as i32 {
        return H3Error::E_RES_DOMAIN;
    }
    *out = areas[res as usize];
    H3Error::E_SUCCESS
}

// H3Error H3_EXPORT(getHexagonAreaAvgM2)(int res, double *out) {
//     static const double areas[] = {
//         4.357449416078390e+12, 6.097884417941339e+11, 8.680178039899731e+10,
//         1.239343465508818e+10, 1.770347654491309e+09, 2.529038581819452e+08,
//         3.612906216441250e+07, 5.161293359717198e+06, 7.373275975944188e+05,
//         1.053325134272069e+05, 1.504750190766437e+04, 2.149643129451882e+03,
//         3.070918756316063e+02, 4.387026794728301e+01, 6.267181135324322e+00,
//         8.953115907605802e-01};
//     if (res < 0 || res > MAX_H3_RES) {
//         return E_RES_DOMAIN;
//     }
//     *out = areas[res];
//     return E_SUCCESS;
// }

// H3Error H3_EXPORT(getHexagonEdgeLengthAvgKm)(int res, double *out) {
//     static const double lens[] = {
//         1107.712591, 418.6760055, 158.2446558, 59.81085794,
//         22.6063794,  8.544408276, 3.229482772, 1.220629759,
//         0.461354684, 0.174375668, 0.065907807, 0.024910561,
//         0.009415526, 0.003559893, 0.001348575, 0.000509713};
//     if (res < 0 || res > MAX_H3_RES) {
//         return E_RES_DOMAIN;
//     }
//     *out = lens[res];
//     return E_SUCCESS;
// }

// H3Error H3_EXPORT(getHexagonEdgeLengthAvgM)(int res, double *out) {
//     static const double lens[] = {
//         1107712.591, 418676.0055, 158244.6558, 59810.85794,
//         22606.3794,  8544.408276, 3229.482772, 1220.629759,
//         461.3546837, 174.3756681, 65.90780749, 24.9105614,
//         9.415526211, 3.559893033, 1.348574562, 0.509713273};
//     if (res < 0 || res > MAX_H3_RES) {
//         return E_RES_DOMAIN;
//     }
//     *out = lens[res];
//     return E_SUCCESS;
// }

// H3Error H3_EXPORT(getNumCells)(int res, int64_t *out) {
//     if (res < 0 || res > MAX_H3_RES) {
//         return E_RES_DOMAIN;
//     }
//     *out = 2 + 120 * _ipow(7, res);
//     return E_SUCCESS;
// }

// /**
//  * Surface area in radians^2 of spherical triangle on unit sphere.
//  *
//  * For the math, see:
//  * https://en.wikipedia.org/wiki/Spherical_trigonometry#Area_and_spherical_excess
//  *
//  * @param   a  length of triangle side A in radians
//  * @param   b  length of triangle side B in radians
//  * @param   c  length of triangle side C in radians
//  *
//  * @return     area in radians^2 of triangle on unit sphere
//  */
// double triangleEdgeLengthsToArea(double a, double b, double c) {
//     double s = (a + b + c) / 2;

//     a = (s - a) / 2;
//     b = (s - b) / 2;
//     c = (s - c) / 2;
//     s = s / 2;

//     return 4 * atan(sqrt(tan(s) * tan(a) * tan(b) * tan(c)));
// }

// /**
//  * Compute area in radians^2 of a spherical triangle, given its vertices.
//  *
//  * @param   a  vertex lat/lng in radians
//  * @param   b  vertex lat/lng in radians
//  * @param   c  vertex lat/lng in radians
//  *
//  * @return     area of triangle on unit sphere, in radians^2
//  */
// double triangleArea(const LatLng *a, const LatLng *b, const LatLng *c) {
//     return triangleEdgeLengthsToArea(H3_EXPORT(distanceRads)(a, b),
//                                      H3_EXPORT(distanceRads)(b, c),
//                                      H3_EXPORT(distanceRads)(c, a));
// }

// /**
//  * Area of H3 cell in radians^2.
//  *
//  * The area is calculated by breaking the cell into spherical triangles and
//  * summing up their areas. Note that some H3 cells (hexagons and pentagons)
//  * are irregular, and have more than 6 or 5 sides.
//  *
//  * todo: optimize the computation by re-using the edges shared between triangles
//  *
//  * @param   cell  H3 cell
//  * @param    out  cell area in radians^2
//  * @return        E_SUCCESS on success, or an error code otherwise
//  */
// H3Error H3_EXPORT(cellAreaRads2)(H3Index cell, double *out) {
//     LatLng c;
//     CellBoundary cb;
//     H3Error err = H3_EXPORT(cellToLatLng)(cell, &c);
//     if (err) {
//         return err;
//     }
//     err = H3_EXPORT(cellToBoundary)(cell, &cb);
//     if (err) {
//         // TODO: Uncoverable because cellToLatLng will have returned an error
//         // already
//         return err;
//     }

//     double area = 0.0;
//     for (int i = 0; i < cb.numVerts; i++) {
//         int j = (i + 1) % cb.numVerts;
//         area += triangleArea(&cb.verts[i], &cb.verts[j], &c);
//     }

//     *out = area;
//     return E_SUCCESS;
// }

// /**
//  * Area of H3 cell in kilometers^2.
//  */
// H3Error H3_EXPORT(cellAreaKm2)(H3Index cell, double *out) {
//     H3Error err = H3_EXPORT(cellAreaRads2)(cell, out);
//     if (!err) {
//         *out = *out * EARTH_RADIUS_KM * EARTH_RADIUS_KM;
//     }
//     return err;
// }

// /**
//  * Area of H3 cell in meters^2.
//  */
// H3Error H3_EXPORT(cellAreaM2)(H3Index cell, double *out) {
//     H3Error err = H3_EXPORT(cellAreaKm2)(cell, out);
//     if (!err) {
//         *out = *out * 1000 * 1000;
//     }
//     return err;
// }

// /**
//  * Length of a directed edge in radians.
//  *
//  * @param   edge  H3 directed edge
//  *
//  * @return        length in radians
//  */
// H3Error H3_EXPORT(exactEdgeLengthRads)(H3Index edge, double *length) {
//     CellBoundary cb;

//     H3Error err = H3_EXPORT(directedEdgeToBoundary)(edge, &cb);
//     if (err) {
//         return err;
//     }

//     *length = 0.0;
//     for (int i = 0; i < cb.numVerts - 1; i++) {
//         *length += H3_EXPORT(distanceRads)(&cb.verts[i], &cb.verts[i + 1]);
//     }

//     return E_SUCCESS;
// }

// /**
//  * Length of a directed edge in kilometers.
//  */
// H3Error H3_EXPORT(exactEdgeLengthKm)(H3Index edge, double *length) {
//     H3Error err = H3_EXPORT(exactEdgeLengthRads)(edge, length);
//     *length = *length * EARTH_RADIUS_KM;
//     return err;
// }

// /**
//  * Length of a directed edge in meters.
//  */
// H3Error H3_EXPORT(exactEdgeLengthM)(H3Index edge, double *length) {
//     H3Error err = H3_EXPORT(exactEdgeLengthKm)(edge, length);
//     *length = *length * 1000;
//     return err;
// }

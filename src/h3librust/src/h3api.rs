/** Maximum number of cell boundary vertices; worst case is pentagon:
 *  5 original verts + 5 edge crossings
 */
pub const MAX_CELL_BNDRY_VERTS: usize = 10;

/** @struct LatLng
    @brief latitude/longitude in radians
*/
#[derive(Copy, Clone)]
pub struct LatLng {
    // latitude in radians
    pub lat: f64,
    // longitude in radians
    pub lng: f64,
}

/** @struct CellBoundary
    @brief cell boundary in latitude/longitude
*/
pub struct CellBoundary {
    // number of certices
    pub numVerts: i32,
    // vertices in ccw order
    pub verts: [LatLng; MAX_CELL_BNDRY_VERTS],
}

//  /** @struct GeoLoop
//   *  @brief similar to CellBoundary, but requires more alloc work
//   */
//   pub struct GeoLoop {
//     numVerts: i32,
//     verts: &LatLng,
// }

//  /** @struct GeoPolygon
//   *  @brief Simplified core of GeoJSON Polygon coordinates definition
//   */
//   pub struct GeoPolygon {
//     // exterior boundary of the polygon
//     geoloop: GeoLoop,
//     // number of elements in the array pointed to by holes
//     numHoles: i32,
//     // interior boundaries (holes) in the polygon
//     holes: &GeoLoop,
//   }

//  /** @struct GeoMultiPolygon
//   *  @brief Simplified core of GeoJSON MultiPolygon coordinates definition
//   */
//   pub struct GeoMultiPolygon {
//     numPolygons: i32,
//     polygons: &GeoPolygon,
//   }

//  /** @struct LinkedLatLng
//   *  @brief A coordinate node in a linked geo structure, part of a linked list
//   */
//   pub struct LinkedLatLng {
//      vertex: LatLng,
//      next: &LinkedLatLng,
//  }

//  /** @struct LinkedGeoLoop
//   *  @brief A loop node in a linked geo structure, part of a linked list
//   */
//   pub struct LinkedGeoLoop {
//     first: &LinkedLatLng,
//     last: &LinkedLatLng,
//     next: &LinkedGeoLoop,
//  }

//  /** @struct LinkedGeoPolygon
//   *  @brief A polygon node in a linked geo structure, part of a linked list.
//   */
//  pub struct LinkedGeoPolygon {
//     first: &LinkedGeoLoop,
//     last: &LinkedGeoLoop,
//     next: &LinkedGeoPolygon,
//  }

/** @struct CoordIJ
 * @brief IJ hexagon coordinates
 *
 * Each axis is spaced 120 degrees apart.
 */
pub struct CoordIJ {
    // i component
    pub i: i32,
    // j component
    pub j: i32,
}

#[repr(u32)]
pub enum H3Error {
    E_SUCCESS = 0, // Success (no error)
    E_FAILED = 1,  // The operation failed but a more specific error is not available
    E_DOMAIN = 2,  // Argument was outside of acceptable range (when a more
    // specific error code is not available)
    E_LATLNG_DOMAIN = 3, // Latitude or longitude arguments were outside of acceptable range
    E_RES_DOMAIN = 4,    // Resolution argument was outside of acceptable range
    E_CELL_INVALID = 5,  // `H3Index` cell argument was not valid
    E_DIR_EDGE_INVALID = 6, // `H3Index` directed edge argument was not valid
    E_UNDIR_EDGE_INVALID = 7, // `H3Index` undirected edge argument was not valid
    E_VERTEX_INVALID = 8, // `H3Index` vertex argument was not valid
    E_PENTAGON = 9,      // Pentagon distortion was encountered which the algorithm
    // could not handle it
    E_DUPLICATE_INPUT = 10, // Duplicate input was encountered in the arguments
    // and the algorithm could not handle it
    E_NOT_NEIGHBORS = 11,  // `H3Index` cell arguments were not neighbors
    E_RES_MISMATCH = 12,   // `H3Index` cell arguments had incompatible resolutions
    E_MEMORY = 13,         // Necessary memory allocation failed
    E_MEMORY_BOUNDS = 14,  // Bounds of provided memory were not large enough
    E_OPTION_INVALID = 15, // Mode or flags argument was not valid.
}

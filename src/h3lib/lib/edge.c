/*
 * Copyright 2022 Uber Technologies, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *         http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/** @file  edge.c
 * @brief Edge functions for manipulating (undirected) edge indexes.
 */

#include <inttypes.h>
#include <stdbool.h>

#include "algos.h"
#include "constants.h"
#include "coordijk.h"
#include "h3Assert.h"
#include "h3Index.h"
#include "latLng.h"
#include "vertex.h"

/**
 * Reorder the given neighboring cells into a canonical "origin", "destination"
 * order.
 *
 * The ordering created by this function is intended to give most cells the
 * same number of origin as destination directions. This is done using the
 * indexing digits of the cells. When cells are on different base cells, the
 * base cell number is used to determine ordering.
 *
 * Invalid inputs, such as cells at different resolutions, non-neighboring
 * cells, the same cells, etc. will not crash but the ordering produced may not
 * be stable.
 */
void canonicalizeCellOrder(H3Index cell1, H3Index cell2, H3Index *origin,
                           H3Index *destination) {
    int bc1 = H3_GET_BASE_CELL(cell1);
    int bc2 = H3_GET_BASE_CELL(cell2);

    bool ownership;
    if (bc1 != bc2) {
        ownership = bc1 < bc2;
    } else {
        int r = H3_GET_RESOLUTION(cell1);
        if (r != 0) {
            Direction cell1Digit = H3_GET_INDEX_DIGIT(cell1, r - 1);
            Direction cell2Digit = H3_GET_INDEX_DIGIT(cell1, r - 1);
            assert(cell1Digit >= 0 && cell1Digit <= INVALID_DIGIT);
            assert(cell2Digit >= 0 && cell2Digit <= INVALID_DIGIT);
            // + 1 since this table includes values for INVALID_DIGIT
            bool lookupTable[NUM_DIGITS + 1][NUM_DIGITS + 1] = {
                {0, 1, 1, 0, 1, 0, 0, 0}, {0, 0, 0, 1, 1, 1, 0, 0},
                {0, 1, 0, 1, 0, 0, 1, 0}, {1, 0, 0, 0, 1, 0, 1, 0},
                {0, 0, 1, 0, 0, 1, 1, 0}, {1, 0, 1, 1, 0, 0, 0, 0},
                {1, 1, 0, 0, 0, 1, 0, 0}, {1, 1, 1, 1, 1, 1, 1, 1},
            };
            ownership = lookupTable[cell1Digit][cell2Digit];
        } else {
            // Only occurs if the same res0 cells are passed in for cell1 and
            // cell2.
            ownership = false;
        }
    }

    *origin = ownership ? cell1 : cell2;
    *destination = ownership ? cell2 : cell1;
}

/**
 * Wrap the error code from a directed edge function and present
 * undirected edge errors instead.
 */
H3Error wrapDirectedEdgeError(H3Error err) {
    if (err == E_DIR_EDGE_INVALID) {
        return E_UNDIR_EDGE_INVALID;
    }
    return err;
}

/**
 * Allows for operations on an edge index as if it were a directed edge
 * from the edge owner to the neighboring cell.
 * @param edge Undirected edge index
 * @return H3Index Directed edge index
 */
H3Index edgeAsDirectedEdge(H3Index edge) {
    // Do not make indexes that are not edges look "valid".
    if (H3_GET_MODE(edge) == H3_EDGE_MODE) {
        H3_SET_MODE(edge, H3_DIRECTEDEDGE_MODE);
    }
    return edge;
}

/**
 * Returns an edge H3 index based on the provided neighboring cells
 * @param cell1 An H3 hexagon index
 * @param cell2 A neighboring H3 hexagon index
 * @param out Output: the edge H3Index
 */
H3Error H3_EXPORT(cellsToEdge)(H3Index cell1, H3Index cell2, H3Index *out) {
    H3Index origin;
    H3Index dest;
    canonicalizeCellOrder(cell1, cell2, &origin, &dest);
    H3Error edgeErr = H3_EXPORT(cellsToDirectedEdge)(origin, dest, out);
    if (!edgeErr) {
        H3_SET_MODE(*out, H3_EDGE_MODE);
        return edgeErr;
    } else {
        return edgeErr;
    }
}

/**
 * Determines if the provided H3Index is a valid edge index
 * @param edge The edge H3Index
 * @return 1 if it is an edge H3Index, otherwise 0.
 */
int H3_EXPORT(isValidEdge)(H3Index edge) {
    if (H3_GET_MODE(edge) != H3_EDGE_MODE) {
        return 0;
    }
    Direction neighborDirection = H3_GET_RESERVED_BITS(edge);
    if (neighborDirection <= CENTER_DIGIT || neighborDirection >= NUM_DIGITS) {
        return 0;
    }

    H3Index cells[2] = {0};
    // We also rely on the first returned cell being the "owning" cell.
    H3Error cellsResult = H3_EXPORT(edgeToCells)(edge, cells);
    if (cellsResult) {
        return 0;
    }
    if (H3_EXPORT(isPentagon)(cells[0]) && neighborDirection == K_AXES_DIGIT) {
        // Deleted direction from a pentagon.
        return 0;
    }
    H3Index origin, dest;
    canonicalizeCellOrder(cells[0], cells[1], &origin, &dest);
    if (origin != cells[0] || dest != cells[1]) {
        // Not normalized
        return 0;
    }

    // If the owning cell is valid, we expect the destination cell will always
    // be valid.
    return H3_EXPORT(isValidCell)(cells[0]) &&
           ALWAYS(H3_EXPORT(isValidCell)(cells[1]));
}

/**
 * Returns the cell pair of hexagon IDs for the given edge ID.
 *
 * The first cell returned is always the "owning" cell of the edge.
 * @param edge The edge H3Index
 * @param cells Pointer to memory to store cell IDs
 */
H3Error H3_EXPORT(edgeToCells)(H3Index edge, H3Index *cells) {
    // Note: this function will accept directed edges as well, but report
    // E_UNDIR_EDGE_INVALID errors.
    H3Index directedEdge = edgeAsDirectedEdge(edge);
    H3Error cellsResult = H3_EXPORT(directedEdgeToCells)(directedEdge, cells);
    if (cellsResult) {
        return wrapDirectedEdgeError(cellsResult);
    }
    return E_SUCCESS;
}

/**
 * Provides all of the edges from the current H3Index.
 * @param origin The origin hexagon H3Index to find edges for.
 * @param edges The memory to store all of the edges inside.
 */
H3Error H3_EXPORT(cellToEdges)(H3Index origin, H3Index *edges) {
    H3Index neighborRing[7] = {0};
    H3Error gridDiskErr = H3_EXPORT(gridDisk)(origin, 1, neighborRing);
    if (gridDiskErr) {
        return gridDiskErr;
    }
    int edgesIndex = 0;
    for (int i = 0; i < 7; i++) {
        if (neighborRing[i] != origin && neighborRing[i]) {
            H3Error error = H3_EXPORT(cellsToEdge)(origin, neighborRing[i],
                                                   &edges[edgesIndex]);
            if (error) {
                return error;
            }
            edgesIndex++;
        }
    }
    return E_SUCCESS;
}

/**
 * Provides the coordinates defining the edge.
 * @param edge The edge H3Index
 * @param cb The cellboundary object to store the edge coordinates.
 */
H3Error H3_EXPORT(edgeToBoundary)(H3Index edge, CellBoundary *cb) {
    // Note: this function will accept directed edges as well, but report
    // E_UNDIR_EDGE_INVALID errors.
    H3Index directedEdge = edgeAsDirectedEdge(edge);
    return wrapDirectedEdgeError(
        H3_EXPORT(directedEdgeToBoundary)(directedEdge, cb));
}

/**
 * Provides the undirected edge for a given directed edge.
 * @param edge Directed edge
 * @param out Output undirected edge
 */
H3Error H3_EXPORT(directedEdgeToEdge)(H3Index edge, H3Index *out) {
    H3Index originDestination[2] = {0};
    H3Error odError = H3_EXPORT(directedEdgeToCells)(edge, originDestination);
    if (odError) {
        return odError;
    }
    return H3_EXPORT(cellsToEdge)(originDestination[0], originDestination[1],
                                  out);
}

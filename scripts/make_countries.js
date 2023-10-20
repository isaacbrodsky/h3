#!/usr/bin/env node
/*
 * Copyright 2023 Uber Technologies, Inc.
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

/** @file make_countries.js
 * @brief Script to make country test fixtures based on Natural Earth data.
 *        This makes the fixture with polygonToCells benchmarks.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');

// Using GeoJSON version as it is easy to convert
const SOURCE_URL = 'https://raw.githubusercontent.com/martynafford/natural-earth-geojson/master/110m/cultural/ne_110m_admin_0_countries.json';
const TARGET = process.argv[2];

// Use Node HTTPS module for download, to avoid dependencies
function getSource(url) {
  return new Promise((resolve, reject) => {
    https.get(url, (resp) => {
      let data = '';
      resp.on('data', (chunk) => {
        data += chunk;
      });
      resp.on('end', () => {
        resolve(data);
      });
    }).on('error', reject);
  });
}

function degsToRads(deg) {
    return (deg * Math.PI) / 180;
}

function formatCoord([lng, lat]) {
  return `{${degsToRads(lat)}, ${degsToRads(lng)}}`;
}

function formatGeoLoop(loop) {
  return `{
    .numVerts = ${loop.length},
    .verts = (LatLng[]){
      ${loop.map(formatCoord).join(',\n')}
    }
  }`
}

function formatGeoPolygon(poly) {
  const holes = poly.slice(1);

  return `{
    .geoloop = ${formatGeoLoop(poly[0])},
    .numHoles = ${holes.length}${holes.length ? `,
    .holes = (GeoLoop[]) {
      ${holes.map(formatGeoLoop).join(',\n')}
    }` : ''}
  }`
}

async function makeCountries(sourceUrl, targetPath) {
  console.log(`Downloading from ${sourceUrl}...`)
  const countriesJson = await getSource(sourceUrl);
  const countries = JSON.parse(countriesJson);

  console.log(`Download completed, found ${countries.features.length} countries`);

  const polygons = [];
  const names = [];

  let i = 0;

  for (const {geometry, properties: {ADMIN: name}} of countries.features) {
    if (geometry.type === 'Polygon') {
      polygons.push(geometry.coordinates);
      names.push({i: String(i++), name});
    } else if (geometry.type === 'MultiPolygon') {
      polygons.push(...geometry.coordinates);
      let index = String(i++);
      i += geometry.coordinates.length - 1;
      names.push({i: `${index} - ${i}`, name});
    }
  }

  console.log(`Found ${polygons.length} polygons`);

  const out = `/*
   * Copyright 2023 Uber Technologies, Inc.
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
  
  #include "benchmark.h"
  #include "h3api.h"
  #include "polyfill.h"

  /*
  Country names associated with each polygon:
  ${
    names.map(({name, i}) => `${i}. ${name}`).join('\n')
  }
   */

  const GeoPolygon COUNTRIES[${polygons.length}] = {${
    polygons.map(formatGeoPolygon).join(',')
  }};

  BEGIN_BENCHMARKS();
  
  
  int64_t numHexagons;
  H3Index *hexagons;
  
  for (int res = 0; res < 8; res++) {

    printf("Res %d", res);
  
    BENCHMARK(polygonToCells_AllCountries, 5, {
      for (int index = 0; index < ${polygons.length}; index++) {
        H3_EXPORT(maxPolygonToCellsSize)(&COUNTRIES[index], res, 0, &numHexagons);
        hexagons = calloc(numHexagons, sizeof(H3Index));
        H3_EXPORT(polygonToCells)(&COUNTRIES[index], res, 0, hexagons);
        free(hexagons);
      }
    });
  
  }
  
  END_BENCHMARKS();
  `

  const outPath = path.join(__dirname, '..', targetPath);
  fs.writeFileSync(outPath, out, 'utf-8');

  console.log(`Wrote fixture to ${outPath}`);
}

makeCountries(SOURCE_URL, TARGET);




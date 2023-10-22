import contextily as cx
import pandas as pd
import h3
import shapely
import geopandas as gpd
import math
import matplotlib.pyplot as plt

df = pd.read_csv('../build/report.csv')

gdf = gpd.GeoDataFrame(df, geometry=df['cell'].apply(lambda c: shapely.Polygon(h3.cell_to_boundary(c, geo_json=True))), crs="EPSG:4326")

sf_polygon = shapely.Polygon([(math.degrees(y), math.degrees(x)) for x, y in [(0.659966917655, -2.1364398519396),  (0.6595011102219, -2.1359434279405),
    (0.6583348114025, -2.1354884206045), (0.6581220034068, -2.1382437718946),
    (0.6594479998527, -2.1384597563896), (0.6599990002976, -2.1376771158464)]])

saved_polys = []

buf = 0.05

for i, x in enumerate(gdf.iterrows()):
    print(i)
    t = x[1].type
    if t == 'eval':
        t = 'b'
    elif t == 'eval2':
        t = 'y'
    elif t in ['found', 'found2']:
        t = 'g'
        saved_polys.append(x[1].geometry.boundary)

    # Note: degrees buffer
    ax = gpd.GeoSeries([x[1].geometry.boundary, sf_polygon.boundary, *saved_polys], crs="EPSG:4326").plot(color=[t, 'r', *(['g'] * len(saved_polys))])
    ax.set_xlim((sf_polygon.bounds[0] - buf, sf_polygon.bounds[2] + buf))
    ax.set_ylim((sf_polygon.bounds[1] - buf, sf_polygon.bounds[3] + buf))
    cx.add_basemap(ax, crs="EPSG:4326", source=cx.providers.CartoDB.Positron)
    plt.savefig(f'out/{i}')
    plt.close()


# %%
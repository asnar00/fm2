#!/usr/bin/env python3
"""Vendor country outlines for /map/country-icon.

Natural Earth 1:110m admin-0 countries — public domain, pinned by commit —
reduced to what an icon and a point-in-country test actually need: per
country an ISO code, a bounding box, and simplified rings.

The binary stays out of git (the recipe is the record, as with fetch_stt.py
and fetch_find.py); run this to produce it.

  tools/fetch_countries.py
"""

import json
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT = REPO / "features/miso/loop/map/country-icon/assets/geo/countries.json"

# pinned revision of natural-earth-vector; 110m is the icon-scale dataset
REV = "ca96624a56bd078437bca8184e78163e5039ad19"
URL = ("https://raw.githubusercontent.com/nvkelso/natural-earth-vector/"
       f"{REV}/geojson/ne_110m_admin_0_countries.geojson")

# ~5.5km at the equator: far finer than a 24px icon can show, and inside
# the error a phone's own fix carries anyway
TOL = 0.05


def simplify(ring):
    """Drop points that add nothing at icon scale; keep the ring closed."""
    out = [ring[0]]
    for p in ring[1:]:
        q = out[-1]
        if abs(p[0] - q[0]) >= TOL or abs(p[1] - q[1]) >= TOL:
            out.append(p)
    if len(out) < 4:
        return None
    if out[0] != out[-1]:
        out.append(out[0])
    return [[round(x, 2), round(y, 2)] for x, y in out]


def area(ring):
    s = 0.0
    for i in range(len(ring) - 1):
        s += ring[i][0] * ring[i + 1][1] - ring[i + 1][0] * ring[i][1]
    return abs(s) / 2.0


def main():
    print(f"fetching {URL}")
    raw = subprocess.run(["curl", "-sL", "--max-time", "120", URL],
                         capture_output=True).stdout
    if len(raw) < 10000:
        sys.exit("fetch failed (is the revision still there?)")
    gj = json.loads(raw)

    countries = {}
    for f in gj["features"]:
        p = f["properties"]
        code = (p.get("ISO_A2_EH") or p.get("ISO_A2") or "").strip()
        if not code or code == "-99":
            continue
        name = p.get("NAME") or p.get("ADMIN") or code
        geom = f["geometry"]
        polys = ([geom["coordinates"]] if geom["type"] == "Polygon"
                 else geom["coordinates"])
        rings = []
        for poly in polys:
            r = simplify(poly[0])          # outer ring only; holes don't show
            if r:
                rings.append(r)
        if not rings:
            continue
        # keep the mainland plus anything big enough to read as part of the
        # shape; a hundred specks would only muddy a 24px silhouette
        rings.sort(key=area, reverse=True)
        biggest = area(rings[0])
        rings = [r for r in rings if area(r) >= biggest * 0.02][:12]
        xs = [x for r in rings for x, _ in r]
        ys = [y for r in rings for _, y in r]
        countries[code] = {
            "n": name,
            "b": [round(min(xs), 2), round(min(ys), 2),
                  round(max(xs), 2), round(max(ys), 2)],
            "r": rings,
        }

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(countries, separators=(",", ":")))
    kb = OUT.stat().st_size / 1024
    print(f"wrote {OUT.relative_to(REPO)} — {len(countries)} countries, {kb:.0f}KB")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Pull every named street and place inside the constituency into the mini's
context store, once, so transcription can be seeded offline afterwards.

    python3 tools/streets.py            # dry: says what it would fetch
    python3 tools/streets.py --go       # fetch and write
    python3 tools/streets.py --show     # what is stored now

The boundary comes from /boundaries' own committed geojson — the same file the
map draws — so nothing here knows the name of a constituency. The pull is one
Overpass query for `highway` ways with a `name` and for named `place` nodes
inside the boundary's bounding box, filtered afterwards against the polygon
itself. The answer is written to $MISO_CONTEXT_DIR/streets.json (default
~/.miso-context/streets.json), which is outside the synced tree: a deploy
cannot touch it and a re-run is the only thing that changes it.

Shape:
  {"generated": <ms>, "constituency": "Sevenoaks", "source": "overpass",
   "items": [{"name": "High Street", "lat": 51.27, "lon": 0.19}, ...]}

Each name appears once per distinct place: a street that runs through three
wards is three entries only if OSM has three separate named ways far apart,
which is what makes "nearest thirty" mean something.
"""
import argparse
import json
import os
import pathlib
import sys
import time
import urllib.error
import urllib.request

REPO = pathlib.Path(__file__).resolve().parent.parent
BOUNDARIES = (REPO / "features/miso/loop/cards/browse/map/basemap/boundaries"
              / "assets/map/boundaries.geojson")
OVERPASS = "https://overpass-api.de/api/interpreter"
UA = "miso/1.0 (campaign field notes; one-off street pull)"


def context_dir() -> pathlib.Path:
    d = os.environ.get("MISO_CONTEXT_DIR")
    return pathlib.Path(d) if d else pathlib.Path.home() / ".miso-context"


def out_file() -> pathlib.Path:
    return context_dir() / "streets.json"


def constituency():
    """The constituency feature, its name and its rings (lon, lat pairs)."""
    doc = json.loads(BOUNDARIES.read_text())
    for f in doc["features"]:
        if f["properties"].get("kind") != "constituency":
            continue
        g = f["geometry"]
        rings = [g["coordinates"][0]] if g["type"] == "Polygon" else [
            poly[0] for poly in g["coordinates"]]
        return f["properties"].get("name", ""), rings
    sys.exit("streets: no constituency feature in boundaries.geojson")


def bbox(rings):
    lats = [p[1] for r in rings for p in r]
    lons = [p[0] for r in rings for p in r]
    return min(lats), min(lons), max(lats), max(lons)


def inside(rings, lat, lon) -> bool:
    """Ray casting against every ring; a point in any ring is inside. The
    constituency is one simple polygon, so no hole arithmetic is needed."""
    for ring in rings:
        hit = False
        n = len(ring)
        for i in range(n):
            x1, y1 = ring[i][0], ring[i][1]
            x2, y2 = ring[(i + 1) % n][0], ring[(i + 1) % n][1]
            if (y1 > lat) != (y2 > lat):
                xat = x1 + (lat - y1) * (x2 - x1) / (y2 - y1)
                if lon < xat:
                    hit = not hit
        if hit:
            return True
    return False


def query(s, w, n, e) -> str:
    return f"""[out:json][timeout:180];
(
  way["highway"]["name"]({s},{w},{n},{e});
  node["place"]["name"]({s},{w},{n},{e});
);
out center tags;
"""


def fetch(q: str):
    req = urllib.request.Request(OVERPASS, data=q.encode("utf-8"),
                                 headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=300) as r:
        return json.loads(r.read().decode("utf-8"))


def collect(payload, rings):
    """One entry per name per distinct spot. Two ways with the same name
    within about 300 m are the same street cut into segments by OSM, so the
    first one wins; further away they are kept apart, because "nearest" then
    means something different at either end of a long road."""
    seen = {}
    out = []
    for el in payload.get("elements", []):
        name = (el.get("tags") or {}).get("name")
        if not name:
            continue
        if el["type"] == "node":
            lat, lon = el.get("lat"), el.get("lon")
        else:
            c = el.get("center") or {}
            lat, lon = c.get("lat"), c.get("lon")
        if lat is None or lon is None:
            continue
        if not inside(rings, lat, lon):
            continue
        near = False
        for plat, plon in seen.get(name, []):
            if abs(plat - lat) < 0.0027 and abs(plon - lon) < 0.0043:
                near = True
                break
        if near:
            continue
        seen.setdefault(name, []).append((lat, lon))
        out.append({"name": name, "lat": round(lat, 5), "lon": round(lon, 5)})
    out.sort(key=lambda e: (e["name"], e["lat"]))
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--go", action="store_true", help="fetch and write")
    ap.add_argument("--show", action="store_true", help="what is stored now")
    args = ap.parse_args()

    if args.show:
        f = out_file()
        if not f.exists():
            print(f"streets: nothing at {f}")
            return
        d = json.loads(f.read_text())
        print(f"{f}: {len(d['items'])} named places in {d.get('constituency')}, "
              f"generated {time.strftime('%Y-%m-%d %H:%M', time.localtime(d['generated'] / 1000))}")
        for e in d["items"][:12]:
            print(f"  {e['name']}  {e['lat']},{e['lon']}")
        return

    name, rings = constituency()
    s, w, n, e = bbox(rings)
    print(f"streets: {name}, bounding box {s:.4f},{w:.4f} to {n:.4f},{e:.4f}")
    if not args.go:
        print("streets: dry run — pass --go to fetch from Overpass and write "
              f"{out_file()}")
        return
    try:
        payload = fetch(query(s, w, n, e))
    except (urllib.error.URLError, TimeoutError) as err:
        sys.exit(f"streets: Overpass would not answer ({err}) — try again later; "
                 "nothing was written")
    items = collect(payload, rings)
    if not items:
        sys.exit("streets: Overpass answered with nothing inside the boundary — "
                 "nothing written, since an empty file would read as 'no streets "
                 "here' for ever")
    out_file().parent.mkdir(parents=True, exist_ok=True)
    out_file().write_text(json.dumps({
        "generated": int(time.time() * 1000),
        "constituency": name,
        "source": "overpass",
        "items": items,
    }))
    print(f"streets: {len(items)} named places written to {out_file()}")


if __name__ == "__main__":
    main()

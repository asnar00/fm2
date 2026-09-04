struct feature_LinesToo;
impl feature_LinesToo {
    // ---- the ink ------------------------------------------------------------
    // The boundary lines move into the squares, beside the region's own ground.
    // Two styles, and they are a DUPLICATION of the two that draw the vector
    // layer today — /outlined's black ward line and /light-basemap's dashed
    // constituency edge — because those live in JavaScript and this runs on the
    // server. They are stated once here, they ride in the cache stamp, and if
    // one is changed the other has to change with it. That is the price of
    // baking; the alternative was the page telling the server what colour to
    // use, which would have made a tile url a style sheet.
    //
    // Widths are in TILE pixels, which is the same as CSS pixels — Leaflet
    // draws a 256-pixel square into a 256-CSS-pixel box — so a line is the
    // same width on the screen at every zoom, which is what a boundary is
    // supposed to look like. It is not a width on the ground.

    fn lines_ward_width() -> f64 {
        1.2
    }

    fn lines_edge_width() -> f64 {
        2.0
    }

    // "7 5" — /light-basemap's dashArray, in the same tile pixels
    fn lines_dash_on() -> f64 {
        7.0
    }

    fn lines_dash_off() -> f64 {
        5.0
    }

    // the style, as a string, so a change of any of it lands in the cache
    // generation and re-bakes rather than leaving old squares with old ink
    fn lines_style() -> String {
        format!("ward#000000@{};edge#4a4a54@{};dash{}/{}",
                lines_ward_width(), lines_edge_width(),
                lines_dash_on(), lines_dash_off())
    }

    fn baked_stamp() -> String {
        format!("{}-{:016x}", existing.baked_stamp(), lines_fnv(lines_style()))
    }

    fn lines_fnv(s: String) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in s.as_bytes().iter() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }

    // ---- the segments -------------------------------------------------------
    // Every ring of every feature of one kind, projected into this square's own
    // pixel space and cut down to the ones that can reach it. Flat, as
    // /baked's rings are, because a vector of vectors is a comma-bearing type:
    // [ax, ay, bx, by, ...].
    //
    // The constituency's dash is applied HERE, while the ring is being walked,
    // from a length accumulated since the ring's first point — so the pattern
    // depends on the ring and the zoom and not on which square it is being
    // drawn into, and a dash never jumps at a tile edge.

    fn lines_segs(kind: String, z: u32, x: u32, y: u32) -> Vec<f64> {
        let raw = match std::fs::read_to_string(baked_file_path()) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let d: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };
        let world = 256.0 * ((1u64 << z) as f64);
        let ox = (x as f64) * 256.0;
        let oy = (y as f64) * 256.0;
        let dashed = kind == "constituency";
        let half = if dashed { lines_edge_width() } else { lines_ward_width() } / 2.0;
        let pad = half + 1.0;
        let mut out: Vec<f64> = Vec::new();
        let empty = Vec::new();
        for f in d["features"].as_array().unwrap_or(&empty).iter() {
            if f["properties"]["kind"].as_str() != Some(kind.as_str()) {
                continue;
            }
            let g = f["geometry"].clone();
            let mut parts: Vec<serde_json::Value> = Vec::new();
            let t = g["type"].as_str().unwrap_or("");
            if t == "Polygon" {
                parts.push(g["coordinates"].clone());
            } else if t == "MultiPolygon" {
                for p in g["coordinates"].as_array().unwrap_or(&empty).iter() {
                    parts.push(p.clone());
                }
            }
            for part in parts.iter() {
                for ring in part.as_array().unwrap_or(&empty).iter() {
                    let pts = ring.as_array().unwrap_or(&empty);
                    if pts.len() < 2 {
                        continue;
                    }
                    let mut run = 0.0f64;      // length walked along this ring
                    let mut px0 = 0.0f64;
                    let mut py0 = 0.0f64;
                    for i in 0..pts.len() {
                        let c = pts[i].as_array().unwrap_or(&empty);
                        if c.len() < 2 {
                            continue;
                        }
                        let cx = baked_px(c[0].as_f64().unwrap_or(0.0), world) - ox;
                        let cy = baked_py(c[1].as_f64().unwrap_or(0.0), world) - oy;
                        if i > 0 {
                            let seg = ((cx - px0) * (cx - px0)
                                       + (cy - py0) * (cy - py0)).sqrt();
                            if dashed {
                                lines_dashed(&mut out, px0, py0, cx, cy, run, seg,
                                             pad);
                            } else {
                                lines_keep(&mut out, px0, py0, cx, cy, pad);
                            }
                            run += seg;
                        }
                        px0 = cx;
                        py0 = cy;
                    }
                }
            }
        }
        out
    }

    // one segment, kept only if its own box can reach this square
    fn lines_keep(out: &mut Vec<f64>, ax: f64, ay: f64, bx: f64, by: f64,
                  pad: f64) {
        let lox = if ax < bx { ax } else { bx } - pad;
        let hix = if ax > bx { ax } else { bx } + pad;
        let loy = if ay < by { ay } else { by } - pad;
        let hiy = if ay > by { ay } else { by } + pad;
        if hix < 0.0 || lox > 256.0 || hiy < 0.0 || loy > 256.0 {
            return;
        }
        out.push(ax);
        out.push(ay);
        out.push(bx);
        out.push(by);
    }

    // the same segment cut into the "on" pieces of the dash pattern, using a
    // length measured from the ring's start so the pattern is the ring's and
    // not the square's
    fn lines_dashed(out: &mut Vec<f64>, ax: f64, ay: f64, bx: f64, by: f64,
                    run: f64, seg: f64, pad: f64) {
        if seg <= 0.0 {
            return;
        }
        let period = lines_dash_on() + lines_dash_off();
        let mut s = 0.0f64;
        while s < seg {
            let at = (run + s) % period;
            let left = if at < lines_dash_on() {
                lines_dash_on() - at
            } else {
                period - at
            };
            let step = if left < seg - s { left } else { seg - s };
            if at < lines_dash_on() {
                let t0 = s / seg;
                let t1 = (s + step) / seg;
                lines_keep(out,
                           ax + (bx - ax) * t0, ay + (by - ay) * t0,
                           ax + (bx - ax) * t1, ay + (by - ay) * t1,
                           pad);
            }
            s += if step > 0.0001 { step } else { 0.0001 };
        }
    }

    // ---- the rasteriser -----------------------------------------------------
    // The same shape as /baked's mask: four sub-scanlines per output row, exact
    // horizontal overlap into a row accumulator, one byte of alpha out. What
    // differs is the shape being filled — a stroke is the set of points within
    // half a width of a segment, a capsule, and a capsule is convex, so its
    // intersection with one scanline is a single interval. No 1024 × 1024
    // buffer is made: the working set stays two rows and a segment list, which
    // is what let the bake run in five megabytes and must keep doing so.
    //
    // Segments are bucketed by output row first. At zoom 11 the whole district
    // is inside one square and there are two thousand of them; without the
    // buckets every row would test every segment.

    fn lines_ink(segs: &Vec<f64>, half: f64) -> Vec<u8> {
        let mut ink = vec![0u8; 256 * 256];
        if segs.len() < 4 {
            return ink;
        }
        let mut buckets: Vec<Vec<usize>> = Vec::new();
        for _ in 0..256 {
            buckets.push(Vec::new());
        }
        let mut e = 0usize;
        while e + 3 < segs.len() {
            let ay = segs[e + 1];
            let by = segs[e + 3];
            let mut lo = (if ay < by { ay } else { by }) - half - 1.0;
            let mut hi = (if ay > by { ay } else { by }) + half + 1.0;
            if lo < 0.0 { lo = 0.0; }
            if hi > 255.0 { hi = 255.0; }
            if hi >= lo {
                let r0 = lo as usize;
                let r1 = hi as usize;
                for r in r0..(r1 + 1) {
                    if r < 256 {
                        buckets[r].push(e);
                    }
                }
            }
            e += 4;
        }
        let mut acc = vec![0.0f32; 256];
        for row in 0..256usize {
            if buckets[row].is_empty() {
                continue;
            }
            for v in acc.iter_mut() {
                *v = 0.0;
            }
            for sub in 0..4usize {
                let yl = row as f64 + (sub as f64 + 0.5) / 4.0;
                for at in buckets[row].iter() {
                    let i = *at;
                    let span = lines_at(segs[i], segs[i + 1], segs[i + 2],
                                        segs[i + 3], half, yl);
                    if span.len() == 2 {
                        baked_span(&mut acc, span[0], span[1]);
                    }
                }
            }
            for c in 0..256usize {
                let v = acc[c] * 0.25 * 255.0;
                let a = if v <= 0.0 {
                    0
                } else if v >= 255.0 {
                    255
                } else {
                    v as u8
                };
                if a > ink[row * 256 + c] {
                    ink[row * 256 + c] = a;
                }
            }
        }
        ink
    }

    // where one capsule meets one scanline: [lo, hi], or empty. A capsule is
    // the union of two discs and the rectangle between them, and it is convex,
    // so the answer is the least and greatest x of whichever of the three the
    // line actually crosses.
    fn lines_at(ax: f64, ay: f64, bx: f64, by: f64, half: f64, yl: f64) -> Vec<f64> {
        let mut lo = 1.0e30f64;
        let mut hi = -1.0e30f64;
        let mut hit = false;
        // the two end discs
        let da = yl - ay;
        if da.abs() <= half {
            let s = (half * half - da * da).sqrt();
            if ax - s < lo { lo = ax - s; }
            if ax + s > hi { hi = ax + s; }
            hit = true;
        }
        let db = yl - by;
        if db.abs() <= half {
            let s = (half * half - db * db).sqrt();
            if bx - s < lo { lo = bx - s; }
            if bx + s > hi { hi = bx + s; }
            hit = true;
        }
        // the rectangle between them: the segment offset by half a width each
        // way along its own normal
        let dx = bx - ax;
        let dy = by - ay;
        let len = (dx * dx + dy * dy).sqrt();
        if len > 1.0e-9 {
            let nx = -dy / len * half;
            let ny = dx / len * half;
            let cx = [ax + nx, bx + nx, bx - nx, ax - nx];
            let cy = [ay + ny, by + ny, by - ny, ay - ny];
            for k in 0..4usize {
                let j = (k + 1) % 4;
                let y0 = cy[k];
                let y1 = cy[j];
                if (y0 <= yl && y1 > yl) || (y1 <= yl && y0 > yl) {
                    let t = (yl - y0) / (y1 - y0);
                    let xx = cx[k] + (cx[j] - cx[k]) * t;
                    if xx < lo { lo = xx; }
                    if xx > hi { hi = xx; }
                    hit = true;
                }
            }
        }
        let mut out: Vec<f64> = Vec::new();
        if hit && hi > lo {
            out.push(lo);
            out.push(hi);
        }
        out
    }

    // ---- painting -----------------------------------------------------------

    fn lines_paint(rgb: Vec<u8>, ink: &Vec<u8>, r: u8, g: u8, b: u8) -> Vec<u8> {
        let n = 256usize * 256usize;
        let mut out = rgb;
        for i in 0..n {
            let a = ink[i] as u32;
            if a == 0 {
                continue;
            }
            let k = 255 - a;
            let c = [r as u32, g as u32, b as u32];
            for ch in 0..3usize {
                let was = out[i * 3 + ch] as u32;
                out[i * 3 + ch] = (((was * k + c[ch] * a) + 127) / 255) as u8;
            }
        }
        out
    }

    // ---- the two seams /baked opened ----------------------------------------
    // The constituency's dashed edge goes down first and the wards over it, so
    // that where they run together the ward's black wins — which is the order
    // the vector layer drew them in, the features being in that order in the
    // file.

    fn baked_extra(rgb: Vec<u8>, code: String, z: u32, x: u32, y: u32) -> Vec<u8> {
        let base = existing.baked_extra(rgb, code, z, x, y);
        let edge = lines_segs("constituency".to_string(), z, x, y);
        let ward = lines_segs("ward".to_string(), z, x, y);
        let mut out = base;
        if edge.len() >= 4 {
            let ink = lines_ink(&edge, lines_edge_width() / 2.0);
            out = lines_paint(out, &ink, 0x4a, 0x4a, 0x54);
        }
        if ward.len() >= 4 {
            let ink = lines_ink(&ward, lines_ward_width() / 2.0);
            out = lines_paint(out, &ink, 0x00, 0x00, 0x00);
        }
        out
    }

    // a square any boundary line reaches has to be composited, whatever the
    // region says about it — that is the whole of this node's ask. The test is
    // the segments' own boxes, not the file's, so a square in the middle of a
    // big ward is still passed through untouched and still costs no disk.
    fn baked_must(code: String, z: u32, x: u32, y: u32) -> bool {
        if existing.baked_must(code, z, x, y) {
            return true;
        }
        if !lines_segs("ward".to_string(), z, x, y).is_empty() {
            return true;
        }
        !lines_segs("constituency".to_string(), z, x, y).is_empty()
    }
}

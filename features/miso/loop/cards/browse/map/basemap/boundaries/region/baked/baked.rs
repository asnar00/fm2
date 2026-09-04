struct feature_Baked;
impl feature_Baked {
    // ---- the baked ground --------------------------------------------------
    // `tiles/region/{code}/{z}/{x}/{y}.png` — the everyday ground square with
    // the Outdoors square drawn over it inside the region's polygon, so the
    // boundary lives in the pixels and the map has ONE ground to scale.
    //
    // This route is claimed ahead of /region's `tiles/outdoors/` and /tiles'
    // `tiles/` because this node is newer than both and its link therefore
    // runs first. It answers only its own prefix.

    fn route(r: request) -> response {
        if r.path.starts_with("tiles/region/") {
            return baked_serve(r.path.clone());
        }
        existing.route(r)
    }

    // ---- the parser --------------------------------------------------------
    // Two functions and not one because the chain parser cannot read a
    // comma-bearing return type, and a code plus three numbers is one. Both
    // must answer for the route to serve anything.
    //
    // The code is letters and digits only, at most twenty: a name of that
    // shape cannot contain a separator, so no path this accepts can leave the
    // directory it is joined to. That is /tiles' whole security argument, one
    // segment further along.

    fn baked_code(path: String) -> String {
        let rest = match path.strip_prefix("tiles/region/") {
            Some(s) => s.to_string(),
            None => return String::new(),
        };
        let code = match rest.split('/').next() {
            Some(c) => c.to_string(),
            None => return String::new(),
        };
        if code.is_empty() || code.len() > 20 {
            return String::new();
        }
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return String::new();
        }
        code
    }

    fn baked_coords(path: String) -> Vec<u32> {
        let rest = match path.strip_prefix("tiles/region/") {
            Some(s) => s.to_string(),
            None => return Vec::new(),
        };
        let rest = match rest.strip_suffix(".png") {
            Some(s) => s.to_string(),
            None => return Vec::new(),
        };
        let parts: Vec<String> = rest.split('/').map(|p| p.to_string()).collect();
        if parts.len() != 4 {
            return Vec::new();
        }
        let mut out: Vec<u32> = Vec::new();
        for p in parts.iter().skip(1) {
            if p.is_empty() || p.len() > 8 {
                return Vec::new();
            }
            if !p.chars().all(|c| c.is_ascii_digit()) {
                return Vec::new();
            }
            match p.parse::<u32>() {
                Ok(n) => out.push(n),
                Err(_) => return Vec::new(),
            }
        }
        if out[0] > 19 {
            return Vec::new();
        }
        let side = 1u32 << out[0];
        if out[1] >= side || out[2] >= side {
            return Vec::new();
        }
        out
    }

    // ---- where the baked squares live --------------------------------------

    fn baked_context() -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        match std::env::var("MISO_CONTEXT_DIR") {
            Ok(d) => {
                if d.is_empty() {
                    format!("{}/.miso-context", home)
                } else {
                    d
                }
            }
            Err(_) => format!("{}/.miso-context", home),
        }
    }

    fn baked_dir() -> String {
        format!("{}/tiles-baked", baked_context())
    }

    // the cache generation: everything that could change the picture and
    // nothing that could not. The boundary file's bytes, and the two basemap
    // urls with their QUERIES REMOVED — the Stadia key rides in the query, and
    // rotating a key must not throw away a bake. FNV-1a because this is a
    // cache key and not a signature; nothing about it has to resist anyone.
    fn baked_stamp() -> String {
        let mut h: u64 = 0xcbf29ce484222325;
        let file = std::fs::read(baked_file_path()).unwrap_or_default();
        for b in file.iter() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        let mut mix = baked_strip_query(baked_ground_url());
        mix.push('|');
        mix.push_str(&baked_strip_query(region_source()));
        for b in mix.as_bytes().iter() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", h)
    }

    fn baked_strip_query(url: String) -> String {
        match url.find('?') {
            Some(i) => url[..i].to_string(),
            None => url,
        }
    }

    // ---- the polygon -------------------------------------------------------

    fn baked_file_path() -> String {
        "site/map/boundaries.geojson".to_string()
    }

    fn baked_geo(code: String) -> serde_json::Value {
        let raw = match std::fs::read_to_string(baked_file_path()) {
            Ok(s) => s,
            Err(_) => return serde_json::Value::Null,
        };
        let d: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(_) => return serde_json::Value::Null,
        };
        let empty = Vec::new();
        for f in d["features"].as_array().unwrap_or(&empty).iter() {
            // case-insensitively, because the CACHE path is: this box's
            // filesystem folds case, so `e14001465` read a square baked for
            // `E14001465` while an exact match here would have called the
            // same url a 404 when the cache was cold. One url must not be two
            // answers, so the lookup folds too — ONS codes are unique either
            // way, and the cache directory is upper-cased so one region keeps
            // one directory.
            match f["properties"]["code"].as_str() {
                Some(c) => {
                    if c.eq_ignore_ascii_case(code.as_str()) {
                        return f["geometry"].clone();
                    }
                }
                None => {}
            }
        }
        serde_json::Value::Null
    }

    // every ring of a Polygon, or of every part of a MultiPolygon, flattened
    // into one vector: [n0, lon, lat, lon, lat, ..., n1, lon, lat, ...] where
    // nK is the POINT count of ring K. Flat because a vector of vectors is a
    // comma-bearing type the chain parser cannot read — and because the
    // rasteriser wants one contiguous run of numbers anyway.
    fn baked_rings(geo: serde_json::Value) -> Vec<f64> {
        let kind = geo["type"].as_str().unwrap_or("");
        let empty = Vec::new();
        let mut parts: Vec<serde_json::Value> = Vec::new();
        if kind == "Polygon" {
            parts.push(geo["coordinates"].clone());
        } else if kind == "MultiPolygon" {
            for p in geo["coordinates"].as_array().unwrap_or(&empty).iter() {
                parts.push(p.clone());
            }
        } else {
            return Vec::new();
        }
        let mut out: Vec<f64> = Vec::new();
        for part in parts.iter() {
            for ring in part.as_array().unwrap_or(&empty).iter() {
                let pts = ring.as_array().unwrap_or(&empty);
                if pts.len() < 3 {
                    continue;
                }
                out.push(pts.len() as f64);
                for p in pts.iter() {
                    let c = p.as_array().unwrap_or(&empty);
                    if c.len() < 2 {
                        out.push(0.0);
                        out.push(0.0);
                        continue;
                    }
                    out.push(c[0].as_f64().unwrap_or(0.0));
                    out.push(c[1].as_f64().unwrap_or(0.0));
                }
            }
        }
        out
    }

    // [west, south, east, north]; empty for no rings
    fn baked_box(rings: &Vec<f64>) -> Vec<f64> {
        let mut i = 0usize;
        let mut w = 1.0e30;
        let mut s = 1.0e30;
        let mut e = -1.0e30;
        let mut n = -1.0e30;
        let mut any = false;
        while i < rings.len() {
            let count = rings[i] as usize;
            i += 1;
            for k in 0..count {
                let at = i + k * 2;
                if at + 1 >= rings.len() {
                    break;
                }
                let lon = rings[at];
                let lat = rings[at + 1];
                if lon < w { w = lon; }
                if lon > e { e = lon; }
                if lat < s { s = lat; }
                if lat > n { n = lat; }
                any = true;
            }
            i += count * 2;
        }
        if !any {
            return Vec::new();
        }
        let mut out: Vec<f64> = Vec::new();
        out.push(w);
        out.push(s);
        out.push(e);
        out.push(n);
        out
    }

    // ---- Web Mercator ------------------------------------------------------

    fn baked_px(lon: f64, world: f64) -> f64 {
        (lon + 180.0) / 360.0 * world
    }

    fn baked_py(lat: f64, world: f64) -> f64 {
        // the projection stops being finite at the poles; the slippy scheme's
        // own limit is 85.0511, and every boundary we draw is at 51
        let mut l = lat;
        if l > 85.05112878 { l = 85.05112878; }
        if l < -85.05112878 { l = -85.05112878; }
        let r = l.to_radians();
        let s = (r.tan() + 1.0 / r.cos()).ln() / std::f64::consts::PI;
        (1.0 - s) / 2.0 * world
    }

    // ---- the mask ----------------------------------------------------------
    // One byte of coverage per pixel, 256×256.
    //
    // Four sub-scanlines per output row — the ×4 the brief asked for — and in
    // the x direction the span's EXACT overlap with each pixel rather than a
    // subpixel count, which is both more accurate than ×4 and cheaper, since
    // the crossings are already real numbers. Even-odd on the sorted crossings,
    // so a hole is a hole and a detached part is a part without this code
    // having to know which ring is which or which way it was wound.
    //
    // The working set is two 256-wide rows and one crossing list: the mask
    // itself is the only 64 KB here, and nothing 1024 × 1024 is ever made.

    fn baked_mask(rings: &Vec<f64>, z: u32, x: u32, y: u32) -> Vec<u8> {
        let world = 256.0 * ((1u64 << z) as f64);
        let ox = (x as f64) * 256.0;
        let oy = (y as f64) * 256.0;
        // the edges, in tile-local pixel space: [ax, ay, bx, by, ...]
        let mut edges: Vec<f64> = Vec::new();
        let mut i = 0usize;
        while i < rings.len() {
            let count = rings[i] as usize;
            i += 1;
            let mut prev_x = 0.0;
            let mut prev_y = 0.0;
            for k in 0..count {
                let at = i + k * 2;
                if at + 1 >= rings.len() {
                    break;
                }
                let cx = baked_px(rings[at], world) - ox;
                let cy = baked_py(rings[at + 1], world) - oy;
                if k > 0 {
                    edges.push(prev_x);
                    edges.push(prev_y);
                    edges.push(cx);
                    edges.push(cy);
                }
                prev_x = cx;
                prev_y = cy;
            }
            i += count * 2;
        }
        let mut mask = vec![0u8; 256 * 256];
        if edges.is_empty() {
            return mask;
        }
        let mut acc = vec![0.0f32; 256];
        let mut xs: Vec<f64> = Vec::new();
        for row in 0..256usize {
            for v in acc.iter_mut() {
                *v = 0.0;
            }
            for sub in 0..4usize {
                let yl = row as f64 + (sub as f64 + 0.5) / 4.0;
                xs.clear();
                let mut e = 0usize;
                while e + 3 < edges.len() {
                    let ay = edges[e + 1];
                    let by = edges[e + 3];
                    // half-open in y, so a vertex on the line is counted once
                    if (ay <= yl && by > yl) || (by <= yl && ay > yl) {
                        let ax = edges[e];
                        let bx = edges[e + 2];
                        xs.push(ax + (yl - ay) / (by - ay) * (bx - ax));
                    }
                    e += 4;
                }
                if xs.len() < 2 {
                    continue;
                }
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mut p = 0usize;
                while p + 1 < xs.len() {
                    baked_span(&mut acc, xs[p], xs[p + 1]);
                    p += 2;
                }
            }
            for c in 0..256usize {
                let v = acc[c] * 0.25 * 255.0;
                mask[row * 256 + c] = if v <= 0.0 {
                    0
                } else if v >= 255.0 {
                    255
                } else {
                    v as u8
                };
            }
        }
        mask
    }

    // one span of one sub-scanline, added to the row accumulator as the exact
    // fraction of each pixel it covers
    fn baked_span(acc: &mut Vec<f32>, x0: f64, x1: f64) {
        let mut a = x0;
        let mut b = x1;
        if a < 0.0 { a = 0.0; }
        if b > 256.0 { b = 256.0; }
        if b <= a {
            return;
        }
        let first = a.floor() as usize;
        let last = (b - 1.0e-9).floor() as usize;
        if first >= 256 {
            return;
        }
        if first == last {
            acc[first] += (b - a) as f32;
            return;
        }
        acc[first] += ((first as f64 + 1.0) - a) as f32;
        let mut c = first + 1;
        while c < last && c < 256 {
            acc[c] += 1.0;
            c += 1;
        }
        if last < 256 {
            acc[last] += (b - last as f64) as f32;
        }
    }

    // ---- the pictures ------------------------------------------------------

    // any colour type in, 256×256 RGB8 out, or empty. EXPAND turns a palette
    // or a transparency chunk into real channels and STRIP_16 flattens a deep
    // image, so the match below only has to know the eight-bit shapes.
    fn baked_rgb(bytes: &Vec<u8>) -> Vec<u8> {
        let mut dec = png::Decoder::new(&bytes[..]);
        dec.set_transformations(png::Transformations::EXPAND
                                | png::Transformations::STRIP_16);
        let mut reader = match dec.read_info() {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let mut buf = vec![0u8; reader.output_buffer_size()];
        let info = match reader.next_frame(&mut buf) {
            Ok(i) => i,
            Err(_) => return Vec::new(),
        };
        if info.width != 256 || info.height != 256 {
            return Vec::new();
        }
        let n = 256usize * 256usize;
        let mut out = vec![0u8; n * 3];
        match info.color_type {
            png::ColorType::Rgb => {
                out.copy_from_slice(&buf[..n * 3]);
            }
            png::ColorType::Rgba => {
                for i in 0..n {
                    out[i * 3] = buf[i * 4];
                    out[i * 3 + 1] = buf[i * 4 + 1];
                    out[i * 3 + 2] = buf[i * 4 + 2];
                }
            }
            png::ColorType::Grayscale => {
                for i in 0..n {
                    out[i * 3] = buf[i];
                    out[i * 3 + 1] = buf[i];
                    out[i * 3 + 2] = buf[i];
                }
            }
            png::ColorType::GrayscaleAlpha => {
                for i in 0..n {
                    out[i * 3] = buf[i * 2];
                    out[i * 3 + 1] = buf[i * 2];
                    out[i * 3 + 2] = buf[i * 2];
                }
            }
            _ => return Vec::new(),
        }
        out
    }

    fn baked_over(ground: &Vec<u8>, over: &Vec<u8>, mask: &Vec<u8>) -> Vec<u8> {
        let n = 256usize * 256usize;
        let mut out = vec![0u8; n * 3];
        for i in 0..n {
            let a = mask[i] as u32;
            let b = 255 - a;
            for c in 0..3usize {
                let g = ground[i * 3 + c] as u32;
                let o = over[i * 3 + c] as u32;
                out[i * 3 + c] = (((g * b + o * a) + 127) / 255) as u8;
            }
        }
        out
    }

    // Fast and not Best: the bake runs on the request's own thread and a
    // canvasser is waiting for the square. Fast costs a few kilobytes.
    fn baked_png(rgb: &Vec<u8>) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, 256, 256);
            enc.set_color(png::ColorType::Rgb);
            enc.set_depth(png::BitDepth::Eight);
            enc.set_compression(png::Compression::Fast);
            let mut w = match enc.write_header() {
                Ok(w) => w,
                Err(_) => return Vec::new(),
            };
            if w.write_image_data(&rgb[..]).is_err() {
                return Vec::new();
            }
        }
        out
    }

    // ---- the two source grounds --------------------------------------------
    // The everyday ground is read from /tiles' OWN cache path and written back
    // into it. /region calls nothing of /tiles' so that a browse-tree node does
    // not tie its tick to a serve-tree one, and this keeps that — the path is
    // read, not the code. Sharing the file rather than keeping a private copy
    // is the point: the everyday ground is Stadia too, and a second cache
    // would mean fetching every square of Sevenoaks twice from a metered
    // account. Same naming, same bytes, same PNG check, so either writer makes
    // a file the other reads.

    fn baked_ground_url() -> String {
        match std::env::var("MISO_TILE_URL") {
            Ok(u) => u,
            Err(_) => String::new(),
        }
    }

    fn baked_ground(z: u32, x: u32, y: u32) -> Vec<u8> {
        let dir = format!("{}/tiles/{}/{}", baked_context(), z, x);
        let file = format!("{}/{}.png", dir, y);
        match std::fs::read(file.clone()) {
            Ok(b) => {
                if !b.is_empty() {
                    return b;
                }
            }
            Err(_) => {}
        }
        let src = baked_ground_url();
        if src.is_empty() {
            return Vec::new();
        }
        let bytes = baked_fetch(src, z, x, y);
        if !region_is_png(&bytes) {
            return Vec::new();
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, bytes.clone());
        bytes
    }

    // the Outdoors square through /region's own cache and fetch — this node's
    // parent, and therefore always composed with it
    fn baked_outdoors(z: u32, x: u32, y: u32) -> Vec<u8> {
        let dir = format!("{}/{}/{}", region_dir(), z, x);
        let file = format!("{}/{}.png", dir, y);
        match std::fs::read(file.clone()) {
            Ok(b) => {
                if !b.is_empty() {
                    return b;
                }
            }
            Err(_) => {}
        }
        let bytes = region_fetch(z, x, y);
        if !region_is_png(&bytes) {
            return Vec::new();
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, bytes.clone());
        bytes
    }

    fn baked_fetch(src: String, z: u32, x: u32, y: u32) -> Vec<u8> {
        let url = src
            .replace("{z}", &z.to_string())
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string());
        let out = std::process::Command::new("curl")
            .arg("-s")
            .arg("-f")
            .arg("-L")
            .arg("--connect-timeout").arg("4")
            .arg("--max-time").arg("10")
            .arg("-A").arg(region_agent())
            .arg(url)
            .output();
        match out {
            Ok(o) => {
                if o.status.success() {
                    o.stdout
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        }
    }

    // ---- the flow ----------------------------------------------------------

    fn baked_serve(path: String) -> response {
        let code = baked_code(path.clone());
        let c = baked_coords(path);
        if code.is_empty() || c.len() != 3 {
            return text_response(404, "not found");
        }
        let z = c[0];
        let x = c[1];
        let y = c[2];

        // the bake cache first: a straddling square is written once and read
        // for ever after.
        //
        // Beside it, two zero-byte markers. A square wholly inside or wholly
        // outside the polygon is served from the two caches that already hold
        // it and no picture is written — the brief's rule, and what keeps a
        // third copy of every square in Kent off the disk. But *deciding* that
        // means rasterising the mask, and the rig measured a wholly-inside
        // square at 21 ms against 8 ms for a baked one: the interior is most
        // of a region, so the common square was the slow one. The marker
        // remembers the decision without remembering the picture.
        let dir = format!("{}/{}/{}/{}/{}", baked_dir(), baked_stamp(),
                          code.to_ascii_uppercase(), z, x);
        let file = format!("{}/{}.png", dir, y);
        match std::fs::read(file.clone()) {
            Ok(b) => {
                if !b.is_empty() {
                    println!("miso: baked {} {}/{}/{} disk", code, z, x, y);
                    return baked_response(b);
                }
            }
            Err(_) => {}
        }
        let mark_g = format!("{}/{}.g", dir, y);
        let mark_o = format!("{}/{}.o", dir, y);
        if std::path::Path::new(&mark_g).exists() {
            return baked_plain(baked_ground(z, x, y), code, z, x, y, "clear (marked)");
        }
        if std::path::Path::new(&mark_o).exists() {
            return baked_plain(baked_outdoors(z, x, y), code, z, x, y, "full (marked)");
        }

        let rings = baked_rings(baked_geo(code.clone()));
        if rings.is_empty() {
            return text_response(404, "not found");
        }
        // the polygon's own box against the tile's: the cheap way past every
        // square the region cannot reach, before anything is rasterised
        let bb = baked_box(&rings);
        if bb.len() != 4 {
            return text_response(404, "not found");
        }
        let world = 256.0 * ((1u64 << z) as f64);
        let ox = (x as f64) * 256.0;
        let oy = (y as f64) * 256.0;
        let bx0 = baked_px(bb[0], world) - ox;
        let bx1 = baked_px(bb[2], world) - ox;
        let by0 = baked_py(bb[3], world) - oy;   // north is the smaller y
        let by1 = baked_py(bb[1], world) - oy;
        if bx1 < 0.0 || bx0 > 256.0 || by1 < 0.0 || by0 > 256.0 {
            baked_mark(dir.clone(), mark_g.clone());
            return baked_plain(baked_ground(z, x, y), code, z, x, y, "outside");
        }

        let mask = baked_mask(&rings, z, x, y);
        let mut lo = false;
        let mut hi = false;
        for m in mask.iter() {
            if *m == 0 { lo = true; } else if *m == 255 { hi = true; } else { lo = true; hi = true; break; }
            if lo && hi { break; }
        }
        // wholly one or the other: the cached square as it stands, and nothing
        // written — the brief's rule, and what keeps a third copy of every
        // square in Kent off the disk
        if !hi {
            baked_mark(dir.clone(), mark_g);
            return baked_plain(baked_ground(z, x, y), code, z, x, y, "clear");
        }
        if !lo {
            baked_mark(dir.clone(), mark_o);
            return baked_plain(baked_outdoors(z, x, y), code, z, x, y, "full");
        }

        let g = baked_rgb(&baked_ground(z, x, y));
        if g.is_empty() {
            println!("miso: baked {} {}/{}/{} no ground", code, z, x, y);
            return text_response(404, "not found");
        }
        let o = baked_rgb(&baked_outdoors(z, x, y));
        if o.is_empty() {
            // the Outdoors source is unreachable: there is nothing to draw
            // over, so the ground square IS the baked square — and it matches
            // the ground layer beneath it exactly. Not written: a failure must
            // not become a week-long ghost.
            return baked_plain(baked_ground(z, x, y), code, z, x, y, "no outdoors");
        }
        let png = baked_png(&baked_over(&g, &o, &mask));
        if png.is_empty() {
            return text_response(404, "not found");
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, png.clone());
        println!("miso: baked {} {}/{}/{} composited {} bytes", code, z, x, y,
                 png.len());
        baked_response(png)
    }

    // a zero-byte file whose NAME is the whole of what it remembers: this
    // square is all ground, or all Outdoors. Failing to write one costs a
    // rasterisation next time and nothing else, so nothing here is checked.
    fn baked_mark(dir: String, path: String) {
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(path, Vec::new());
    }

    fn baked_plain(bytes: Vec<u8>, code: String, z: u32, x: u32, y: u32,
                   why: &'static str) -> response {
        if bytes.is_empty() {
            println!("miso: baked {} {}/{}/{} {} but missing", code, z, x, y, why);
            return text_response(404, "not found");
        }
        println!("miso: baked {} {}/{}/{} {}", code, z, x, y, why);
        baked_response(bytes)
    }

    fn baked_response(bytes: Vec<u8>) -> response {
        response { status: 200, ctype: "image/png".to_string(), body: bytes,
                   set_cookie: String::new(),
                   cache: "public, max-age=604800".to_string() }
    }
}

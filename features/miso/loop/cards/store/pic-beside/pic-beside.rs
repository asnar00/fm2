struct feature_PicBeside;
impl feature_PicBeside {
    // ---- the store -------------------------------------------------------
    // one file per picture, addressed by id alone and never rewritten.
    // Deliberately NOT under a per-user directory the way /mirror's clips are:
    // a clip lives in its owner's dir and that is exactly why a copy's clip
    // will not play in the recipient's world. A picture must show, so
    // authority is decided per request (pic_may_read) instead of by the path.

    fn pic_dir() -> String {
        format!("{}/.miso-blobs/pics",
                std::env::var("HOME").unwrap_or(".".to_string()))
    }

    fn pic_file(id: String) -> String {
        format!("{}/{}", pic_dir(), id)
    }

    // an id is 24 lowercase hex characters and nothing else, so no id can name
    // a file outside the store and "retrofit" can never be read as one.
    fn pic_id_ok(id: &String) -> bool {
        id.len() == 24 && id.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    }

    fn pic_ref(id: String) -> String {
        format!("pic/{}", id)
    }

    // one picture's ceiling. Well above /cards' 8KB and far below a phone
    // photograph, because what arrives here has already been through shrink.
    fn pic_max() -> usize {
        1048576
    }

    // ---- the road --------------------------------------------------------
    // this link is the OUTERMOST on the route chain, being the newest node's,
    // which puts it outside /edit's turn boundary. That is what makes
    // pic_holds able to name another world and read its live value —
    // /exchange documents the same reasoning for the same reason. Everything
    // that is not ours goes straight to existing, so /msg and every other
    // route are untouched.

    fn route(r: request) -> response {
        if r.path == "pic/retrofit" {
            return pic_retrofit_route(r);
        }
        let id = match r.path.strip_prefix("pic/") {
            Some(rest) => rest.to_string(),
            None => return existing.route(r),
        };
        if !msg_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        if !pic_id_ok(&id) {
            return json_response(400, "{\"ok\":false,\"error\":\"bad id\"}".to_string());
        }
        if r.method == "POST" {
            return pic_take(r, id);
        }
        pic_serve(r, id)
    }

    // writing: open to any logged-in caller and write-once. To overwrite
    // somebody's picture you would have to guess an id already in use, and to
    // guess it you would already have to hold the card that names it. An id
    // that is already stored answers ok without writing, which is what makes
    // the device's retry safe.
    fn pic_take(r: request, id: String) -> response {
        let file = pic_file(id.clone());
        if std::path::Path::new(&file).exists() {
            return json_response(200, "{\"ok\":true,\"stored\":false}".to_string());
        }
        if r.raw.len() > pic_max() || r.raw.is_empty() {
            return json_response(413, "{\"ok\":false,\"error\":\"too big\"}".to_string());
        }
        let _ = std::fs::create_dir_all(pic_dir());
        match std::fs::write(file, &r.raw) {
            Ok(_) => json_response(200, "{\"ok\":true,\"stored\":true}".to_string()),
            Err(e) => {
                println!("pic-beside: could not store {}: {}", pic_ref(id), e);
                json_response(500, "{\"ok\":false}".to_string())
            }
        }
    }

    // reading: THE authority rule. You may see a picture if a card in your own
    // world names it. Nothing else — not who uploaded it, not who you are
    // invite-linked to. That rule is exactly the visibility /exchange already
    // grants, so it follows /co-members and every later way of handing a card
    // over with no code here, and a /delete tombstone that drops the reference
    // takes the access away with it.
    fn pic_serve(r: request, id: String) -> response {
        if !pic_may_read(r.cookie.clone(), id.clone()) {
            return json_response(403, "{\"ok\":false,\"error\":\"not yours to see\"}".to_string());
        }
        match std::fs::read(pic_file(id)) {
            Ok(bytes) => response { status: 200,
                                    ctype: pic_kind_of(&bytes),
                                    body: bytes,
                                    set_cookie: String::new(),
                                    // the bytes at an id never change, so this
                                    // is honest. private, because who may read
                                    // it is decided per caller above.
                                    cache: "private, max-age=31536000, immutable".to_string() },
            Err(_) => json_response(404, "{\"ok\":false}".to_string()),
        }
    }

    fn pic_may_read(cookie: String, id: String) -> bool {
        let who = sender_of(cookie);
        if who.is_empty() {
            return false;
        }
        pic_holds(who, id)
    }

    // one world's cards, read live, and asked one question. Safe only from
    // outside a turn, which is where the only caller is.
    fn pic_holds(key: String, id: String) -> bool {
        let saved = context_user_now();
        context_user_set(key);
        let list = cards_read();
        context_user_set(saved);
        list.contains(&pic_ref(id))
    }

    // ---- the retrofit ----------------------------------------------------
    // /retrofit is doctrine: a change to what a card holds names its backfill
    // before it is built, and the backfill is revertible. `out` moves inline
    // pictures into the store; `back` puts them inline again. Screened exactly
    // as POST diag/context is — open on localhost for tooling, cookie-gated
    // through the tunnel.

    fn pic_retrofit_route(r: request) -> response {
        if r.method != "POST" {
            return json_response(405, "{\"ok\":false}".to_string());
        }
        if r.tunnel && !authed(r.cookie.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let body: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::json!({}));
        let mode = body["mode"].as_str().unwrap_or("out").to_string();
        if mode != "out" && mode != "back" {
            return json_response(400,
                "{\"ok\":false,\"error\":\"mode is out or back\"}".to_string());
        }
        let dry = body["dry"].as_bool().unwrap_or(false);
        let one = body["world"].as_str().unwrap_or("").to_string();
        let worlds = if one.is_empty() { pic_worlds() } else { vec![one] };
        let mut report: Vec<serde_json::Value> = Vec::new();
        for w in worlds {
            report.push(pic_retrofit_world(w, mode.clone(), dry));
        }
        json_response(200, serde_json::json!({
            "ok": true, "mode": mode, "dry": dry, "worlds": report }).to_string())
    }

    // where the op logs are. Spelled out here rather than borrowed from
    // /remember's context_dir(), so the retrofit does not make this node fail
    // to link without that one — the same two lines, and they must stay in
    // step if the op log ever moves.
    fn pic_log_dir() -> String {
        match std::env::var("MISO_CONTEXT_DIR") {
            Ok(d) if !d.is_empty() => d,
            _ => format!("{}/.miso-context",
                         std::env::var("HOME").unwrap_or_default()),
        }
    }

    // the op-log directory names every world the server has ever written, with
    // each key percent-encoded by context_log_file. Read the names back.
    fn pic_worlds() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let entries = match std::fs::read_dir(pic_log_dir()) {
            Ok(e) => e,
            Err(_) => return out,
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let stem = match name.strip_suffix(".log") {
                Some(s) => s.to_string(),
                None => continue,
            };
            if stem.starts_with('_') {
                continue;               // _global is nobody's cards
            }
            out.push(pic_unescape(stem));
        }
        out.sort();
        out
    }

    // the inverse of context_log_file's escaping, and nothing more: %XX back
    // to a byte, everything else through.
    fn pic_unescape(name: String) -> String {
        let bytes = name.as_bytes();
        let mut out: Vec<u8> = Vec::new();
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                let hex = String::from_utf8_lossy(&bytes[i + 1..i + 3]).to_string();
                match u8::from_str_radix(&hex, 16) {
                    Ok(b) => { out.push(b); i += 3; continue; }
                    Err(_) => {}
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8_lossy(&out).to_string()
    }

    fn pic_retrofit_world(key: String, mode: String, dry: bool) -> serde_json::Value {
        let saved = context_user_now();
        context_user_set(key.clone());
        let before = cards_read();
        context_user_set(saved);
        let mut list: serde_json::Value = serde_json::from_str(&before)
            .unwrap_or(serde_json::Value::Null);
        if !list.is_array() {
            return serde_json::json!({ "world": key, "moved": 0, "note": "no cards" });
        }
        let mut moved = 0u64;
        let mut failed = 0u64;
        let n = list.as_array().map(|a| a.len()).unwrap_or(0);
        for i in 0..n {
            let blocks = list[i]["blocks"].as_array().map(|a| a.len()).unwrap_or(0);
            for b in 0..blocks {
                if list[i]["blocks"][b]["kind"].as_str().unwrap_or("") != "picture" {
                    continue;
                }
                let data = list[i]["blocks"][b]["data"].as_str().unwrap_or("").to_string();
                let next = if mode == "out" { pic_out_block(data.clone()) }
                           else { pic_back_block(data.clone()) };
                if next.is_empty() || next == data {
                    if mode == "back" && data.starts_with("pic/") {
                        failed += 1;    // the bytes are gone; the reference stays
                    }
                    continue;
                }
                list[i]["blocks"][b]["data"] = serde_json::json!(next);
                moved += 1;
            }
        }
        let after = list.to_string();
        if moved > 0 && !dry {
            pic_write_list(key.clone(), after.clone());
        }
        serde_json::json!({ "world": key, "moved": moved, "unreadable": failed,
                            "was": before.len(), "now": after.len() })
    }

    // inline bytes out: the id is the content hash, truncated, so running the
    // retrofit twice is a no-op and the same photograph in two worlds
    // converges on one file. An empty answer means "leave this block alone".
    fn pic_out_block(data: String) -> String {
        if !data.starts_with("data:") {
            return String::new();
        }
        let b64 = match data.find(";base64,") {
            Some(i) => data[i + 8..].to_string(),
            None => return String::new(),
        };
        use base64::Engine;
        let bytes = match base64::engine::general_purpose::STANDARD.decode(b64) {
            Ok(v) => v,
            Err(_) => return String::new(),
        };
        if bytes.is_empty() || bytes.len() > pic_max() {
            return String::new();
        }
        let id = pic_hash(&bytes);
        let file = pic_file(id.clone());
        if !std::path::Path::new(&file).exists() {
            let _ = std::fs::create_dir_all(pic_dir());
            if std::fs::write(file, &bytes).is_err() {
                return String::new();
            }
        }
        pic_ref(id)
    }

    // and back again: the bytes read out of the store and spelled as a data
    // URL. A reference whose bytes are gone is left exactly as it is — a
    // revert that cannot restore the picture must not destroy the address of
    // it.
    fn pic_back_block(data: String) -> String {
        let id = match data.strip_prefix("pic/") {
            Some(s) => s.to_string(),
            None => return String::new(),
        };
        if !pic_id_ok(&id) {
            return String::new();
        }
        let bytes = match std::fs::read(pic_file(id)) {
            Ok(b) => b,
            Err(_) => return String::new(),
        };
        use base64::Engine;
        format!("data:{};base64,{}", pic_kind_of(&bytes),
                base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    // what the first bytes say the picture is. Stored nowhere, because the
    // file itself is the honest record of its own type.
    fn pic_kind_of(bytes: &Vec<u8>) -> String {
        if bytes.len() > 3 && bytes[0] == 0x89 && bytes[1] == 0x50 {
            return "image/png".to_string();
        }
        if bytes.len() > 3 && bytes[0] == 0x47 && bytes[1] == 0x49 {
            return "image/gif".to_string();
        }
        if bytes.len() > 12 && bytes[8] == 0x57 && bytes[9] == 0x45 {
            return "image/webp".to_string();
        }
        "image/jpeg".to_string()
    }

    fn pic_hash(bytes: &Vec<u8>) -> String {
        use sha2::Digest;
        let mut h = sha2::Sha256::new();
        h.update(bytes);
        let out = h.finalize();
        let mut s = String::new();
        for b in out.iter().take(12) {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }

    // the op door, the one /exchange writes a copy through: a CtxOp on the
    // cards var handed to handle_msg with this thread acting as the world.
    // /guard merges it — every card is present with its `edited` untouched, so
    // the merge resolves each tie to the incoming list and the move lands —
    // /converge relays it to open pages and /remember logs it, which is what
    // leaves the op log able to restore every prior value.
    fn pic_write_list(key: String, list: String) {
        let msg = serde_json::json!({
            "type": "CtxOp",
            "_from": "",
            "data": {
                "path": "miso/loop/cards",
                "name": "cards",
                "op": "set",
                "value": list
            }
        }).to_string();
        let saved = context_user_now();
        context_user_set(key.clone());
        let reply = handle_msg(msg);
        context_user_set(saved);
        if !reply.contains("CtxUpdate") {
            println!("pic-beside: {} refused the retrofit: {}", key, reply);
        }
    }
}

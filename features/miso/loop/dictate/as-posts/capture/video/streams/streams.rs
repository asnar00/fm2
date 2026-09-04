struct feature_Streams;
impl feature_Streams {
    // ---- where the pieces wait ---------------------------------------------
    // beside the clips, in a directory named for the recording, so a clip that
    // never completes is one directory to find and one to remove. The blob
    // root is spelled out rather than borrowed for the reason /pic-beside
    // gives for the same two lines.

    fn streams_root() -> String {
        format!("{}/.miso-blobs", std::env::var("HOME").unwrap_or(".".to_string()))
    }

    fn streams_parts_dir(world: String, id: String) -> String {
        format!("{}/{}/parts/{}", streams_root(), world, id)
    }

    // one piece's ceiling and the most a recording may be cut into. At the
    // recorder's bitrate two seconds is well under a megabyte and a minute is
    // thirty pieces, so both of these are the walls of the room and not the
    // furniture: they exist so a wedged recorder cannot fill the disk.
    fn streams_part_max() -> usize {
        8388608
    }

    fn streams_parts_max() -> u64 {
        200
    }

    // ---- the route ---------------------------------------------------------
    // POST blob/<id>/part/<n>, beside /mirror's POST blob/<id>. This link is
    // outermost, so the longer path is claimed before /mirror sees it and
    // reads it as an id with slashes in it.

    fn route(r: request) -> response {
        let rest = match r.path.strip_prefix("blob/") {
            Some(rest) => rest.to_string(),
            None => return existing.route(r),
        };
        let cut = match rest.find("/part/") {
            Some(at) => at,
            None => return existing.route(r),
        };
        if r.method != "POST" {
            return json_response(405, "{\"ok\":false}".to_string());
        }
        if !msg_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let id = rest[..cut].to_string();
        let nth = rest[cut + 6..].to_string();
        if !streams_id_ok(&id) || !streams_n_ok(&nth) {
            return json_response(400, "{\"ok\":false,\"error\":\"bad part\"}".to_string());
        }
        if r.raw.is_empty() || r.raw.len() > streams_part_max() {
            return json_response(413, "{\"ok\":false,\"error\":\"bad size\"}".to_string());
        }
        let world = blob_user(r.cookie.clone(), r.tunnel);
        let dir = streams_parts_dir(world.clone(), id.clone());
        let _ = std::fs::create_dir_all(dir.clone());
        if std::fs::write(format!("{}/{}", dir, nth), &r.raw).is_err() {
            return json_response(500, "{\"ok\":false}".to_string());
        }
        // a piece arriving late — the phone was offline when it was made — may
        // be the one that completes a clip whose RecShared came and went. The
        // count is on the index by then, so the join is tried here too and the
        // clip is queued the moment it is whole.
        let want = streams_wanted(world.clone(), id.clone());
        if want > 0 && streams_join(world.clone(), id.clone(), want) {
            transcribed_queue(world.clone(), id.clone());
            std::thread::spawn(move || {
                transcribed_drain();
            });
        }
        json_response(200, "{\"ok\":true}".to_string())
    }

    fn streams_id_ok(id: &String) -> bool {
        !id.is_empty() && id.len() < 80
            && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_')
    }

    fn streams_n_ok(n: &String) -> bool {
        !n.is_empty() && n.len() < 4 && n.chars().all(|c| c.is_ascii_digit())
            && n.parse::<u64>().unwrap_or(streams_parts_max()) < streams_parts_max()
    }

    // ---- joining -----------------------------------------------------------
    // MediaRecorder's pieces are one file cut at byte boundaries, so the join
    // is a concatenation in order and nothing else. It happens once: the parts
    // directory goes when the clip is written, and a clip that already exists
    // is never rebuilt.

    fn streams_join(world: String, id: String, want: u64) -> bool {
        let file = format!("{}/{}/{}", streams_root(), world, id);
        if std::path::Path::new(&file).exists() {
            let _ = std::fs::remove_dir_all(streams_parts_dir(world, id));
            return false;                 // already whole; nothing new happened
        }
        let dir = streams_parts_dir(world.clone(), id.clone());
        let mut all: Vec<u8> = Vec::new();
        let mut n: u64 = 0;
        while n < want {
            match std::fs::read(format!("{}/{}", dir, n)) {
                Ok(mut bytes) => all.append(&mut bytes),
                Err(_) => return false,   // a piece is still on its way
            }
            n = n + 1;
        }
        let _ = std::fs::create_dir_all(format!("{}/{}", streams_root(), world));
        if std::fs::write(&file, &all).is_err() {
            println!("streams: could not join {} ({} pieces)", id, want);
            return false;
        }
        let _ = std::fs::remove_dir_all(dir);
        println!("streams: {} joined from {} pieces, {} bytes", id, want, all.len());
        true
    }

    // how many pieces this recording was cut into, off /mirror's index — which
    // is where RecShared's metadata lands. Zero means "not streamed": either
    // the announcement has not arrived yet, or the phone fell back to sending
    // the whole clip, and in both cases there is nothing here to join.
    fn streams_wanted(world: String, id: String) -> u64 {
        let raw = std::fs::read_to_string(format!("{}/{}/index.json", streams_root(), world))
            .unwrap_or_default();
        let index: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for e in index.as_array().unwrap_or(&empty) {
            if e["id"].as_str().unwrap_or("") == id {
                return e["parts"].as_u64().unwrap_or(0);
            }
        }
        0
    }

    // ---- the announcement --------------------------------------------------
    // RecShared says the recording is finished and how many pieces it is. The
    // join must happen BEFORE the rest of the chain runs, because further in
    // is where the clip is queued for transcription and a queued clip with no
    // file is a job that gets dropped. The count is read off the message
    // rather than the index for the same reason: the index is written further
    // in too.

    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "RecShared" {
            return existing.handle_msg(msg);
        }
        let from = m["_from"].as_str().unwrap_or("").to_string();
        let world = if from.is_empty() { "_local".to_string() } else { from };
        let id = m["data"]["id"].as_str().unwrap_or("").to_string();
        let want = m["data"]["parts"].as_u64().unwrap_or(0);
        if streams_id_ok(&id) && want > 0 && want <= streams_parts_max() {
            streams_join(world, id, want);
        }
        existing.handle_msg(msg)
    }
}

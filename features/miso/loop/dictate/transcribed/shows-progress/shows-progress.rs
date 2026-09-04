struct feature_ShowsProgress;
impl feature_ShowsProgress {
    // ---- what the phone is told, and when ----------------------------------
    // the hint /as-posts drew was wired to `dict_transcribe`, which /dictate's
    // scheduler only ever sets when a rung's PAGE slot answers ready — and no
    // rung ever redefined one, because every rung we have is the server's. So
    // the hint has never drawn since the server took transcription over. The
    // state it needs is the server's queue, so the server sends it.
    //
    // Published ON CHANGE only. The broadcast slot holds fifty entries and
    // every waiting phone re-parses it five times a second, so a message every
    // ten seconds per world would be the most expensive thing on the box. The
    // three moments a world's set can change are the three links below.

    // BESIDE the queue, never inside it: every `*.json` in the queue directory
    // is a job to the scanner, and a note-to-self left in there is read as a
    // clip with no timestamp and parked as a day old. Found on the rig the
    // first time this ran — `transcribed: told parked (older than a day)`.
    fn shows_progress_seen_file(world: String) -> String {
        format!("{}/told.json", transcribed_world_dir(world))
    }

    // the ids this world is waiting on, and which of them are unhappy. `stuck`
    // is a job that has failed enough times to be worth a different word on the
    // screen — the engineer sheet has the detail; the post says "still trying".
    fn shows_progress_stuck_after() -> u64 {
        3
    }

    fn shows_progress_set(world: String) -> serde_json::Value {
        let mut working: Vec<serde_json::Value> = Vec::new();
        let mut stuck: Vec<serde_json::Value> = Vec::new();
        let entries = match std::fs::read_dir(transcribed_queue_dir(world.clone())) {
            Ok(e) => e,
            Err(_) => return serde_json::json!({ "working": working, "stuck": stuck }),
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let id = match name.strip_suffix(".json") {
                Some(s) => s.to_string(),
                None => continue,
            };
            let raw = std::fs::read_to_string(e.path()).unwrap_or_default();
            let j: serde_json::Value = serde_json::from_str(&raw)
                .unwrap_or(serde_json::json!({}));
            if j["tries"].as_u64().unwrap_or(0) >= shows_progress_stuck_after() {
                stuck.push(serde_json::json!(id));
            } else {
                working.push(serde_json::json!(id));
            }
        }
        // a parked job is still the post's business: it says "still trying"
        // rather than going quiet, because nothing has been given up on.
        let parked = format!("{}/parked", transcribed_queue_dir(world));
        if let Ok(es) = std::fs::read_dir(parked) {
            for e in es.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if let Some(id) = name.strip_suffix(".json") {
                    stuck.push(serde_json::json!(id.to_string()));
                }
            }
        }
        serde_json::json!({ "working": working, "stuck": stuck })
    }

    fn shows_progress_tell(world: String) {
        if world.is_empty() || world == "_local" {
            return;
        }
        let now = shows_progress_set(world.clone());
        let was = std::fs::read_to_string(shows_progress_seen_file(world.clone()))
            .unwrap_or_default();
        let text = now.to_string();
        if was == text {
            return;                 // nothing moved; the slot stays quiet
        }
        let _ = std::fs::create_dir_all(transcribed_queue_dir(world.clone()));
        let _ = std::fs::write(shows_progress_seen_file(world.clone()), text);
        publish(format!("user.{}", world), serde_json::json!({
            "type": "Transcribing", "data": now }).to_string());
    }

    // the three moments: a clip joins the queue, a clip leaves it landed, and
    // a clip is rescheduled (which is how `working` becomes `stuck`).

    fn transcribed_queue(world: String, id: String) {
        existing.transcribed_queue(world.clone(), id);
        shows_progress_tell(world);
    }

    fn transcribed_finish(world: String, id: String, rung: String, grade: i64) {
        existing.transcribed_finish(world.clone(), id, rung, grade);
        shows_progress_tell(world);
    }

    fn transcribed_retry(world: String, id: String, tries: u64, why: String) -> bool {
        let done = existing.transcribed_retry(world.clone(), id, tries, why);
        shows_progress_tell(world);
        done
    }

    // ---- the world's side --------------------------------------------------

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "Transcribing" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["dict_working"] = e["data"].clone();
        s.to_string()
    }

    // ---- the mark ----------------------------------------------------------
    // two surfaces, both by naming an attribute the stylesheet knows: the play
    // row on an open post (which already carries `data-rec`), and the post's
    // tile in the grid (which carries `data-card`). /as-posts marks the drawn
    // page from `render` for the same reason — only one card page is ever open
    // and the state is only here.

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let working = s["dict_working"]["working"].as_array().unwrap_or(&empty).clone();
        let stuck = s["dict_working"]["stuck"].as_array().unwrap_or(&empty).clone();
        if working.is_empty() && stuck.is_empty() {
            return base;
        }
        let cards: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let mut out = base;
        for v in working.iter() {
            let id = v.as_str().unwrap_or("").to_string();
            if !id.is_empty() {
                out = shows_progress_mark(out, id, "on".to_string(), &cards);
            }
        }
        for v in stuck.iter() {
            let id = v.as_str().unwrap_or("").to_string();
            if !id.is_empty() {
                out = shows_progress_mark(out, id, "stuck".to_string(), &cards);
            }
        }
        out
    }

    fn shows_progress_mark(html: String, rec: String, kind: String,
                           cards: &serde_json::Value) -> String {
        // the open post's play row
        let mut out = html.replace(&format!("data-rec=\"{}\"", rec),
                                   &format!("data-rec=\"{}\" data-work=\"{}\"", rec, kind));
        // and its tile, found by the card that owns the recording
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in cards.as_array().unwrap_or(&empty) {
            if c["rec"].as_str().unwrap_or("") != rec {
                continue;
            }
            let cid = c["id"].as_str().unwrap_or("").to_string();
            if cid.is_empty() {
                continue;
            }
            out = out.replace(&format!("<div class=\"card-tile\" data-card=\"{}\"", cid),
                              &format!("<div class=\"card-tile\" data-work=\"{}\" data-card=\"{}\"",
                                       kind, cid));
        }
        out
    }
}

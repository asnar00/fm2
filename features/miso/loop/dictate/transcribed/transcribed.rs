struct feature_Transcribed;
impl feature_Transcribed {
    // ---- the two paths, spelled out ---------------------------------------
    // /mirror's blob store and /remember's op log, written here rather than
    // borrowed, so that unticking either of those cannot stop this node
    // linking. /pic-beside carries the same two lines for the same reason;
    // they must stay in step if a store ever moves.

    fn transcribed_root() -> String {
        format!("{}/.miso-blobs", std::env::var("HOME").unwrap_or(".".to_string()))
    }

    fn transcribed_world_dir(world: String) -> String {
        format!("{}/{}", transcribed_root(), world)
    }

    fn transcribed_queue_dir(world: String) -> String {
        format!("{}/queue", transcribed_world_dir(world))
    }

    fn transcribed_done_dir(world: String) -> String {
        format!("{}/queue/done", transcribed_world_dir(world))
    }

    // ---- the extension points ---------------------------------------------
    // three bases that answer "nothing here". A rung child redefines the
    // first two; /context redefines the third. With none of them ticked this
    // node queues, finds no rung, and leaves every job where it is — which is
    // the same as not being here at all.

    // one clip's work order: {world, id, path, vocab, want}. `want` is the
    // grade being asked for, and a rung answers only when it is its own — the
    // ladder is walked HERE, by transcribed_run, from the best grade down, so
    // each rung is asked at most once per clip whatever order the chain
    // happens to be in (same-anchor siblings load in name order, misses.md
    // 2026-09-03, and neither rung may assume it is outermost). The answer is
    // {text, rung, grade}, or "" for "not mine, or I could not".
    fn transcribe_rung(job: String) -> String {
        let _ = job;
        String::new()
    }

    // the highest grade that is REACHABLE on this box right now — a key
    // present, a worker alive. Zero means nothing can transcribe, so nothing
    // is queued and no phone is told a transcript is coming.
    fn transcribe_best_grade() -> i64 {
        0
    }

    // the phrases a clip is seeded with, from the post it belongs to.
    fn transcribe_vocab(card: String) -> Vec<String> {
        let _ = card;
        let out: Vec<String> = Vec::new();
        out
    }

    // ---- the trigger -------------------------------------------------------
    // no timer anywhere. A clip arriving and a device booting are the two
    // moments when there might be work, and both are messages this server
    // already answers.

    fn handle_msg(msg: String) -> String {
        let reply = existing.handle_msg(msg.clone());
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        let t = m["type"].as_str().unwrap_or("").to_string();
        if t != "RecShared" && t != "RecIndex" {
            return reply;
        }
        let from = m["_from"].as_str().unwrap_or("").to_string();
        let world = if from.is_empty() { "_local".to_string() } else { from.clone() };
        if t == "RecShared" {
            let id = m["data"]["id"].as_str().unwrap_or("").to_string();
            transcribed_queue(world.clone(), id);
        }
        if !from.is_empty() {
            publish(format!("user.{}", from), serde_json::json!({
                "type": "TranscribeRungsAre",
                "data": { "best": transcribe_best_grade() }
            }).to_string());
        }
        std::thread::spawn(move || {
            transcribed_drain();
        });
        reply
    }

    // ---- the queue ---------------------------------------------------------

    // a clip's id may not be a path, and this is the only place a filename is
    // built from one. /mirror's blob_id_ok already refused anything else on
    // the way in; this is the second look, because a filename is worth two.
    fn transcribed_id_ok(id: &String) -> bool {
        !id.is_empty() && id.len() < 80
            && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_')
    }

    // an id already queued keeps its job: rewriting it would reset the try
    // count and a clip nothing can transcribe would be retried for ever.
    // An id already done is not re-queued here — that is `transcribed_upgrade`'s
    // decision and it has a counter of its own.
    fn transcribed_queue(world: String, id: String) {
        if !transcribed_id_ok(&id) {
            return;
        }
        let dir = transcribed_queue_dir(world.clone());
        let job = format!("{}/{}.json", dir, id);
        if std::path::Path::new(&job).exists() {
            return;
        }
        if std::path::Path::new(&format!("{}/{}.json", transcribed_done_dir(world.clone()), id)).exists() {
            return;
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(job, serde_json::json!({
            "id": id, "at": now_ms(), "tries": 0 }).to_string());
    }

    // every world that has a queue directory. The blob root's children are
    // world keys as /mirror wrote them, so no unescaping is needed.
    fn transcribed_worlds() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let entries = match std::fs::read_dir(transcribed_root()) {
            Ok(e) => e,
            Err(_) => return out,
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name == "pics" || name.starts_with('.') {
                continue;
            }
            if std::path::Path::new(&transcribed_queue_dir(name.clone())).is_dir() {
                out.push(name);
            }
        }
        out.sort();
        out
    }

    // the jobs of one world, oldest first. A file that will not parse is a
    // job with no stamp, which sorts first and is dropped on its first run.
    fn transcribed_jobs(world: String) -> Vec<serde_json::Value> {
        let mut out: Vec<serde_json::Value> = Vec::new();
        let entries = match std::fs::read_dir(transcribed_queue_dir(world)) {
            Ok(e) => e,
            Err(_) => return out,
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let id = match name.strip_suffix(".json") {
                Some(s) => s.to_string(),
                None => continue,
            };
            if !transcribed_id_ok(&id) {
                continue;
            }
            let raw = std::fs::read_to_string(e.path()).unwrap_or_default();
            let mut j: serde_json::Value = serde_json::from_str(&raw)
                .unwrap_or(serde_json::json!({}));
            j["id"] = serde_json::json!(id);
            out.push(j);
        }
        out.sort_by_key(|j| j["at"].as_u64().unwrap_or(0));
        out
    }

    // ---- the lock ----------------------------------------------------------
    // one clip at a time on the whole box: the mini holds one warm model and
    // the live server beside it, and two would swap. The lock is a file with
    // a stamp, and a stamp old enough that nothing can still be running it is
    // no lock at all — /reports' answer to the same question.

    fn transcribed_lock_file() -> String {
        format!("{}/queue.lock", transcribed_root())
    }

    fn transcribed_lock_stale_ms() -> u64 {
        1200000
    }

    fn transcribed_take_lock() -> bool {
        let file = transcribed_lock_file();
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let held: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let at = held["at"].as_u64().unwrap_or(0);
        if at > 0 && now_ms() < at + transcribed_lock_stale_ms() {
            return false;
        }
        let _ = std::fs::create_dir_all(transcribed_root());
        let _ = std::fs::write(file, serde_json::json!({
            "at": now_ms(), "pid": std::process::id() }).to_string());
        true
    }

    // the lock is re-stamped while a long job runs, so a clip that takes ten
    // minutes does not look abandoned to the next drain.
    fn transcribed_touch_lock() {
        let _ = std::fs::write(transcribed_lock_file(), serde_json::json!({
            "at": now_ms(), "pid": std::process::id() }).to_string());
    }

    fn transcribed_drop_lock() {
        let _ = std::fs::remove_file(transcribed_lock_file());
    }

    // ---- the drain ---------------------------------------------------------
    // three rounds, twenty seconds apart, then gone. The rounds exist for one
    // failure: a clip reaches the server before its post does (the phone
    // uploads bytes and syncs cards on two different roads), and twenty
    // seconds is long enough for the card to arrive without a thread that
    // lives for ever. Every way out drops the lock.

    fn transcribed_rounds() -> u32 {
        3
    }

    fn transcribed_drain() {
        if transcribe_best_grade() == 0 {
            return;
        }
        if !transcribed_take_lock() {
            return;
        }
        transcribed_upgrade();
        let mut round: u32 = 0;
        while round < transcribed_rounds() {
            let mut left = 0;
            for world in transcribed_worlds() {
                for job in transcribed_jobs(world.clone()) {
                    transcribed_touch_lock();
                    if !transcribed_run(world.clone(), job) {
                        left = left + 1;
                    }
                }
            }
            if left == 0 {
                break;
            }
            round = round + 1;
            if round < transcribed_rounds() {
                std::thread::sleep(std::time::Duration::from_secs(20));
            }
        }
        transcribed_drop_lock();
    }

    // ---- one job -----------------------------------------------------------
    // true means "finished with, one way or another" — landed, or dropped.
    // false means "still waiting", which is what makes the drain go round.

    fn transcribed_max_tries() -> u64 {
        5
    }

    fn transcribed_job_life_ms() -> u64 {
        86400000
    }

    // ---- the two ends of a job that did not land ---------------------------
    // seams, so that WHAT HAPPENS to a job nothing could do today is one
    // decision in one place. The answers here are exactly what this node did
    // before they had names: a few more tries and then the job is gone.
    // /keeps-trying redefines both — nothing is ever dropped, and the ladder
    // is a schedule rather than a count.

    // true means "finished with"; false means "it will be tried again".
    fn transcribed_retry(world: String, id: String, tries: u64, why: String) -> bool {
        if tries >= transcribed_max_tries() {
            println!("transcribed: {} after {} tries; dropping ({})", id, tries, why);
            transcribed_forget(world, id);
            return true;
        }
        transcribed_bump(world, id, tries);
        false
    }

    // a job so old that nothing is coming for it.
    fn transcribed_expire(world: String, id: String) -> bool {
        println!("transcribed: giving up on {} in {} (too old)", id, world);
        transcribed_forget(world, id);
        true
    }

    fn transcribed_run(world: String, job: serde_json::Value) -> bool {
        let id = job["id"].as_str().unwrap_or("").to_string();
        let at = job["at"].as_u64().unwrap_or(0);
        if at == 0 || now_ms() > at + transcribed_job_life_ms() {
            return transcribed_expire(world, id);
        }
        let path = format!("{}/{}", transcribed_world_dir(world.clone()), id);
        if !std::path::Path::new(&path).exists() {
            println!("transcribed: {} has no clip on disk; dropping the job", id);
            transcribed_forget(world, id);
            return true;
        }
        let card = transcribed_card_of(world.clone(), id.clone());
        if card.is_null() {
            let tries = job["tries"].as_u64().unwrap_or(0) + 1;
            return transcribed_retry(world, id, tries, "no post for it yet".to_string());
        }
        // a tombstone is never re-worded: the words left the world when the
        // author deleted the note, and a transcript arriving later must not
        // put them back.
        if card["deleted"].as_u64().unwrap_or(0) > 0 {
            println!("transcribed: {} belongs to a deleted post; dropping", id);
            transcribed_forget(world, id);
            return true;
        }
        let vocab = transcribe_vocab(card.to_string());
        // the ladder, best grade first. A rung that cannot do the work today
        // answers with nothing and the next one down is asked — which is what
        // makes "no key, no network" fall to the mini rather than fail.
        let mut a = serde_json::Value::Null;
        let mut want = transcribe_best_grade();
        while want > 0 {
            let order = serde_json::json!({
                "world": world.clone(), "id": id.clone(), "path": path.clone(),
                "vocab": vocab.clone(), "want": want }).to_string();
            let tried: serde_json::Value = serde_json::from_str(&transcribe_rung(order))
                .unwrap_or(serde_json::Value::Null);
            // silence is an ANSWER and stops the ladder. A clip with nothing
            // said in it must land nothing — and must not be handed down to a
            // rung that would write a model's invented words into the post.
            if !tried["text"].as_str().unwrap_or("").trim().is_empty()
                || tried["silent"].as_bool() == Some(true) {
                a = tried;
                break;
            }
            want = want - 1;
        }
        if a["silent"].as_bool() == Some(true) {
            println!("transcribed: {} is silence; the post keeps its own words", id);
            transcribed_finish(world, id,
                               a["rung"].as_str().unwrap_or("").to_string(),
                               a["grade"].as_i64().unwrap_or(1));
            return true;
        }
        let text = a["text"].as_str().unwrap_or("").trim().to_string();
        if text.is_empty() {
            let tries = job["tries"].as_u64().unwrap_or(0) + 1;
            let why = if transcribe_best_grade() == 0 {
                "no rung reachable".to_string()
            } else {
                "no rung answered".to_string()
            };
            return transcribed_retry(world, id, tries, why);
        }
        let rung = a["rung"].as_str().unwrap_or("server").to_string();
        let grade = a["grade"].as_i64().unwrap_or(1);
        transcribed_land(world.clone(), id.clone(), text, rung.clone(), grade);
        transcribed_finish(world, id, rung, grade);
        true
    }

    fn transcribed_bump(world: String, id: String, tries: u64) {
        let file = format!("{}/{}.json", transcribed_queue_dir(world), id);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let mut j: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        j["tries"] = serde_json::json!(tries);
        let _ = std::fs::write(file, j.to_string());
    }

    fn transcribed_forget(world: String, id: String) {
        let _ = std::fs::remove_file(format!("{}/{}.json", transcribed_queue_dir(world), id));
    }

    // the job leaves the queue and a stamp takes its place, so a second
    // arrival of the same clip is not transcribed twice and the upgrade pass
    // has something to compare against.
    fn transcribed_finish(world: String, id: String, rung: String, grade: i64) {
        transcribed_forget(world.clone(), id.clone());
        let dir = transcribed_done_dir(world);
        let file = format!("{}/{}.json", dir, id);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let was: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, serde_json::json!({
            "rung": rung, "grade": grade, "at": now_ms(),
            "upgrades": was["upgrades"].as_u64().unwrap_or(0) }).to_string());
    }

    // ---- the upgrade in place ----------------------------------------------
    // a better rung came into reach, so the clips a worse one wrote are put
    // back. Twenty per pass, so a box that has just been given a key does not
    // spend an hour on its history before it answers today's clip; and twice
    // per clip, so a rung that ADVERTISES a grade it cannot deliver costs two
    // runs and then stops — which is the loop this would otherwise be.

    fn transcribed_upgrade_cap() -> u64 {
        2
    }

    fn transcribed_upgrade() {
        let best = transcribe_best_grade();
        let mut budget = 20;
        for world in transcribed_worlds() {
            let entries = match std::fs::read_dir(transcribed_done_dir(world.clone())) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for e in entries.flatten() {
                if budget == 0 {
                    return;
                }
                let name = e.file_name().to_string_lossy().to_string();
                let id = match name.strip_suffix(".json") {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                if !transcribed_id_ok(&id) {
                    continue;
                }
                let raw = std::fs::read_to_string(e.path()).unwrap_or_default();
                let d: serde_json::Value = serde_json::from_str(&raw)
                    .unwrap_or(serde_json::json!({}));
                if d["grade"].as_i64().unwrap_or(99) >= best {
                    continue;
                }
                let used = d["upgrades"].as_u64().unwrap_or(0);
                if used >= transcribed_upgrade_cap() {
                    continue;
                }
                let mut next = d.clone();
                next["upgrades"] = serde_json::json!(used + 1);
                let _ = std::fs::write(e.path(), next.to_string());
                let dir = transcribed_queue_dir(world.clone());
                let _ = std::fs::create_dir_all(dir.clone());
                let _ = std::fs::write(format!("{}/{}.json", dir, id), serde_json::json!({
                    "id": id, "at": now_ms(), "tries": 0 }).to_string());
                budget = budget - 1;
            }
        }
    }

    // ---- the landing -------------------------------------------------------

    // the post a clip belongs to, in its owner's world. `rec` is /as-posts'
    // key and survives a delete, which is what lets a tombstone be recognised
    // rather than looked for and missed.
    fn transcribed_card_of(world: String, id: String) -> serde_json::Value {
        let list: serde_json::Value = serde_json::from_str(&exchange_cards_of(world))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if c["rec"].as_str().unwrap_or("") != id {
                continue;
            }
            // a copy is not ours to edit: /exchange's own test
            if !c["from"].is_null() {
                continue;
            }
            return c.clone();
        }
        serde_json::Value::Null
    }

    // /as-posts writes a transcript into a post's words from a `dict_files`
    // entry, and will not write over words a person typed. The same function
    // is called here with a one-field entry, so the server's landing and the
    // phone's are the same landing and doing both is doing one.
    fn transcribed_land(world: String, id: String, text: String, rung: String, grade: i64) {
        let mut card = transcribed_card_of(world.clone(), id.clone());
        if card.is_null() {
            return;
        }
        let file = serde_json::json!({ "id": id.clone(), "transcript": text.clone() });
        if as_posts_land(&mut card, &file) {
            // /exchange hands a changed card on from its ROUTE link, watching
            // the caller's own cards across a POST /msg — and this landing has
            // no request and no cookie, so the words would have stopped in the
            // author's world and the colleague who could see the post would
            // have watched it stay empty for ever. Found on the rig, 2026-09-04.
            // The two reads are the same two the route takes.
            let before = exchange_cards_of(world.clone());
            transcribed_stamp(world.clone(), card);
            let after = exchange_cards_of(world.clone());
            if after != before {
                exchange_share(world.clone(), before, after);
            }
        }
        // addressed with the WORLD KEY, exactly as /mirror addresses RecShared:
        // /whole-number turns `user.phone:+…` into the opaque audience a
        // waiting phone is listening on. The four-digit tag is the CtxOp
        // relay's name for a person and is not this one.
        if world.is_empty() || world == "_local" {
            return;
        }
        publish(format!("user.{}", world), serde_json::json!({
            "type": "Transcribed",
            "data": { "id": id, "text": text, "rung": rung, "grade": grade }
        }).to_string());
    }

    // one card into its own world, through the door /exchange gives a card
    // by: a `set` carrying that card alone, which /guard merges by id,
    // /remember logs and /converge relays to open pages. Copied from
    // /reports' stamp, which needs it for the same reason: work finished on a
    // thread, with no request to answer.
    fn transcribed_stamp(world: String, card: serde_json::Value) {
        let mut one: Vec<serde_json::Value> = Vec::new();
        one.push(card);
        let value = serde_json::Value::Array(one).to_string();
        let msg = serde_json::json!({
            "type": "CtxOp",
            "_from": exchange_audience_of(world.clone()),
            "data": {
                "path": "miso/loop/cards",
                "name": "cards",
                "op": "set",
                "value": value
            }
        }).to_string();
        let saved = context_user_now();
        context_user_set(world);
        let reply = handle_msg(msg);
        context_user_set(saved);
        let rv: serde_json::Value = serde_json::from_str(&reply)
            .unwrap_or(serde_json::Value::Null);
        if rv["type"].as_str().unwrap_or("") != "CtxUpdate" {
            println!("transcribed: a stamp did not land ({})",
                     rv["error"].as_str().unwrap_or("no reason given"));
        }
    }

    // ---- the one client link -----------------------------------------------
    // what the phone is told about the rungs. It changes nothing about where
    // the words come from; it is what lets /dictate's scheduler say
    // "transcribing…" truthfully, and stay quiet when nothing is listening.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "TranscribeRungsAre" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["dict_rungs"] = e["data"].clone();
        s.to_string()
    }
}

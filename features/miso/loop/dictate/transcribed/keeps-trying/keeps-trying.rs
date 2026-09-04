struct feature_KeepsTrying;
impl feature_KeepsTrying {
    // ---- the keeper --------------------------------------------------------
    // the drain used to run only when a message arrived — a recording shared,
    // a phone booting — so a job nothing could do today waited for somebody to
    // open the app. On 2026-09-04 that was ash's 15:30 clip: the rung failed
    // (ffmpeg was not on the server's PATH), the job was re-queued by hand, and
    // it sat there until his phone next spoke. A thread of our own is the whole
    // fix: the server tries at boot and keeps trying.

    fn serve() {
        std::thread::spawn(move || {
            keeps_trying_keeper();
        });
        existing.serve();
    }

    // how often the keeper looks. Ten seconds, not thirty, because ten seconds
    // is also the first step of the backoff below and a schedule finer than the
    // clock that reads it is a fiction. A look is a handful of directory
    // entries; a drain only starts when something is actually due.
    fn keeps_trying_tick_secs() -> u64 {
        10
    }

    fn keeps_trying_keeper() {
        loop {
            keeps_trying_pass();
            std::thread::sleep(std::time::Duration::from_secs(keeps_trying_tick_secs()));
        }
    }

    // one look. The notice is written every pass, so the engineer sheet is
    // never reading a stale answer, and a drain is started only when a job is
    // due — a box with an empty queue does no work but the look.
    fn keeps_trying_pass() {
        let due = keeps_trying_due();
        keeps_trying_write_notice(due.clone());
        if due.is_empty() {
            return;
        }
        if transcribe_best_grade() == 0 {
            return;         // nothing can transcribe; the notice says why
        }
        transcribed_drain();
    }

    // every job whose time has come, across every world.
    fn keeps_trying_due() -> Vec<serde_json::Value> {
        let now = now_ms();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for world in transcribed_worlds() {
            for j in keeps_trying_all(world.clone()) {
                if j["next"].as_u64().unwrap_or(0) <= now {
                    let mut e = j.clone();
                    e["world"] = serde_json::json!(world.clone());
                    out.push(e);
                }
            }
        }
        out
    }

    // ---- what the drain is allowed to see ----------------------------------
    // the parent hands out every job it finds; this narrows that to the ones
    // whose next attempt is due, which is what turns a try count into a
    // schedule. The notice needs the unfiltered list and cannot ask `existing`
    // for it (a chain call belongs to its own function), so the queue is read
    // here directly — the same four lines, and they must stay in step with the
    // parent's if the file ever grows a field.

    fn keeps_trying_all(world: String) -> Vec<serde_json::Value> {
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
            let raw = std::fs::read_to_string(e.path()).unwrap_or_default();
            let mut j: serde_json::Value = serde_json::from_str(&raw)
                .unwrap_or(serde_json::json!({}));
            j["id"] = serde_json::json!(id);
            out.push(j);
        }
        out.sort_by_key(|j| j["at"].as_u64().unwrap_or(0));
        out
    }

    fn transcribed_jobs(world: String) -> Vec<serde_json::Value> {
        let now = now_ms();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for j in existing.transcribed_jobs(world) {
            if j["next"].as_u64().unwrap_or(0) <= now {
                out.push(j);
            }
        }
        out
    }

    // ---- the backoff -------------------------------------------------------
    // ten seconds, half a minute, two minutes, ten minutes, then hourly for as
    // long as it takes. Quick enough that a rung fixed while you watch is tried
    // while you are still watching; slow enough that a clip nothing can do costs
    // one attempt an hour rather than one every tick.

    fn keeps_trying_wait_ms(tries: u64) -> u64 {
        if tries <= 1 {
            return 10000;
        }
        if tries == 2 {
            return 30000;
        }
        if tries == 3 {
            return 120000;
        }
        if tries == 4 {
            return 600000;
        }
        3600000
    }

    // ---- nothing is dropped ------------------------------------------------
    // the parent gave a job five tries and then deleted it, which is how a
    // recording could be uploaded, joined, and quietly never transcribed. A job
    // is now kept and rescheduled for ever; a job older than a day is PARKED —
    // moved aside, not deleted — so it is out of the way of today's work and
    // still on the engineer sheet to be looked at.

    fn transcribed_retry(world: String, id: String, tries: u64, why: String) -> bool {
        let file = format!("{}/{}.json", transcribed_queue_dir(world.clone()), id);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let mut j: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        let at = j["at"].as_u64().unwrap_or(0);
        if at > 0 && now_ms() > at + transcribed_job_life_ms() {
            return keeps_trying_park(world, id, why);
        }
        let wait = keeps_trying_wait_ms(tries);
        j["tries"] = serde_json::json!(tries);
        j["next"] = serde_json::json!(now_ms() + wait);
        j["why"] = serde_json::json!(why.clone());
        let _ = std::fs::write(file, j.to_string());
        println!("transcribed: {} not landed ({}); try {} in {}s",
                 id, why, tries, wait / 1000);
        false
    }

    // the parent's "too old" road comes here too, so there is exactly one way
    // a job leaves the queue without landing and it keeps the file.
    fn transcribed_expire(world: String, id: String) -> bool {
        keeps_trying_park(world, id, "older than a day".to_string())
    }

    fn keeps_trying_parked_dir(world: String) -> String {
        format!("{}/parked", transcribed_queue_dir(world))
    }

    fn keeps_trying_park(world: String, id: String, why: String) -> bool {
        let from = format!("{}/{}.json", transcribed_queue_dir(world.clone()), id);
        let raw = std::fs::read_to_string(&from).unwrap_or_default();
        let mut j: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        j["why"] = serde_json::json!(why.clone());
        j["parked"] = serde_json::json!(now_ms());
        let dir = keeps_trying_parked_dir(world.clone());
        let _ = std::fs::create_dir_all(dir.clone());
        let _ = std::fs::write(format!("{}/{}.json", dir, id), j.to_string());
        let _ = std::fs::remove_file(from);
        println!("transcribed: {} parked ({}); it is on the engineer sheet", id, why);
        true
    }

    fn keeps_trying_parked(world: String) -> Vec<serde_json::Value> {
        let mut out: Vec<serde_json::Value> = Vec::new();
        let entries = match std::fs::read_dir(keeps_trying_parked_dir(world)) {
            Ok(e) => e,
            Err(_) => return out,
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let id = match name.strip_suffix(".json") {
                Some(s) => s.to_string(),
                None => continue,
            };
            let raw = std::fs::read_to_string(e.path()).unwrap_or_default();
            let mut j: serde_json::Value = serde_json::from_str(&raw)
                .unwrap_or(serde_json::json!({}));
            j["id"] = serde_json::json!(id);
            out.push(j);
        }
        out
    }

    // ---- a lock whose holder is gone ---------------------------------------
    // the parent's lock goes stale after twenty minutes, which is right for a
    // process that is still working and wrong for one that has died. During a
    // /handover two servers share this file and the incumbent is draining: if
    // it exits mid-clip, twenty minutes of nothing follow. A lock naming a pid
    // that is not running is no lock at all, and `kill -0` is the question.

    fn transcribed_take_lock() -> bool {
        let raw = std::fs::read_to_string(transcribed_lock_file()).unwrap_or_default();
        let held: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let pid = held["pid"].as_u64().unwrap_or(0);
        if pid > 0 && pid != std::process::id() as u64 && !keeps_trying_alive(pid) {
            println!("transcribed: the queue lock was held by {}, which is gone", pid);
            transcribed_drop_lock();
        }
        existing.transcribed_take_lock()
    }

    fn keeps_trying_alive(pid: u64) -> bool {
        match std::process::Command::new("kill")
            .arg("-0").arg(format!("{}", pid)).output() {
            Ok(o) => o.status.success(),
            Err(_) => true,     // cannot ask: assume it is there and wait it out
        }
    }

    // ---- the notice --------------------------------------------------------
    // what the engineer sheet reads. Written on every pass to a file beside the
    // blobs, so the route is a read of one small file and never a walk of every
    // world while a request waits.

    fn keeps_trying_notice_file() -> String {
        format!("{}/queue-notice.json", transcribed_root())
    }

    fn keeps_trying_write_notice(due: Vec<serde_json::Value>) {
        let _ = due;
        let mut waiting: Vec<serde_json::Value> = Vec::new();
        let mut parked: Vec<serde_json::Value> = Vec::new();
        for world in transcribed_worlds() {
            let who = keeps_trying_who(world.clone());
            for j in keeps_trying_all(world.clone()) {
                waiting.push(serde_json::json!({
                    "who": who.clone(),
                    "id": j["id"].clone(),
                    "tries": j["tries"].as_u64().unwrap_or(0),
                    "next": j["next"].as_u64().unwrap_or(0),
                    "why": j["why"].as_str().unwrap_or("waiting its turn") }));
            }
            for j in keeps_trying_parked(world.clone()) {
                parked.push(serde_json::json!({
                    "who": who.clone(),
                    "id": j["id"].clone(),
                    "tries": j["tries"].as_u64().unwrap_or(0),
                    "why": j["why"].as_str().unwrap_or("") }));
            }
        }
        let best = transcribe_best_grade();
        let notice = serde_json::json!({
            "at": now_ms(),
            "best": best,
            "rung": keeps_trying_rung_name(best),
            "why_not": if best == 0 { keeps_trying_why_not() } else { String::new() },
            "waiting": waiting,
            "parked": parked });
        let _ = std::fs::create_dir_all(transcribed_root());
        let _ = std::fs::write(keeps_trying_notice_file(), notice.to_string());
    }

    // a world key is a phone number and must not reach a page; the four-digit
    // tag is what every other surface calls a person by.
    fn keeps_trying_who(world: String) -> String {
        let tag = exchange_audience_of(world.clone());
        if tag.is_empty() {
            return "local".to_string();
        }
        tag
    }

    fn keeps_trying_rung_name(best: i64) -> String {
        if best >= 3 {
            return "api".to_string();
        }
        if best == 2 {
            return "mini".to_string();
        }
        if best == 1 {
            return "local".to_string();
        }
        String::new()
    }

    // ---- why nothing can transcribe ----------------------------------------
    // DIAGNOSTIC ONLY. `transcribe_best_grade()` is the one authority on
    // whether a rung is reachable; this walks the same facts to say WHICH one
    // is missing, and duplicating them is deliberate — a rung node may be
    // unticked, and a sentence that has drifted is better than a node that will
    // not link (/pic-beside's argument for its two copied paths). If this ever
    // disagrees with the number, the number is right.

    fn keeps_trying_why_not() -> String {
        let mut says: Vec<String> = Vec::new();
        let home = std::env::var("HOME").unwrap_or_default();
        let raw = std::fs::read_to_string(format!("{}/.agent-config.json", home))
            .unwrap_or_default();
        let cfg: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let key = cfg["speechmatics"]["api_key"].as_str().unwrap_or("").trim().to_string();
        let env_key = std::env::var("SPEECHMATICS_API_KEY").unwrap_or_default();
        if key.is_empty() && env_key.trim().is_empty() {
            says.push("no speechmatics key in ~/.agent-config.json".to_string());
        }
        if !std::path::Path::new("site/tools/transcribe_api.py").exists() {
            says.push("site/tools/transcribe_api.py is not here".to_string());
        }
        let beat = std::fs::read_to_string(format!("{}/.miso-blobs/whisper/alive", home))
            .unwrap_or_default();
        let b: serde_json::Value = serde_json::from_str(&beat)
            .unwrap_or(serde_json::Value::Null);
        if b["warm"].as_bool() != Some(true)
            || now_ms() > b["at"].as_u64().unwrap_or(0) + 60000 {
            says.push("the mini's transcriber is not beating (launchctl bootstrap com.noob.transcriber)".to_string());
        }
        if says.is_empty() {
            return "no rung answered ready, and every part of one is here".to_string();
        }
        says.join("; ")
    }

    // ---- the route ---------------------------------------------------------
    // screened as /self-check's GET is: free on localhost for tooling, and
    // through the tunnel only for an owner, because it names clip ids.

    fn route(r: request) -> response {
        if r.path != "diag/transcribe" || r.method != "GET" {
            return existing.route(r);
        }
        if r.tunnel {
            let who = context_user_of(r.cookie.clone(), r.tunnel, r.query.clone());
            if !authed(r.cookie.clone()) || authority_rank(who) < 3 {
                return json_response(401, "{\"ok\":false,\"error\":\"owner only\"}".to_string());
            }
        }
        let raw = std::fs::read_to_string(keeps_trying_notice_file()).unwrap_or_default();
        if raw.trim().is_empty() {
            // the keeper has not had its first look yet
            return json_response(200, serde_json::json!({
                "at": 0, "best": transcribe_best_grade(),
                "waiting": [], "parked": [] }).to_string());
        }
        json_response(200, raw)
    }
}

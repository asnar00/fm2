struct feature_Mini;
impl feature_Mini {
    // ---- the grade ---------------------------------------------------------
    // two: the mini's own model, the road when the key or the network is
    // absent. Reachable means the resident transcriber is BEATING and warm —
    // not that it could be started, and not that a model file exists on disk.
    // Nothing else can tell a warm worker from a dead one, and queueing a clip
    // nothing will pick up is how a phone comes to say "transcribing…" for
    // ever. The list order this node reads is /vocabulary's: coarsest first
    // (the constituency), nearest streets after, so a cut takes the far ones.

    fn mini_grade() -> i64 {
        2
    }

    fn transcribe_best_grade() -> i64 {
        let below = existing.transcribe_best_grade();
        if mini_ready() && mini_grade() > below {
            return mini_grade();
        }
        below
    }

    fn mini_beat_life_ms() -> u64 {
        60000
    }

    fn mini_ready() -> bool {
        let raw = std::fs::read_to_string(format!("{}/alive", mini_dir()))
            .unwrap_or_default();
        let b: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        if b["warm"].as_bool() != Some(true) {
            return false;
        }
        let at = b["at"].as_u64().unwrap_or(0);
        at > 0 && now_ms() < at + mini_beat_life_ms()
    }

    // ---- where the worker is spoken to -------------------------------------
    // under the blob root, so a rig with its own HOME talks to its own
    // transcriber and can never be handed the live server's clips. The path is
    // spelled out here rather than borrowed for the reason /pic-beside gives.

    fn mini_dir() -> String {
        format!("{}/.miso-blobs/whisper",
                std::env::var("HOME").unwrap_or(".".to_string()))
    }

    // ---- the seeded prompt -------------------------------------------------
    // whisper's prompt window is about 224 tokens and it is a window, not a
    // limit: what goes past the end pushes the front out. Seven hundred
    // characters is comfortably inside it, and the list arrives coarsest
    // first, so the cut falls on the streets furthest away.

    fn mini_prompt_chars() -> usize {
        700
    }

    fn mini_prompt(vocab: &Vec<serde_json::Value>) -> String {
        let mut words: Vec<String> = Vec::new();
        for v in vocab {
            let s = v.as_str().unwrap_or("").trim().to_string();
            if !s.is_empty() {
                words.push(s);
            }
        }
        if words.is_empty() {
            return String::new();
        }
        let head = words.remove(0);
        let p = if words.is_empty() {
            format!("Canvassing in {}.", head)
        } else {
            format!("Canvassing in {}. Nearby: {}.", head, words.join(", "))
        };
        if p.chars().count() <= mini_prompt_chars() {
            return p;
        }
        let mut cut: String = p.chars().take(mini_prompt_chars()).collect();
        // never end mid-name: back off to the last separator so the model is
        // given whole words to expect.
        match cut.rfind(',') {
            Some(at) => {
                cut.truncate(at);
                format!("{}.", cut)
            }
            None => cut,
        }
    }

    // ---- the rung ----------------------------------------------------------
    // one job file in, one answer file out, named with the clip's id and a
    // nonce. The nonce is what makes a second run of the same clip — a retry,
    // an upgrade — impossible to answer with the first run's words.

    fn mini_patience_secs() -> u64 {
        900
    }

    fn transcribe_rung(job: String) -> String {
        let j: serde_json::Value = serde_json::from_str(&job)
            .unwrap_or(serde_json::Value::Null);
        if j["want"].as_i64() != Some(mini_grade()) {
            return existing.transcribe_rung(job);
        }
        if !mini_ready() {
            return existing.transcribe_rung(job);
        }
        let id = j["id"].as_str().unwrap_or("").to_string();
        let path = j["path"].as_str().unwrap_or("").to_string();
        let empty: Vec<serde_json::Value> = Vec::new();
        let prompt = mini_prompt(j["vocab"].as_array().unwrap_or(&empty));
        let name = format!("{}.{}.json", id, now_ms());
        let inbox = format!("{}/in", mini_dir());
        let outbox = format!("{}/out", mini_dir());
        let _ = std::fs::create_dir_all(inbox.clone());
        let _ = std::fs::create_dir_all(outbox.clone());
        let answer_file = format!("{}/{}", outbox, name);
        if std::fs::write(format!("{}/{}", inbox, name), serde_json::json!({
            "clip": path, "prompt": prompt.clone() }).to_string()).is_err() {
            println!("mini: could not hand {} to the transcriber", id);
            return existing.transcribe_rung(job);
        }
        println!("mini: {} handed over, prompt {} characters",
                 id, prompt.chars().count());
        let mut waited: u64 = 0;
        while waited < mini_patience_secs() {
            if std::path::Path::new(&answer_file).exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            waited = waited + 1;
        }
        if !std::path::Path::new(&answer_file).exists() {
            // the worker died or is wedged: take the job back so a restarted
            // worker does not find it and answer into a file nobody reads.
            let _ = std::fs::remove_file(format!("{}/{}", inbox, name));
            println!("mini: the transcriber did not answer for {} in {}s",
                     id, mini_patience_secs());
            return existing.transcribe_rung(job);
        }
        let raw = std::fs::read_to_string(&answer_file).unwrap_or_default();
        let _ = std::fs::remove_file(&answer_file);
        let a: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let text = a["text"].as_str().unwrap_or("").trim().to_string();
        if text.is_empty() {
            // silence is an ANSWER, not a failure: a clip with nothing said in
            // it must land nothing, and must not be asked again by a lesser
            // rung that would write whisper's invented words into the post.
            if a["silent"].as_bool() == Some(true) {
                println!("mini: {} is silence ({}s of sound); nothing to land",
                         id, a["sound"].as_f64().unwrap_or(0.0));
                return serde_json::json!({
                    "text": "", "rung": "mini", "grade": mini_grade(), "silent": true })
                    .to_string();
            }
            println!("mini: {} came back with no words ({})", id,
                     a["error"].as_str().unwrap_or("no reason given"));
            return existing.transcribe_rung(job);
        }
        println!("mini: {} transcribed, {} characters in {}s, {} MB resident",
                 id, text.len(), a["took"].as_f64().unwrap_or(0.0),
                 a["rss_mb"].as_f64().unwrap_or(0.0));
        serde_json::json!({ "text": text, "rung": "mini", "grade": mini_grade() })
            .to_string()
    }
}

struct feature_Api;
impl feature_Api {
    // ---- the grade ---------------------------------------------------------
    // three: the best words this box can get, and the ones known to work in
    // the field. Reachable means a key and an interpreter — the network is not
    // asked about in advance, because asking costs a round trip and the answer
    // goes stale between the question and the job. A rung that cannot reach
    // the network fails its attempt and /transcribed asks the grade below.

    fn api_grade() -> i64 {
        3
    }

    fn transcribe_best_grade() -> i64 {
        let below = existing.transcribe_best_grade();
        if api_ready() && api_grade() > below {
            return api_grade();
        }
        below
    }

    fn api_ready() -> bool {
        !api_key().is_empty() && !api_python().is_empty()
            && std::path::Path::new(&api_script()).exists()
    }

    // ---- the key -----------------------------------------------------------
    // ~/.agent-config.json beside the SMS and model credentials — never in the
    // repo, never in a plist. It reaches the child in its ENVIRONMENT and not
    // on argv, which is /off-argv's rule and the reason it exists: argv is
    // readable by any local `ps`. Nothing here prints the key, its length or
    // its prefix.

    fn api_config() -> serde_json::Value {
        let raw = std::fs::read_to_string(format!(
            "{}/.agent-config.json", std::env::var("HOME").unwrap_or_default()))
            .unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null)
    }

    fn api_key() -> String {
        match std::env::var("SPEECHMATICS_API_KEY") {
            Ok(k) => {
                let k = k.trim().to_string();
                if !k.is_empty() {
                    return k;
                }
            }
            Err(_) => {}
        }
        api_config()["speechmatics"]["api_key"].as_str().unwrap_or("").trim().to_string()
    }

    // ---- what runs it ------------------------------------------------------
    // the script is this node's own asset, so it is in site/ beside the pages
    // and travels with the deploy; unticking the node takes it out again. The
    // interpreter is whatever python3 the box has — the script uses nothing
    // but the standard library so that this is a true statement — and the
    // first of these that exists wins, because a server started by launchd
    // has no brew directory on its PATH.

    fn api_script() -> String {
        "site/tools/transcribe_api.py".to_string()
    }

    fn api_python() -> String {
        match std::env::var("MISO_PY") {
            Ok(p) if !p.is_empty() && std::path::Path::new(&p).exists() => return p,
            _ => {}
        }
        let home = std::env::var("HOME").unwrap_or_default();
        let mut tries: Vec<String> = Vec::new();
        tries.push(format!("{}/.local/bin/python3", home));
        tries.push("/opt/homebrew/bin/python3".to_string());
        tries.push("/usr/bin/python3".to_string());
        for p in tries {
            if std::path::Path::new(&p).exists() {
                return p;
            }
        }
        String::new()
    }

    // ---- the rung ----------------------------------------------------------
    // /transcribed walks the ladder and says which grade it is asking for, so
    // this link answers for three and passes everything else straight on. One
    // attempt: a failure returns nothing and the mini rung is asked next.

    fn transcribe_rung(job: String) -> String {
        let j: serde_json::Value = serde_json::from_str(&job)
            .unwrap_or(serde_json::Value::Null);
        if j["want"].as_i64() != Some(api_grade()) {
            return existing.transcribe_rung(job);
        }
        if !api_ready() {
            return existing.transcribe_rung(job);
        }
        let path = j["path"].as_str().unwrap_or("").to_string();
        let empty: Vec<serde_json::Value> = Vec::new();
        let phrases: Vec<String> = j["vocab"].as_array().unwrap_or(&empty).iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .filter(|v| !v.is_empty() && !v.contains(','))
            .collect();
        let out = std::process::Command::new(api_python())
            .arg(api_script())
            .arg(path)
            .arg(phrases.join(","))
            .env("SPEECHMATICS_API_KEY", api_key())
            .output();
        let o = match out {
            Ok(o) => o,
            Err(e) => {
                println!("api: could not start the transcriber ({})", e);
                return existing.transcribe_rung(job);
            }
        };
        let said = String::from_utf8_lossy(&o.stdout).to_string();
        let r: serde_json::Value = serde_json::from_str(said.trim())
            .unwrap_or(serde_json::Value::Null);
        let text = api_words(&r);
        if text.is_empty() {
            println!("api: {} came back with no words ({})",
                     j["id"].as_str().unwrap_or(""),
                     r["error"].as_str().unwrap_or("no reason given"));
            return existing.transcribe_rung(job);
        }
        println!("api: {} transcribed, {} characters, {} phrases seeded",
                 j["id"].as_str().unwrap_or(""), text.len(), phrases.len());
        serde_json::json!({ "text": text, "rung": "api", "grade": api_grade() })
            .to_string()
    }

    // the speaker-labelled text is what goes in the post: two people at a
    // doorstep read as two people. With one speaker Speechmatics still labels
    // it "A: ", which says nothing, so a single-speaker answer is the plain
    // words instead.
    fn api_words(r: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        let segs = r["segments"].as_array().unwrap_or(&empty);
        let mut speakers: Vec<String> = Vec::new();
        for s in segs {
            let who = s["speaker"].as_str().unwrap_or("").to_string();
            if !who.is_empty() && !speakers.contains(&who) {
                speakers.push(who);
            }
        }
        if speakers.len() > 1 {
            let t = r["text"].as_str().unwrap_or("").trim().to_string();
            if !t.is_empty() {
                return t;
            }
        }
        r["raw_text"].as_str().unwrap_or("").trim().to_string()
    }
}

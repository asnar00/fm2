struct feature_Corrected;
impl feature_Corrected {
    // ---- where this sits ---------------------------------------------------
    // BEFORE the words land, not after. This link is the newest on
    // transcribed_land, so it runs first and hands the corrected sentence to
    // everything inside it: /as-posts writes the corrected words, `edited` is
    // bumped once, /exchange carries them, and /from-the-words titles what was
    // actually said rather than what was misheard. Landing twice would have put
    // the wrong words on every phone for a second and titled them.
    //
    // The cost is that the words arrive one model call later than they used to.
    // That is the trade taken: a name that is wrong is worse than a note that
    // is two seconds late.

    fn transcribed_land(world: String, id: String, text: String, rung: String, grade: i64) {
        let words = corrected_pass(world.clone(), id.clone(), text.clone(), grade);
        existing.transcribed_land(world, id, words, rung, grade);
    }

    // grade 1 is the on-device whisper, which is not ticked anywhere and was
    // never good enough to be worth correcting; 2 and 3 are the mini and
    // Speechmatics.
    fn corrected_floor() -> i64 {
        2
    }

    fn corrected_pass(world: String, id: String, text: String, grade: i64) -> String {
        if grade < corrected_floor() || text.trim().is_empty() {
            return text;
        }
        let card = transcribed_card_of(world.clone(), id.clone());
        if card.is_null() {
            return text;
        }
        // the author's own words are never touched, and the cheapest way to
        // know is the one /as-posts uses: a text block whose `auto` hash still
        // matches what was last written for it is ours to change. One
        // keystroke and it is theirs for good.
        if corrected_author_edited(&card) {
            return text;
        }
        let near = corrected_nearby(&card);
        if near.is_empty() {
            return text;            // nothing to match against; nothing to do
        }
        let names = corrected_names(text.clone());
        if names.is_empty() {
            return text;            // nothing that looks like a name was said
        }
        // what the correction was actually given, in the log beside its answer:
        // when a name comes back wrong the first question is always "was the
        // right one even offered?", and this is the only place that can say.
        let shown: String = near.join(", ").chars().take(220).collect();
        println!("corrected: {} heard {:?}; near = {}", id, names, shown);
        let asked = corrected_ask(text.clone(), near.clone());
        if asked["ok"].as_bool() != Some(true) {
            println!("corrected: {} not corrected ({})", id,
                     asked["why"].as_str().unwrap_or("no reason given"));
            return text;
        }
        let said = asked["text"].as_str().unwrap_or("").trim().to_string();
        if said.is_empty() || said == text.trim() {
            return text;
        }
        if !corrected_safe(text.clone(), said.clone(), near) {
            println!("corrected: {} DISCARDED an answer that changed more than names: {:?}",
                     id, said);
            corrected_log(world, id, text.clone(), said, false);
            return text;
        }
        corrected_log(world.clone(), id.clone(), text.clone(), said.clone(), true);
        println!("corrected: {} \"{}\" -> \"{}\"", id, text.trim(), said);
        said
    }

    // ---- has a thumb been here? --------------------------------------------
    // /as-posts stamps `auto` with a hash of the words it last wrote. A block
    // whose text no longer hashes to its stamp was edited by hand.

    fn corrected_author_edited(card: &serde_json::Value) -> bool {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") != "text" {
                continue;
            }
            let cur = b["text"].as_str().unwrap_or("").to_string();
            if cur.trim().is_empty() {
                return false;       // nothing there yet: ours to fill
            }
            return b["auto"].as_u64().unwrap_or(0) != as_posts_hash(cur);
        }
        false
    }

    // ---- what is nearby ----------------------------------------------------
    // /vocabulary's own list for this post — which, with /near-the-post, is the
    // streets and places within four hundred metres of where it was made.

    fn corrected_nearby(card: &serde_json::Value) -> Vec<String> {
        transcribe_vocab(card.to_string())
    }

    // ---- what looks like a name --------------------------------------------
    // a capitalised word that is not the first of a sentence, or any word
    // sitting next to a street word. Cheap, and it only decides whether to
    // spend a call at all — the model does the real judging.

    fn corrected_street_words() -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for w in ["street", "road", "lane", "close", "hill", "avenue", "way",
                  "drive", "place", "square", "court", "gardens", "crescent",
                  "terrace", "row", "park", "walk", "yard", "mews", "rise"] {
            v.push(w.to_string());
        }
        v
    }

    fn corrected_bare(w: String) -> String {
        w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_string()
    }

    fn corrected_names(text: String) -> Vec<String> {
        let words: Vec<String> = text.split_whitespace().map(|w| w.to_string()).collect();
        let streets = corrected_street_words();
        let mut out: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < words.len() {
            let bare = corrected_bare(words[i].clone());
            if bare.is_empty() {
                i = i + 1;
                continue;
            }
            let capital = bare.chars().next().map(|c| c.is_uppercase()) == Some(true);
            let mut beside = false;
            if i + 1 < words.len() {
                if streets.contains(&corrected_bare(words[i + 1].clone()).to_lowercase()) {
                    beside = true;
                }
            }
            if i > 0 {
                if streets.contains(&corrected_bare(words[i - 1].clone()).to_lowercase()) {
                    beside = true;
                }
            }
            if (capital && i > 0) || beside {
                if !out.contains(&bare) {
                    out.push(bare);
                }
            }
            i = i + 1;
        }
        out
    }

    // ---- the guard ---------------------------------------------------------
    // the model is asked for the sentence back with only the names changed, and
    // this is what checks that it did. Three tests, and any of them failing
    // throws the answer away:
    //   * the word count moved by more than two;
    //   * a word appeared that was neither in the original nor in the nearby
    //     list — the model wrote something, not corrected something;
    //   * a word disappeared that was not name-shaped — it rewrote the prose.

    fn corrected_slack() -> usize {
        2
    }

    fn corrected_bag(text: String) -> Vec<String> {
        text.split_whitespace()
            .map(|w| corrected_bare(w.to_string()).to_lowercase())
            .filter(|w| !w.is_empty())
            .collect()
    }

    fn corrected_safe(before: String, after: String, near: Vec<String>) -> bool {
        let a = corrected_bag(before.clone());
        let b = corrected_bag(after);
        let long = if a.len() > b.len() { a.len() - b.len() } else { b.len() - a.len() };
        if long > corrected_slack() {
            return false;
        }
        // every word of the nearby phrases, so "Broadwick Street" allows both
        let mut allowed: Vec<String> = Vec::new();
        for p in near.iter() {
            for w in p.split_whitespace() {
                let bare = corrected_bare(w.to_string()).to_lowercase();
                if !bare.is_empty() {
                    allowed.push(bare);
                }
            }
        }
        let names = corrected_names(before.clone());
        let mut name_bag: Vec<String> = Vec::new();
        for n in names.iter() {
            name_bag.push(n.to_lowercase());
        }
        for w in b.iter() {
            if a.contains(w) {
                continue;
            }
            if !allowed.contains(w) {
                return false;       // invented, not corrected
            }
        }
        for w in a.iter() {
            if b.contains(w) {
                continue;
            }
            if !name_bag.contains(w) {
                return false;       // prose was rewritten
            }
        }
        true
    }

    // ---- the record --------------------------------------------------------
    // every correction, kept and readable. Ash wants a manual fix screen after
    // the field test and this file is its seed: what the recogniser heard, what
    // was written instead, and the ones that were thrown away — which are the
    // interesting ones for judging whether the guard is right.

    fn corrected_log_file() -> String {
        format!("{}/corrections.jsonl", vocab_context_dir())
    }

    fn corrected_log(world: String, id: String, before: String, after: String, took: bool) {
        use std::io::Write;
        let line = serde_json::json!({
            "at": now_ms(), "who": exchange_audience_of(world), "id": id,
            "before": before.trim(), "after": after.trim(), "took": took }).to_string();
        let _ = std::fs::create_dir_all(vocab_context_dir());
        let f = std::fs::OpenOptions::new().create(true).append(true)
            .open(corrected_log_file());
        if let Ok(mut f) = f {
            let _ = writeln!(f, "{}", line);
        }
    }

    // ---- the call ----------------------------------------------------------
    // Haiku 4.5 by curl, key on stdin inside a -K config and never on argv —
    // /off-argv's rule, /reports' idiom, and the same road /from-the-words
    // takes. A separate call with its own prompt: this one is not writing, it
    // is choosing between what was heard and what is on the map.

    fn corrected_model() -> String {
        let m = corrected_config()["anthropic"]["correct_model"].as_str()
            .unwrap_or("").trim().to_string();
        if m.is_empty() {
            return "claude-haiku-4-5".to_string();
        }
        m
    }

    fn corrected_config() -> serde_json::Value {
        let raw = std::fs::read_to_string(format!(
            "{}/.agent-config.json", std::env::var("HOME").unwrap_or_default()))
            .unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null)
    }

    fn corrected_key() -> String {
        match std::env::var("ANTHROPIC_API_KEY") {
            Ok(k) => {
                let k = k.trim().to_string();
                if !k.is_empty() {
                    return k;
                }
            }
            Err(_) => {}
        }
        corrected_config()["anthropic"]["api_key"].as_str().unwrap_or("").trim().to_string()
    }

    fn corrected_escape(s: String) -> String {
        s.replace("\\", "\\\\").replace("\"", "\\\"")
    }

    fn corrected_system() -> String {
        String::from(concat!(
            "A canvasser's spoken note has been transcribed, and the recogniser ",
            "may have misheard the names of streets and places. You are given ",
            "the sentence and a list of the streets and places within a few ",
            "hundred metres of where it was recorded. Where a word in the ",
            "sentence is most likely one of those places misheard, replace it. ",
            "Change NOTHING else: not the wording, not the punctuation, not the ",
            "grammar, not a single ordinary word. If nothing in the sentence ",
            "looks like a misheard nearby place, answer with the sentence ",
            "exactly as given. Answer with the sentence and nothing else — no ",
            "quotation marks, no explanation, no preamble."))
    }

    fn corrected_ask(text: String, near: Vec<String>) -> serde_json::Value {
        use std::io::Write;
        let key = corrected_key();
        if key.is_empty() {
            return serde_json::json!({ "ok": false, "why": "no anthropic key on this server" });
        }
        let user = format!("Nearby streets and places: {}\n\nThe sentence:\n\n{}\n",
                           near.join(", "), text);
        let body = serde_json::json!({
            "model": corrected_model(),
            "max_tokens": 1024,
            "system": corrected_system(),
            "messages": [ { "role": "user", "content": user } ]
        }).to_string();
        let dir = format!("{}/corrections-work", transcribed_root());
        let _ = std::fs::create_dir_all(dir.clone());
        let bodyfile = format!("{}/ask-{}.json", dir, std::process::id());
        if std::fs::write(&bodyfile, body.as_bytes()).is_err() {
            return serde_json::json!({ "ok": false, "why": "could not write the request" });
        }
        fm_own_only(&bodyfile);
        let config = format!(
            "url = \"https://api.anthropic.com/v1/messages\"\nheader = \"x-api-key: {}\"\nheader = \"anthropic-version: 2023-06-01\"\nheader = \"content-type: application/json\"\ndata-binary = \"@{}\"\nconnect-timeout = \"15\"\nmax-time = \"60\"\nsilent\nshow-error\n",
            corrected_escape(key), corrected_escape(bodyfile.clone()));
        let child = std::process::Command::new("curl")
            .arg("-K").arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();
        let mut child = match child {
            Ok(c) => c,
            Err(_) => {
                let _ = std::fs::remove_file(&bodyfile);
                return serde_json::json!({ "ok": false, "why": "curl is not on this server" });
            }
        };
        if let Some(mut sin) = child.stdin.take() {
            let _ = sin.write_all(config.as_bytes());
        }
        let out = child.wait_with_output();
        let _ = std::fs::remove_file(&bodyfile);
        let o = match out {
            Ok(o) => o,
            Err(_) => return serde_json::json!({
                "ok": false, "why": "the correction service could not be reached" }),
        };
        corrected_reply(String::from_utf8_lossy(&o.stdout).to_string())
    }

    // an error first, then the stop reason, then the content — the order
    // /reports learned the hard way.
    fn corrected_reply(stdout: String) -> serde_json::Value {
        let v: serde_json::Value = serde_json::from_str(&stdout)
            .unwrap_or(serde_json::Value::Null);
        if v.is_null() {
            return serde_json::json!({
                "ok": false, "why": "the correction service could not be reached" });
        }
        if !v["error"].is_null() {
            return serde_json::json!({ "ok": false,
                "why": v["error"]["message"].as_str().unwrap_or("refused").to_string() });
        }
        if v["stop_reason"].as_str().unwrap_or("") == "refusal" {
            return serde_json::json!({ "ok": false, "why": "the model declined" });
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut text = String::new();
        for b in v["content"].as_array().unwrap_or(&empty) {
            if b["type"].as_str().unwrap_or("") == "text" {
                text.push_str(b["text"].as_str().unwrap_or(""));
            }
        }
        let in_t = v["usage"]["input_tokens"].as_f64().unwrap_or(0.0);
        let out_t = v["usage"]["output_tokens"].as_f64().unwrap_or(0.0);
        println!("corrected: {} in, {} out, ${:.6} ({})",
                 in_t, out_t,
                 in_t * 1.0 / 1000000.0 + out_t * 5.0 / 1000000.0,
                 corrected_model());
        let said = text.trim().trim_matches('"').trim().to_string();
        serde_json::json!({ "ok": true, "text": said })
    }
}

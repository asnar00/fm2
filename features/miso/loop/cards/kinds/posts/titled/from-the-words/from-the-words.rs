struct feature_FromTheWords;
impl feature_FromTheWords {
    // ---- when ---------------------------------------------------------------
    // the moment a transcript lands, and only then. A post minted from a
    // recording has an empty title until somebody types one, and by the time
    // the words are here there is enough to name it with.

    fn transcribed_land(world: String, id: String, text: String, rung: String, grade: i64) {
        existing.transcribed_land(world.clone(), id.clone(), text.clone(), rung, grade);
        if text.trim().is_empty() {
            return;         // no speech: nothing to name it after, and no call
        }
        from_the_words_queue(world.clone(), id.clone(), text);
        from_the_words_try(world, id);
    }

    // ---- the little queue of its own ---------------------------------------
    // a title that could not be written is worth another go, and the keeper's
    // look is where retries belong. It is a separate queue from the clips': a
    // clip whose words have landed is finished, and re-queueing it would ask
    // for the transcript again.

    fn from_the_words_dir(world: String) -> String {
        format!("{}/titles", transcribed_queue_dir(world))
    }

    fn from_the_words_queue(world: String, id: String, text: String) {
        let dir = from_the_words_dir(world);
        let _ = std::fs::create_dir_all(dir.clone());
        let _ = std::fs::write(format!("{}/{}.json", dir, id), serde_json::json!({
            "id": id, "at": now_ms(), "tries": 0, "next": 0,
            "words": text }).to_string());
    }

    fn from_the_words_forget(world: String, id: String) {
        let _ = std::fs::remove_file(format!("{}/{}.json", from_the_words_dir(world), id));
    }

    // the keeper's look, extended. Every title job whose time has come is
    // tried; the backoff is /keeps-trying's own, so a model that is down costs
    // the same rhythm as a rung that is down.
    fn keeps_trying_pass() {
        existing.keeps_trying_pass();
        for world in transcribed_worlds() {
            let entries = match std::fs::read_dir(from_the_words_dir(world.clone())) {
                Ok(e) => e,
                Err(_) => continue,
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
                if j["next"].as_u64().unwrap_or(0) > now_ms() {
                    continue;
                }
                from_the_words_try(world.clone(), id);
            }
        }
    }

    fn from_the_words_again(world: String, id: String, why: String) {
        let file = format!("{}/{}.json", from_the_words_dir(world), id);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let mut j: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        let tries = j["tries"].as_u64().unwrap_or(0) + 1;
        let at = j["at"].as_u64().unwrap_or(0);
        if at > 0 && now_ms() > at + transcribed_job_life_ms() {
            println!("titles: giving up on {} after a day ({})", id, why);
            let _ = std::fs::remove_file(file);
            return;
        }
        j["tries"] = serde_json::json!(tries);
        j["next"] = serde_json::json!(now_ms() + keeps_trying_wait_ms(tries));
        j["why"] = serde_json::json!(why.clone());
        let _ = std::fs::write(file, j.to_string());
        println!("titles: {} not written ({}); try {} later", id, why, tries);
    }

    // ---- one attempt --------------------------------------------------------

    fn from_the_words_try(world: String, id: String) {
        let file = format!("{}/{}.json", from_the_words_dir(world.clone()), id);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let job: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        if job.is_null() {
            return;
        }
        let card = transcribed_card_of(world.clone(), id.clone());
        if card.is_null() || card["deleted"].as_u64().unwrap_or(0) > 0 {
            from_the_words_forget(world, id);
            return;         // deleted while the call was out: nothing to name
        }
        // the author's own title always wins — /keep's rule, and the reason the
        // check is made again HERE rather than only before the call: a thumb
        // can type a title while the model is thinking.
        if !from_the_words_title_of(&card).trim().is_empty() {
            println!("titles: {} already has a title; leaving it alone", id);
            from_the_words_forget(world, id);
            return;
        }
        if from_the_words_key().is_empty() {
            from_the_words_again(world, id, "no anthropic key on this server".to_string());
            return;
        }
        let words = job["words"].as_str().unwrap_or("").to_string();
        let asked = from_the_words_ask(words, from_the_words_around(world.clone(), &card));
        if asked["ok"].as_bool() != Some(true) {
            from_the_words_again(world, id,
                                 asked["why"].as_str().unwrap_or("no reason given").to_string());
            return;
        }
        let title = from_the_words_tidy(asked["text"].as_str().unwrap_or("").to_string());
        if title.is_empty() {
            from_the_words_again(world, id, "the answer was not a title".to_string());
            return;
        }
        from_the_words_write(world.clone(), id.clone(), title);
        from_the_words_forget(world, id);
    }

    // ---- what the model is told ---------------------------------------------
    // the words, the project it belongs to, and the same place-phrases the
    // transcriber was seeded with — so a title may say the street the note was
    // made on, spelled the way the map spells it.

    fn from_the_words_around(world: String, card: &serde_json::Value) -> String {
        let mut bits: Vec<String> = Vec::new();
        let project = from_the_words_project(world);
        if !project.is_empty() {
            bits.push(format!("The project is \"{}\".", project));
        }
        let vocab = transcribe_vocab(card.to_string());
        if !vocab.is_empty() {
            let near: Vec<String> = vocab.into_iter().take(12).collect();
            bits.push(format!("Places nearby, spelled correctly: {}.", near.join(", ")));
        }
        bits.join(" ")
    }

    // the current project's name, off the author's own cards. A post that
    // belongs to no project simply has one less thing in the prompt.
    fn from_the_words_project(world: String) -> String {
        let list: serde_json::Value = serde_json::from_str(&exchange_cards_of(world))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "project" {
                continue;
            }
            if c["deleted"].as_u64().unwrap_or(0) > 0 || !c["from"].is_null() {
                continue;
            }
            for b in c["blocks"].as_array().unwrap_or(&empty) {
                if b["kind"].as_str().unwrap_or("") == "title" {
                    let t = b["text"].as_str().unwrap_or("").trim().to_string();
                    if !t.is_empty() {
                        return t;
                    }
                }
            }
        }
        String::new()
    }

    fn from_the_words_system() -> String {
        String::from(concat!(
            "You name a canvasser's own field note, from the note's words. ",
            "Answer with the title and nothing else: no quotation marks, no ",
            "full stop, no preamble, at most six words. Write it the way the ",
            "canvasser would say it to a colleague — plain, specific, lower ",
            "case except for names. Name the thing that happened or the thing ",
            "that matters, never \"voter conversation\" or \"canvassing note\". ",
            "If a street or a person is named in the note, prefer that. If the ",
            "words are too garbled to name, answer with the single word: none."))
    }

    // ---- the call -----------------------------------------------------------
    // Haiku 4.5 through the Messages API, by curl, with the key on stdin inside
    // a -K config and never on argv — /reports' idiom and /off-argv's rule.
    // Rust has no Anthropic SDK, which is the documented case for raw HTTP.

    fn from_the_words_model() -> String {
        let m = from_the_words_config()["anthropic"]["title_model"].as_str()
            .unwrap_or("").trim().to_string();
        if m.is_empty() {
            // the cheapest current model that can do this, and it is plenty for
            // six words. Named here rather than shared with /reports, which
            // wants the strongest one.
            return "claude-haiku-4-5".to_string();
        }
        m
    }

    fn from_the_words_config() -> serde_json::Value {
        let raw = std::fs::read_to_string(format!(
            "{}/.agent-config.json", std::env::var("HOME").unwrap_or_default()))
            .unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null)
    }

    fn from_the_words_key() -> String {
        match std::env::var("ANTHROPIC_API_KEY") {
            Ok(k) => {
                let k = k.trim().to_string();
                if !k.is_empty() {
                    return k;
                }
            }
            Err(_) => {}
        }
        from_the_words_config()["anthropic"]["api_key"].as_str().unwrap_or("").trim().to_string()
    }

    fn from_the_words_escape(s: String) -> String {
        s.replace("\\", "\\\\").replace("\"", "\\\"")
    }

    fn from_the_words_ask(words: String, around: String) -> serde_json::Value {
        use std::io::Write;
        let key = from_the_words_key();
        if key.is_empty() {
            return serde_json::json!({ "ok": false, "why": "no anthropic key on this server" });
        }
        let user = format!("{}\n\nThe note:\n\n{}\n", around, words);
        let body = serde_json::json!({
            "model": from_the_words_model(),
            // six words cannot need more, and a small ceiling is the cut the
            // brief asked for — made at the model, never in the stylesheet.
            "max_tokens": 64,
            "system": from_the_words_system(),
            "messages": [ { "role": "user", "content": user } ]
        }).to_string();
        let dir = format!("{}/titles-work", transcribed_root());
        let _ = std::fs::create_dir_all(dir.clone());
        let bodyfile = format!("{}/ask-{}.json", dir, std::process::id());
        if std::fs::write(&bodyfile, body.as_bytes()).is_err() {
            return serde_json::json!({ "ok": false, "why": "could not write the request" });
        }
        fm_own_only(&bodyfile);
        let config = format!(
            "url = \"https://api.anthropic.com/v1/messages\"\nheader = \"x-api-key: {}\"\nheader = \"anthropic-version: 2023-06-01\"\nheader = \"content-type: application/json\"\ndata-binary = \"@{}\"\nconnect-timeout = \"15\"\nmax-time = \"60\"\nsilent\nshow-error\n",
            from_the_words_escape(key), from_the_words_escape(bodyfile.clone()));
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
                "ok": false, "why": "the title service could not be reached" }),
        };
        from_the_words_reply(String::from_utf8_lossy(&o.stdout).to_string())
    }

    // the reply, read in the order that matters: an error first, then the stop
    // reason, then the content — /reports learned that reading content first
    // prints an empty answer and calls it a result.
    fn from_the_words_reply(stdout: String) -> serde_json::Value {
        let v: serde_json::Value = serde_json::from_str(&stdout)
            .unwrap_or(serde_json::Value::Null);
        if v.is_null() {
            return serde_json::json!({
                "ok": false, "why": "the title service could not be reached" });
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
        // the cost line, in the log, every call: what it read, what it wrote,
        // and what that came to at the model's published rates.
        let in_t = v["usage"]["input_tokens"].as_f64().unwrap_or(0.0);
        let out_t = v["usage"]["output_tokens"].as_f64().unwrap_or(0.0);
        println!("titles: {} in, {} out, ${:.6} ({})",
                 in_t, out_t,
                 in_t * 1.0 / 1000000.0 + out_t * 5.0 / 1000000.0,
                 from_the_words_model());
        serde_json::json!({ "ok": true, "text": text })
    }

    // six words, no quotes, no full stop — the model is told all of this and
    // mostly obeys; this is the belt to that pair of braces.
    fn from_the_words_tidy(text: String) -> String {
        let t = text.trim().trim_matches('"').trim_matches('\'').trim().to_string();
        let t = t.trim_end_matches('.').trim().to_string();
        if t.is_empty() || t.to_lowercase() == "none" {
            return String::new();
        }
        let words: Vec<String> = t.split_whitespace().map(|w| w.to_string()).collect();
        if words.len() > 6 {
            return words[..6].join(" ");
        }
        if t.chars().count() > 70 {
            return t.chars().take(70).collect();
        }
        t
    }

    // ---- landing it ---------------------------------------------------------
    // the same road the words travel: the block, a bumped `edited`, a stamp
    // into the owner's world, and /exchange's hand-on — which a background
    // thread must do for itself (`transcribed_land` documents why).

    fn from_the_words_title_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "title" {
                return b["text"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    fn from_the_words_write(world: String, id: String, title: String) {
        let mut card = transcribed_card_of(world.clone(), id.clone());
        if card.is_null() {
            return;
        }
        // read once more from the card we are about to write: between the call
        // going out and this line, the author may have typed one.
        if !from_the_words_title_of(&card).trim().is_empty() {
            println!("titles: {} was titled while we were asking; leaving it", id);
            return;
        }
        let mut wrote = false;
        if let Some(blocks) = card["blocks"].as_array_mut() {
            for b in blocks.iter_mut() {
                if b["kind"].as_str().unwrap_or("") != "title" {
                    continue;
                }
                b["text"] = serde_json::json!(title.clone());
                b["auto"] = serde_json::json!(true);
                wrote = true;
            }
        }
        if !wrote {
            return;
        }
        let was = card["edited"].as_u64().unwrap_or(0);
        card["edited"] = serde_json::json!(was + 1);
        let before = exchange_cards_of(world.clone());
        transcribed_stamp(world.clone(), card);
        let after = exchange_cards_of(world.clone());
        if after != before {
            exchange_share(world, before, after);
        }
        println!("titles: {} is now \"{}\"", id, title);
    }
}

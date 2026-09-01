struct feature_Reports;
impl feature_Reports {
    // ---- who may -----------------------------------------------------------
    // reports read the whole of a person's collected world and spend somebody's
    // money doing it, so the rung is /authority's shared-write rung: support and
    // above, the same one inviting takes. The toolbar asks once and hides the
    // glyph on a no; each of the three routes asks again for itself, because a
    // toolbar is a decoration and not a gate.

    fn reports_caller(r: &request) -> String {
        let t = cookie_token(r.cookie.clone());
        if !t.is_empty() && token_valid(t.clone()) {
            format!("phone:{}", token_phone(t))
        } else {
            String::new()
        }
    }

    fn reports_allowed(who: String) -> bool {
        !who.is_empty() && authority_rank(who) >= 2
    }

    fn reports_deny() -> response {
        json_response(403,
            "{\"ok\":false,\"error\":\"reports are for support and above\"}".to_string())
    }

    // ---- the key -----------------------------------------------------------
    // /off-argv's precedent, both halves of it: the value lives in
    // ~/.agent-config.json beside the SMS credentials, and it reaches curl on
    // stdin inside a -K config, never on argv where a local `ps` reads it. The
    // environment variable is the override for a box that would rather set it
    // there. Nothing here prints the key, its length or its prefix.

    fn reports_config() -> serde_json::Value {
        let raw = std::fs::read_to_string(format!(
            "{}/.agent-config.json", std::env::var("HOME").unwrap_or_default()))
            .unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null)
    }

    fn reports_key() -> String {
        match std::env::var("ANTHROPIC_API_KEY") {
            Ok(k) => {
                let k = k.trim().to_string();
                if !k.is_empty() {
                    return k;
                }
            }
            Err(_) => {}
        }
        reports_config()["anthropic"]["api_key"].as_str().unwrap_or("").trim().to_string()
    }

    // the model, and it is written out rather than guessed: `claude-opus-5` is
    // the current one. The config may name another; nothing else may.
    fn reports_model() -> String {
        let m = reports_config()["anthropic"]["model"].as_str().unwrap_or("").trim().to_string();
        if m.is_empty() {
            "claude-opus-5".to_string()
        } else {
            m
        }
    }

    // the report-writer's instructions. A seam: a later node that wants a
    // different register — a briefing note, a press line — redefines this and
    // nothing else.
    fn reports_system() -> String {
        String::from(concat!(
            "You are writing a short internal report for a local political campaign team, ",
            "from their own canvassing data. The data is doorstep posts written or dictated ",
            "by canvassers, each with a time, an author and often a location.\n\n",
            "Answer the question you are given, and nothing else. Write in markdown: a few ",
            "sections with `##` headings, short paragraphs, bullet lists, and a table where a ",
            "table genuinely helps (pipe tables). Do not write a title — the report already ",
            "has one. Do not open with a preamble about what you are about to do.\n\n",
            "Ground every claim in the data. Quote a canvasser's own words when they carry the ",
            "point, in quotation marks, and say roughly how many posts a pattern rests on. If the ",
            "data cannot answer part of the question, say so plainly in one sentence rather than ",
            "guessing or padding. If a location is asked for by ward or area and the data only ",
            "carries coordinates, group by proximity and say that is what you did. Be concise: ",
            "a campaign team reads this between doors."))
    }

    // ---- the routes --------------------------------------------------------

    fn route(r: request) -> response {
        if r.path == "reports/may" && r.method == "GET" {
            return reports_may_route(r);
        }
        if r.path == "reports/run" && r.method == "POST" {
            return reports_run_route(r);
        }
        if r.path.starts_with("reports/") && r.path.ends_with(".pdf") && r.method == "GET" {
            return reports_pdf_route(r);
        }
        existing.route(r)
    }

    // may I? — and does the box have a key to do it with. A member gets a 403,
    // which the page half reads as `may: false` and draws no glyph for
    // (/invite's own shape: an unauthorised answer is not an error on the page).
    fn reports_may_route(r: request) -> response {
        let who = reports_caller(&r);
        if !reports_allowed(who.clone()) {
            return reports_deny();
        }
        json_response(200, serde_json::json!({
            "ok": true, "may": true, "key": !reports_key().is_empty()
        }).to_string())
    }

    // run: answer at once, finish later. A route that took two minutes would be
    // a phone that looked broken, so this stamps `working` into the caller's
    // own world, hands the work to a thread and returns.
    fn reports_run_route(r: request) -> response {
        let who = reports_caller(&r);
        if !reports_allowed(who.clone()) {
            println!("reports: refused a run from {}",
                     if who.is_empty() { "nobody".to_string() } else { who.clone() });
            return reports_deny();
        }
        let b: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let id = b["id"].as_str().unwrap_or("").to_string();
        if !reports_id_ok(&id) {
            return json_response(400, "{\"ok\":false,\"error\":\"bad id\"}".to_string());
        }
        let card = reports_card_of(who.clone(), id.clone());
        if card.is_null() {
            return json_response(404, "{\"ok\":false,\"error\":\"no such report\"}".to_string());
        }
        // a run already under way is not started twice — unless its stamp is
        // old enough that nothing can still be running it (a server restarted
        // mid-generation leaves exactly this), in which case there has to be a
        // way back and this is it.
        let st = reports_state_of(&card);
        if st["status"].as_str().unwrap_or("") == "working"
            && now_ms() < st["at"].as_u64().unwrap_or(0) + reports_stale_ms() {
            return json_response(200, "{\"ok\":false,\"why\":\"already working\"}".to_string());
        }
        if reports_key().is_empty() {
            println!("reports: no API key on this box — {} asked for one", tag_of(who.clone()));
            reports_set_state(who, id, serde_json::json!({
                "status": "nokey", "note": "no API key on the server" }));
            return json_response(200, "{\"ok\":false,\"why\":\"no key\"}".to_string());
        }
        reports_set_state(who.clone(), id.clone(), serde_json::json!({
            "status": "working", "note": "" }));
        let w = who.clone();
        let i = id.clone();
        std::thread::spawn(move || {
            reports_generate(w, i);
        });
        json_response(200, "{\"ok\":true}".to_string())
    }

    // the PDF itself. The last path segment is a slug of the report's title so
    // the phone's share sheet has a name to show; the FILE is found by the id
    // parameter, so the slug never reaches the disk and can be anything at all.
    fn reports_pdf_route(r: request) -> response {
        let who = reports_caller(&r);
        if !reports_allowed(who.clone()) {
            return reports_deny();
        }
        let id = reports_query_id(r.query.clone());
        if !reports_id_ok(&id) {
            return json_response(400, "{\"ok\":false,\"error\":\"bad id\"}".to_string());
        }
        // the card must be in the CALLER's own world: that one lookup is what
        // stops an id being a way to read somebody else's report
        if reports_card_of(who.clone(), id.clone()).is_null() {
            return json_response(404, "{\"ok\":false,\"error\":\"no such report\"}".to_string());
        }
        let file = format!("{}/{}", reports_dir(who), reports_pdf_name(id));
        match std::fs::read(file) {
            Ok(bytes) => response { status: 200, ctype: "application/pdf".to_string(),
                                    body: bytes, set_cookie: String::new(),
                                    cache: "no-store".to_string() },
            Err(_) => json_response(404,
                "{\"ok\":false,\"error\":\"there is no pdf for that report yet\"}".to_string()),
        }
    }

    // ---- names, ids and places on disk -------------------------------------

    fn reports_stale_ms() -> u64 {
        600000
    }

    // an id may be anything a card id is — a name and a millisecond — but it
    // may not be a path. The strictness that matters is in reports_safe, which
    // is what actually builds a filename.
    fn reports_id_ok(id: &String) -> bool {
        !id.is_empty() && id.len() < 120
            && !id.contains('/') && !id.contains('\\') && !id.contains("..")
    }

    fn reports_safe(s: String) -> String {
        let mut out = String::new();
        for c in s.chars() {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                out.push(c);
            } else {
                out.push('_');
            }
        }
        out
    }

    // this node's own corner of the blob store — the same directory family the
    // recordings use, a subdirectory of its own so nothing can collide, and
    // built here rather than borrowed so /reports carries no build-time
    // dependency on /mirror.
    fn reports_dir(who: String) -> String {
        format!("{}/.miso-blobs/{}/reports",
                std::env::var("HOME").unwrap_or(".".to_string()),
                reports_safe(who))
    }

    fn reports_pdf_name(id: String) -> String {
        format!("report.{}.pdf", reports_safe(id))
    }

    fn reports_slug(title: String) -> String {
        let mut out = String::new();
        for c in title.to_lowercase().chars() {
            if out.len() >= 40 {
                break;
            }
            if c.is_ascii_alphanumeric() {
                out.push(c);
            } else if !out.ends_with('-') && !out.is_empty() {
                out.push('-');
            }
        }
        let out = out.trim_matches('-').to_string();
        if out.is_empty() {
            "report".to_string()
        } else {
            out
        }
    }

    fn reports_query_id(q: String) -> String {
        for part in q.split('&') {
            match part.strip_prefix("id=") {
                Some(v) => return reports_unesc(v.to_string()),
                None => {}
            }
        }
        String::new()
    }

    // percent-decoding, for the one parameter this node reads. A '+' is a
    // space in a query string, and a card id can carry one when a person's name
    // does.
    fn reports_unesc(s: String) -> String {
        let b = s.replace('+', " ").into_bytes();
        let mut out: Vec<u8> = Vec::new();
        let mut i = 0usize;
        while i < b.len() {
            if b[i] == 37 && i + 2 < b.len() {
                let hi = reports_hex(b[i + 1]);
                let lo = reports_hex(b[i + 2]);
                if hi < 16 && lo < 16 {
                    out.push(hi * 16 + lo);
                    i += 3;
                    continue;
                }
            }
            out.push(b[i]);
            i += 1;
        }
        String::from_utf8_lossy(&out).to_string()
    }

    fn reports_hex(c: u8) -> u8 {
        if c >= 48 && c <= 57 {
            return c - 48;
        }
        if c >= 97 && c <= 102 {
            return c - 87;
        }
        if c >= 65 && c <= 70 {
            return c - 55;
        }
        255
    }

    // a world key is a phone number and must never reach a log — the opaque
    // tag is what /whole-number established for exactly this
    fn tag_of(who: String) -> String {
        match who.strip_prefix("phone:") {
            Some(p) => tag(p.to_string()),
            None => "somebody".to_string(),
        }
    }

    // ---- the card, and its one state block ---------------------------------
    // the state lives in a block of kind `report`, which /cards' renderer draws
    // nothing for — /location's idiom. Nothing top-level is added, so /guard's
    // merge and /exchange's copy need to learn nothing about reports at all.

    fn reports_card_of(who: String, id: String) -> serde_json::Value {
        let list: serde_json::Value = serde_json::from_str(&exchange_cards_of(who))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            if c["type"].as_str().unwrap_or("") != "report" {
                continue;
            }
            // a copy is not yours to run: /exchange's own test
            if !c["from"].is_null() {
                continue;
            }
            return c.clone();
        }
        serde_json::Value::Null
    }

    fn reports_state_of(card: &serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "report" {
                return b.clone();
            }
        }
        serde_json::json!({ "kind": "report", "status": "new" })
    }

    fn reports_put_state(card: &mut serde_json::Value, st: serde_json::Value) {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut at: i64 = -1;
        let mut i: i64 = 0;
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "report" {
                at = i;
            }
            i += 1;
        }
        if at >= 0 {
            card["blocks"][at as usize] = st;
            return;
        }
        if let Some(arr) = card["blocks"].as_array_mut() {
            arr.push(st);
        }
    }

    fn reports_query_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "text" {
                return b["text"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    fn reports_title_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "title" {
                let t = b["text"].as_str().unwrap_or("").trim().to_string();
                if !t.is_empty() {
                    return t;
                }
            }
        }
        "report".to_string()
    }

    // patch the state block and put the card back in its owner's world. Every
    // stamp moves `edited`, because /guard keeps the newer edit of a shared id
    // and a stamp that did not move it would simply be discarded.
    fn reports_set_state(who: String, id: String, patch: serde_json::Value) {
        let mut card = reports_card_of(who.clone(), id);
        if card.is_null() {
            return;
        }
        let mut st = reports_state_of(&card);
        if let Some(o) = patch.as_object() {
            for k in o.keys() {
                st[k.as_str()] = patch[k.as_str()].clone();
            }
        }
        st["kind"] = serde_json::json!("report");
        let now = now_ms();
        st["at"] = serde_json::json!(now);
        reports_put_state(&mut card, st);
        card["edited"] = serde_json::json!(now);
        reports_stamp(who, card);
    }

    // one card into a named world, through the door /exchange gives a card by:
    // a `set` carrying that card alone, which /guard merges by id (so nothing
    // of theirs can be displaced), /remember logs and /converge relays to their
    // open pages. That relay is what makes the report page update itself the
    // moment the answer lands, with no polling and no clock.
    fn reports_stamp(who: String, card: serde_json::Value) {
        let mut one: Vec<serde_json::Value> = Vec::new();
        one.push(card);
        let value = serde_json::Value::Array(one).to_string();
        let msg = serde_json::json!({
            "type": "CtxOp",
            "_from": exchange_audience_of(who.clone()),
            "data": {
                "path": "miso/loop/cards",
                "name": "cards",
                "op": "set",
                "value": value
            }
        }).to_string();
        let saved = context_user_now();
        context_user_set(who.clone());
        let reply = handle_msg(msg);
        context_user_set(saved);
        let rv: serde_json::Value = serde_json::from_str(&reply)
            .unwrap_or(serde_json::Value::Null);
        if rv["type"].as_str().unwrap_or("") != "CtxUpdate" {
            println!("reports: a stamp did not land ({})",
                     rv["error"].as_str().unwrap_or("no reason given"));
        }
    }

    // ---- the generation thread ---------------------------------------------
    // every way out of this function stamps something: there is no path that
    // leaves a card saying `working` while nothing is working.

    fn reports_generate(who: String, id: String) {
        let card = reports_card_of(who.clone(), id.clone());
        if card.is_null() {
            return;
        }
        let query = reports_query_of(&card);
        let title = reports_title_of(&card);
        if query.trim().is_empty() {
            reports_set_state(who, id, serde_json::json!({
                "status": "failed", "note": "this report has no question in it" }));
            return;
        }
        let work = format!("{}/work", reports_dir(who.clone()));
        let corpus = reports_corpus(who.clone());
        let answer = reports_ask(query.clone(),
                                 corpus["text"].as_str().unwrap_or("").to_string(),
                                 work.clone());
        if !answer["ok"].as_bool().unwrap_or(false) {
            let why = answer["why"].as_str().unwrap_or("the report could not be written");
            println!("reports: {} — a report failed: {}", tag_of(who.clone()), why);
            reports_set_state(who, id, serde_json::json!({
                "status": "failed", "note": why }));
            return;
        }
        let html = reports_html(title.clone(), query,
                                answer["text"].as_str().unwrap_or("").to_string(),
                                corpus.clone(),
                                answer["cut"].as_bool().unwrap_or(false));
        let dir = reports_dir(who.clone());
        let _ = std::fs::create_dir_all(dir.clone());
        let out = format!("{}/{}", dir, reports_pdf_name(id.clone()));
        let trouble = reports_pdf(html, out.clone(), work);
        if !trouble.is_empty() {
            println!("reports: {} — could not print a report: {}", tag_of(who.clone()), trouble);
            reports_set_state(who, id, serde_json::json!({
                "status": "failed", "note": trouble }));
            return;
        }
        println!("reports: {} — wrote {} over {} post(s)",
                 tag_of(who.clone()), reports_pdf_name(id.clone()),
                 corpus["n"].as_u64().unwrap_or(0));
        reports_set_state(who, id, serde_json::json!({
            "status": "ready",
            "pdf": reports_slug(title),
            "generated": now_ms(),
            "through": corpus["through"].clone(),
            "n": corpus["n"].clone(),
            "note": ""
        }));
    }

    // ---- the data ----------------------------------------------------------
    // the asker's OWN world and no other, which is what makes "a report can
    // never tell you something you could not already see" structural rather
    // than a filter: there is no second world in scope to read.
    //
    // This is the seam a narrower report joins at — one ward, one project, one
    // week are all this function returning less, and nothing else changing.

    fn reports_post_cap() -> usize {
        300
    }

    fn reports_text_cap() -> usize {
        160000
    }

    fn reports_corpus(who: String) -> serde_json::Value {
        let list: serde_json::Value = serde_json::from_str(&exchange_cards_of(who))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut posts: Vec<serde_json::Value> = Vec::new();
        let mut projects: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            let k = c["type"].as_str().unwrap_or("");
            if k == "post" {
                posts.push(c.clone());
            }
            if k == "project" {
                projects.push(c.clone());
            }
        }
        posts.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            b["created"].as_u64().unwrap_or(0).cmp(&a["created"].as_u64().unwrap_or(0))
        });
        let total = posts.len();
        let mut text = String::new();
        if !projects.is_empty() {
            text.push_str("PROJECTS THIS TEAM IS WORKING ON\n");
            for p in projects.iter() {
                text.push_str(&format!("- {}: {}\n",
                    reports_title_of(p), reports_words_of(p).replace('\n', " ")));
            }
            text.push('\n');
        }
        text.push_str("DOORSTEP POSTS, NEWEST FIRST\n\n");
        let mut n = 0usize;
        let mut through = 0u64;
        let mut points: Vec<serde_json::Value> = Vec::new();
        for c in posts.iter() {
            if n >= reports_post_cap() || text.len() >= reports_text_cap() {
                break;
            }
            let when = c["created"].as_u64().unwrap_or(0);
            if when > through {
                through = when;
            }
            let at = card_place_of(c.to_string());
            let where_ = if at.is_null() {
                "not recorded".to_string()
            } else {
                format!("{:.5}, {:.5}",
                        at["lat"].as_f64().unwrap_or(0.0), at["lon"].as_f64().unwrap_or(0.0))
            };
            if !at.is_null() {
                points.push(serde_json::json!({
                    "lat": at["lat"].clone(), "lon": at["lon"].clone() }));
            }
            n += 1;
            text.push_str(&format!(
                "--- post {}\nwhen: {} {}\nwho: {}\nwhere: {}\nwords: {}\n\n",
                n, reports_date(when), reports_time(when),
                c["owner"].as_str().unwrap_or("someone"), where_,
                reports_words_of(c)));
        }
        if n == 0 {
            text.push_str("(there are no posts in this world yet)\n");
        }
        serde_json::json!({
            "text": text, "n": n, "total": total,
            "through": through, "points": points
        })
    }

    // every text block of a card, in order — which for a post is what was
    // written or what /dictate transcribed into it
    fn reports_words_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out = String::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") != "text" {
                continue;
            }
            let t = b["text"].as_str().unwrap_or("").trim().to_string();
            if t.is_empty() {
                continue;
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&t);
        }
        if out.is_empty() {
            return "(nothing written)".to_string();
        }
        out
    }

    // ---- dates, without a clock in the browser half ------------------------
    // /browse's civil-from-days arithmetic, in UTC, for the same reason it was
    // written: the loop's other half has no time zone and no SystemTime.

    fn reports_date(ms: u64) -> String {
        if ms == 0 {
            return "unknown".to_string();
        }
        let days = (ms / 86400000) as i64;
        format!("{:04}-{:02}-{:02}", browse_civil_year(days),
                browse_civil_month(days), browse_civil_day(days))
    }

    fn reports_time(ms: u64) -> String {
        if ms == 0 {
            return String::new();
        }
        let s = (ms / 1000) % 86400;
        format!("{:02}:{:02}", s / 3600, (s % 3600) / 60)
    }

    // ---- the model call ----------------------------------------------------

    fn reports_curl_escape(s: String) -> String {
        s.replace("\\", "\\\\").replace("\"", "\\\"")
    }

    fn reports_ask(query: String, corpus: String, work: String) -> serde_json::Value {
        use std::io::Write;
        let key = reports_key();
        if key.is_empty() {
            return serde_json::json!({ "ok": false, "why": "no API key on the server" });
        }
        let user = format!(
            "{}\n\n----\n\nThe question this report must answer:\n\n{}\n",
            corpus, query);
        let body = serde_json::json!({
            "model": reports_model(),
            "max_tokens": 16000,
            "fallbacks": "default",
            "system": reports_system(),
            "messages": [ { "role": "user", "content": user } ]
        }).to_string();
        let _ = std::fs::create_dir_all(work.clone());
        let bodyfile = format!("{}/ask.json", work);
        if std::fs::write(&bodyfile, body.as_bytes()).is_err() {
            return serde_json::json!({ "ok": false, "why": "could not write the request" });
        }
        fm_own_only(&bodyfile);
        // the key goes on stdin inside the config, never on argv; the body goes
        // by file because it is large, and the file is owner-only and removed
        // whether the call worked or not
        let config = format!(
            "url = \"https://api.anthropic.com/v1/messages\"\nheader = \"x-api-key: {}\"\nheader = \"anthropic-version: 2023-06-01\"\nheader = \"anthropic-beta: server-side-fallback-2026-07-01\"\nheader = \"content-type: application/json\"\ndata-binary = \"@{}\"\nconnect-timeout = \"15\"\nmax-time = \"900\"\nsilent\nshow-error\n",
            reports_curl_escape(key), reports_curl_escape(bodyfile.clone()));
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
                "ok": false, "why": "the report service could not be reached" }),
        };
        let stdout = String::from_utf8_lossy(&o.stdout).to_string();
        reports_reply(stdout)
    }

    // the reply, read with a real parser and never a regex, and read in the
    // order that matters: an error first, then `stop_reason`, then the content.
    // `refusal` is a real stop reason on this model family and it arrives with
    // HTTP 200 — reading content first would print an empty page and call it a
    // report.
    fn reports_reply(stdout: String) -> serde_json::Value {
        let v: serde_json::Value = serde_json::from_str(&stdout)
            .unwrap_or(serde_json::Value::Null);
        if v.is_null() {
            return serde_json::json!({
                "ok": false, "why": "the report service could not be reached" });
        }
        if !v["error"].is_null() {
            let m = v["error"]["message"].as_str()
                .unwrap_or("the report service refused the request").to_string();
            return serde_json::json!({ "ok": false, "why": m });
        }
        let stop = v["stop_reason"].as_str().unwrap_or("").to_string();
        if stop == "refusal" {
            return serde_json::json!({
                "ok": false, "why": "the model declined to write this report" });
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut text = String::new();
        for b in v["content"].as_array().unwrap_or(&empty) {
            if b["type"].as_str().unwrap_or("") == "text" {
                text.push_str(b["text"].as_str().unwrap_or(""));
            }
        }
        if text.trim().is_empty() {
            return serde_json::json!({
                "ok": false, "why": "the report came back empty" });
        }
        serde_json::json!({ "ok": true, "text": text, "cut": stop == "max_tokens" })
    }

    // ---- printing ----------------------------------------------------------

    fn reports_chrome() -> String {
        match std::env::var("MISO_CHROME") {
            Ok(c) => {
                if !c.is_empty() {
                    return c;
                }
            }
            Err(_) => {}
        }
        let tries = ["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                     "/Applications/Chromium.app/Contents/MacOS/Chromium",
                     "/usr/bin/google-chrome",
                     "/usr/bin/chromium",
                     "/usr/bin/chromium-browser"];
        for t in tries.iter() {
            if std::path::Path::new(t).exists() {
                return t.to_string();
            }
        }
        String::new()
    }

    // how long a print may take before it is abandoned. The wall clock matters:
    // this thread is the only thing that will ever stamp the card, so a printer
    // that never returns is a report that says `working` for ever.
    fn reports_print_ms() -> u32 {
        120000
    }

    // is the file on disk a WHOLE pdf? `--print-to-pdf` writes the trailer
    // last, so `%%EOF` at the end is the difference between a finished document
    // and a file that happens to exist.
    fn reports_pdf_whole(out: &String) -> bool {
        let bytes = match std::fs::read(out) {
            Ok(b) => b,
            Err(_) => return false,
        };
        if bytes.len() < 500 {
            return false;
        }
        let from = bytes.len() - std::cmp::min(1024, bytes.len());
        String::from_utf8_lossy(&bytes[from..]).contains("%%EOF")
    }

    // the HTML is kept beside the PDF rather than deleted, so a print that goes
    // wrong is inspectable afterwards. A throwaway profile directory, so a
    // headless run can never touch anybody's real Chrome.
    //
    // **Chrome does not reliably exit after `--print-to-pdf`** — rig-found, and
    // it is the ordinary case rather than the rare one: on this box the file is
    // written in about four seconds and the process then sits there for ever,
    // which held the generation thread inside `wait_with_output` and left every
    // report saying `working`. So the finished FILE is the signal, not the exit
    // status: the printer is watched for its output, given a moment to close
    // the file, and then killed. A printer that has produced nothing by
    // `reports_print_ms` is killed too, and that is a failure with a sentence
    // on the card.
    fn reports_pdf(html: String, out: String, work: String) -> String {
        let _ = std::fs::create_dir_all(work.clone());
        let page = format!("{}/report.html", work);
        if std::fs::write(&page, html.as_bytes()).is_err() {
            return "could not write the report page".to_string();
        }
        let chrome = reports_chrome();
        if chrome.is_empty() {
            return "there is no Chrome on this server to print with".to_string();
        }
        // a stale pdf from a previous run would look like this one's output
        let _ = std::fs::remove_file(out.clone());
        let profile = format!("{}/chrome", work);
        let _ = std::fs::create_dir_all(profile.clone());
        let spawned = std::process::Command::new(chrome)
            .arg("--headless")
            .arg("--disable-gpu")
            .arg("--no-sandbox")
            .arg("--no-first-run")
            .arg("--no-default-browser-check")
            .arg("--disable-extensions")
            .arg("--disable-background-networking")
            .arg("--disable-component-update")
            .arg("--disable-default-apps")
            .arg("--disable-sync")
            .arg("--no-pdf-header-footer")
            .arg("--virtual-time-budget=8000")
            .arg(format!("--user-data-dir={}", profile))
            .arg(format!("--print-to-pdf={}", out))
            .arg(format!("file://{}", page))
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        let mut child = match spawned {
            Ok(c) => c,
            Err(_) => return "the printer would not run".to_string(),
        };
        let mut waited: u32 = 0;
        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => {}
                Err(_) => break,
            }
            if reports_pdf_whole(&out) {
                // it has written the trailer; give it a breath to close the
                // handle, then take the printer away
                std::thread::sleep(std::time::Duration::from_millis(300));
                let _ = child.kill();
                let _ = child.wait();
                break;
            }
            if waited >= reports_print_ms() {
                let _ = child.kill();
                let _ = child.wait();
                return "the printer did not finish in time".to_string();
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
            waited += 200;
        }
        if !reports_pdf_whole(&out) {
            return "the printed report came out empty".to_string();
        }
        String::new()
    }

    // ---- the print page ----------------------------------------------------
    // the one artifact that leaves the app, so it is LIGHT: it goes on paper.
    // The house restraint holds — quiet type, one accent used sparingly, no
    // rules doing a heading's job — but the palette is inverted, because a dark
    // page printed is a great deal of ink and a filter over it is /taste's
    // standing "no".

    fn reports_print_css() -> String {
        String::from(concat!(
            "@page { size: A4; margin: 16mm 15mm 14mm 15mm; }\n",
            "* { box-sizing: border-box; }\n",
            "body { margin: 0; color: #1a1a1d; background: #fff;\n",
            "  font: 10.5pt/1.55 -apple-system, BlinkMacSystemFont, 'Helvetica Neue', Arial, sans-serif;\n",
            "  -webkit-print-color-adjust: exact; print-color-adjust: exact; }\n",
            ".mark { font-size: 8.5pt; letter-spacing: .18em; text-transform: uppercase;\n",
            "  color: #8a8a92; margin: 0 0 14pt 0; }\n",
            "h1 { font-size: 20pt; line-height: 1.15; font-weight: 600; letter-spacing: -.01em;\n",
            "  margin: 0 0 6pt 0; color: #101012; }\n",
            ".ask { font-size: 10.5pt; color: #4a4a52; margin: 0 0 4pt 0; }\n",
            ".ask em { font-style: normal; color: #1a1a1d; }\n",
            ".rule { height: 2px; background: #3d5a7d; width: 46pt; margin: 10pt 0 16pt 0; }\n",
            "h2 { font-size: 13pt; font-weight: 600; margin: 18pt 0 5pt 0; color: #101012;\n",
            "  page-break-after: avoid; }\n",
            "h3 { font-size: 11pt; font-weight: 600; margin: 13pt 0 4pt 0; color: #101012;\n",
            "  page-break-after: avoid; }\n",
            "h4 { font-size: 10.5pt; font-weight: 600; margin: 11pt 0 3pt 0; color: #3a3a42;\n",
            "  page-break-after: avoid; }\n",
            "p { margin: 0 0 7pt 0; }\n",
            "ul, ol { margin: 0 0 8pt 0; padding-left: 15pt; }\n",
            "li { margin: 0 0 3pt 0; }\n",
            "code { font-family: ui-monospace, Menlo, monospace; font-size: 9pt;\n",
            "  background: #f3f3f6; padding: 0 2pt; border-radius: 2pt; }\n",
            "table { border-collapse: collapse; width: 100%; margin: 4pt 0 12pt 0;\n",
            "  font-size: 9.5pt; page-break-inside: avoid; }\n",
            "th { text-align: left; font-weight: 600; color: #101012;\n",
            "  border-bottom: 1px solid #b9b9c2; padding: 4pt 6pt 4pt 0; }\n",
            "td { border-bottom: 1px solid #e4e4ea; padding: 4pt 6pt 4pt 0;\n",
            "  vertical-align: top; }\n",
            "tr:last-child td { border-bottom: none; }\n",
            ".mapwrap { page-break-inside: avoid; margin-top: 14pt; }\n",
            ".mapframe { position: relative; overflow: hidden; border: 1px solid #d6d6de;\n",
            "  border-radius: 4pt; }\n",
            ".mapscale { position: absolute; left: 0; top: 0; transform-origin: 0 0; }\n",
            ".mapscale img { position: absolute; width: 256px; height: 256px; display: block; }\n",
            ".pin { position: absolute; width: 15px; height: 15px; margin: -7.5px 0 0 -7.5px;\n",
            "  border-radius: 50%; background: #3d5a7d; border: 3px solid #fff;\n",
            "  box-shadow: 0 0 0 1px rgba(0,0,0,.25); }\n",
            ".credit { font-size: 7.5pt; color: #8a8a92; margin: 4pt 0 0 0; }\n",
            "footer { margin-top: 20pt; padding-top: 8pt; border-top: 1px solid #e4e4ea;\n",
            "  font-size: 8.5pt; color: #6a6a72; }\n",
            "footer div { margin: 0 0 2pt 0; }\n"))
    }

    fn reports_html(title: String, query: String, answer: String, corpus: serde_json::Value, cut: bool) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        let points = corpus["points"].as_array().unwrap_or(&empty).clone();
        let n = corpus["n"].as_u64().unwrap_or(0);
        let total = corpus["total"].as_u64().unwrap_or(0);
        let through = corpus["through"].as_u64().unwrap_or(0);
        let mut foot = String::new();
        foot.push_str(&format!("<div>generated {} {} \u{00b7} miso</div>",
                               reports_date(now_ms()), reports_time(now_ms())));
        if through > 0 {
            foot.push_str(&format!("<div>data through {}</div>", reports_date(through)));
        }
        if total > n {
            foot.push_str(&format!(
                "<div>read the newest {} of {} posts \u{2014} older posts were not included</div>",
                n, total));
        } else {
            foot.push_str(&format!("<div>read {} of {} posts</div>", n, total));
        }
        if cut {
            foot.push_str("<div>the answer was cut short at its length limit</div>");
        }
        format!(concat!("<!doctype html><html><head><meta charset=\"utf-8\">",
                        "<title>{}</title><style>{}</style></head><body>",
                        "<div class=\"mark\">miso</div><h1>{}</h1>",
                        "<p class=\"ask\">asked: <em>{}</em></p><div class=\"rule\"></div>",
                        "{}{}<footer>{}</footer></body></html>"),
                reports_esc(title.clone()), reports_print_css(),
                reports_esc(title), reports_esc(query),
                reports_md(answer), reports_map_html(&points), foot)
    }

    // ---- markdown, strictly ------------------------------------------------
    // the text is escaped BEFORE anything is done to it, so nothing the model
    // writes can put markup on the page. What is understood: headings, bullet
    // and numbered lists, pipe tables, bold, italic and code. Everything else
    // is a paragraph.

    fn reports_esc(s: String) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    }

    fn reports_pairs(s: String, mark: String, tag: String) -> String {
        let mut out = String::new();
        let mut rest = s;
        let mut open = false;
        loop {
            let at = match rest.find(&mark) {
                Some(i) => i,
                None => {
                    out.push_str(&rest);
                    break;
                }
            };
            out.push_str(&rest[..at]);
            if open {
                out.push_str(&format!("</{}>", tag));
            } else {
                out.push_str(&format!("<{}>", tag));
            }
            open = !open;
            rest = rest[at + mark.len()..].to_string();
        }
        if open {
            out.push_str(&format!("</{}>", tag));
        }
        out
    }

    fn reports_inline(s: String) -> String {
        let e = reports_esc(s);
        let e = reports_pairs(e, "**".to_string(), "strong".to_string());
        let e = reports_pairs(e, "`".to_string(), "code".to_string());
        reports_pairs(e, "*".to_string(), "em".to_string())
    }

    fn reports_md(text: String) -> String {
        let mut out = String::new();
        let mut mode = String::new();
        let mut para = String::new();
        let mut rows: Vec<String> = Vec::new();
        for raw in text.lines() {
            let t = raw.trim().to_string();
            if t.starts_with('|') && t.ends_with('|') && t.len() > 2 {
                out.push_str(&reports_flush(&mut mode, &mut para));
                rows.push(t);
                continue;
            }
            if !rows.is_empty() {
                out.push_str(&reports_table(&rows));
                rows.clear();
            }
            if t.is_empty() {
                out.push_str(&reports_flush(&mut mode, &mut para));
                continue;
            }
            if t.starts_with("### ") {
                out.push_str(&reports_flush(&mut mode, &mut para));
                out.push_str(&format!("<h4>{}</h4>", reports_inline(t[4..].to_string())));
                continue;
            }
            if t.starts_with("## ") {
                out.push_str(&reports_flush(&mut mode, &mut para));
                out.push_str(&format!("<h3>{}</h3>", reports_inline(t[3..].to_string())));
                continue;
            }
            if t.starts_with("# ") {
                out.push_str(&reports_flush(&mut mode, &mut para));
                out.push_str(&format!("<h2>{}</h2>", reports_inline(t[2..].to_string())));
                continue;
            }
            if t.chars().all(|c| c == '-' || c == '*' || c == '_') && t.len() >= 3 {
                out.push_str(&reports_flush(&mut mode, &mut para));
                continue;
            }
            if t.starts_with("- ") || t.starts_with("* ") {
                if mode == "ul" {
                    out.push_str(&reports_item(&mut mode, &mut para));
                } else {
                    out.push_str(&reports_flush(&mut mode, &mut para));
                    out.push_str("<ul>");
                    mode = "ul".to_string();
                }
                para.push_str(&t[2..]);
                continue;
            }
            if reports_numbered(&t) {
                if mode == "ol" {
                    out.push_str(&reports_item(&mut mode, &mut para));
                } else {
                    out.push_str(&reports_flush(&mut mode, &mut para));
                    out.push_str("<ol>");
                    mode = "ol".to_string();
                }
                let after = match t.find(". ") {
                    Some(i) => t[i + 2..].to_string(),
                    None => t.clone(),
                };
                para.push_str(&after);
                continue;
            }
            // a plain line under a list is that item CONTINUING — markdown's
            // lazy continuation, and the difference between a wrapped bullet
            // and a bullet followed by an orphaned half-sentence (rig-found:
            // the model wraps its lines and the first print broke every long
            // one in two)
            if !para.is_empty() {
                para.push(' ');
            }
            para.push_str(&t);
            if mode.is_empty() {
                mode = "p".to_string();
            }
        }
        if !rows.is_empty() {
            out.push_str(&reports_table(&rows));
        }
        out.push_str(&reports_flush(&mut mode, &mut para));
        out
    }

    fn reports_numbered(t: &String) -> bool {
        let at = match t.find(". ") {
            Some(i) => i,
            None => return false,
        };
        at > 0 && at < 4 && t[..at].chars().all(|c| c.is_ascii_digit())
    }

    // end the pending paragraph or list ITEM, and leave the block open
    fn reports_item(mode: &mut String, para: &mut String) -> String {
        let out = if para.trim().is_empty() {
            String::new()
        } else if mode.as_str() == "ul" || mode.as_str() == "ol" {
            format!("<li>{}</li>", reports_inline(para.clone()))
        } else if mode.as_str() == "p" {
            format!("<p>{}</p>", reports_inline(para.clone()))
        } else {
            String::new()
        };
        para.clear();
        out
    }

    // end the pending item AND close the block it was in
    fn reports_flush(mode: &mut String, para: &mut String) -> String {
        let mut out = reports_item(mode, para);
        if mode.as_str() == "ul" {
            out.push_str("</ul>");
        }
        if mode.as_str() == "ol" {
            out.push_str("</ol>");
        }
        mode.clear();
        out
    }

    // a pipe table: the first row is the header when the second is the usual
    // row of dashes, and otherwise every row is a body row.
    fn reports_table(rows: &Vec<String>) -> String {
        let mut cells: Vec<Vec<String>> = Vec::new();
        for r in rows.iter() {
            let inner = r.trim_matches('|').to_string();
            let one: Vec<String> = inner.split('|').map(|c| c.trim().to_string()).collect();
            cells.push(one);
        }
        if cells.is_empty() {
            return String::new();
        }
        let mut head = false;
        if cells.len() > 1 {
            head = cells[1].iter().all(|c| {
                !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':' || ch == ' ')
            });
        }
        let mut out = String::from("<table>");
        let mut i = 0usize;
        for row in cells.iter() {
            if head && i == 1 {
                i += 1;
                continue;
            }
            let tag = if head && i == 0 { "th" } else { "td" };
            out.push_str("<tr>");
            for c in row.iter() {
                out.push_str(&format!("<{}>{}</{}>", tag, reports_inline(c.clone()), tag));
            }
            out.push_str("</tr>");
            i += 1;
        }
        out.push_str("</table>");
        out
    }

    // ---- the map -----------------------------------------------------------
    // drawn by the server, not by Leaflet: a headless Chrome loading our own
    // page would arrive without a session cookie, would have to be handed a
    // credential to get past /gate, and would then have to be raced for "has
    // the map finished drawing" before the print. Fetching the tiles here
    // removes all three, and the picture of the world is still ours and still
    // cached on our own disk.

    fn reports_map_w() -> f64 {
        1120.0
    }

    fn reports_map_h() -> f64 {
        740.0
    }

    // the print basemap is LIGHT, and that is /taste 9 rather than an
    // oversight: the app's map is dark because the app is dark, and a dark map
    // on paper is either a great deal of ink or a filter working to correct an
    // asset. A different source, not a correction.
    //
    // It is OpenStreetMap's own standard rendering rather than CARTO's light
    // basemap, and that was found on the rig rather than chosen from a list:
    // `basemaps.cartocdn.com` now answers 200 with a tile that says API KEY
    // REQUIRED across it in grey. A watermark is not something to filter off —
    // it is the wrong source. OSM's standard tiles are open, need no key, and
    // are drawn light; the policy they come under asks for a User-Agent naming
    // the application and a way to reach whoever runs it, which is what
    // `reports_agent` sends, and for modest use, which a cached mosaic per
    // report is. MISO_PRINT_TILE_URL points this anywhere.
    fn reports_map_source() -> String {
        match std::env::var("MISO_PRINT_TILE_URL") {
            Ok(u) => {
                if !u.is_empty() {
                    return u;
                }
            }
            Err(_) => {}
        }
        "https://tile.openstreetmap.org/{z}/{x}/{y}.png".to_string()
    }

    // the credit the licence asks for, kept beside the source that earned it
    fn reports_map_credit() -> String {
        if reports_map_source().contains("cartocdn.com") {
            return "\u{00a9} OpenStreetMap contributors \u{00a9} CARTO".to_string();
        }
        "\u{00a9} OpenStreetMap contributors".to_string()
    }

    fn reports_agent() -> String {
        "miso/1.0 (https://miso.xn--nb-lkaa.org; ash.nehru@gmail.com)".to_string()
    }

    fn reports_tiles_dir() -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        let base = match std::env::var("MISO_CONTEXT_DIR") {
            Ok(d) => {
                if d.is_empty() {
                    format!("{}/.miso-context", home)
                } else {
                    d
                }
            }
            Err(_) => format!("{}/.miso-context", home),
        };
        format!("{}/tiles-print", base)
    }

    fn reports_is_png(bytes: &Vec<u8>) -> bool {
        bytes.len() > 8 && bytes[0] == 137 && bytes[1] == 80 && bytes[2] == 78
            && bytes[3] == 71 && bytes[4] == 13 && bytes[5] == 10
            && bytes[6] == 26 && bytes[7] == 10
    }

    // disk first, upstream once — /tiles' own discipline, with this node's own
    // source and its own cache, so /reports has no build-time dependency on
    // /tiles and toggles on its own. An unreachable tile is a MISSING tile: the
    // mosaic leaves a gap and the map is still a map.
    fn reports_tile(z: u32, x: i64, y: i64) -> Vec<u8> {
        let side: i64 = 1 << z;
        if x < 0 || y < 0 || x >= side || y >= side {
            return Vec::new();
        }
        let dir = format!("{}/{}/{}", reports_tiles_dir(), z, x);
        let file = format!("{}/{}.png", dir, y);
        match std::fs::read(file.clone()) {
            Ok(b) => {
                if reports_is_png(&b) {
                    return b;
                }
            }
            Err(_) => {}
        }
        let url = reports_map_source()
            .replace("{z}", &z.to_string())
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string());
        let out = std::process::Command::new("curl")
            .arg("-s").arg("-f").arg("-L")
            .arg("--connect-timeout").arg("5")
            .arg("--max-time").arg("15")
            .arg("-A").arg(reports_agent())
            .arg(url)
            .output();
        let bytes = match out {
            Ok(o) => {
                if o.status.success() {
                    o.stdout
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        };
        if !reports_is_png(&bytes) {
            return Vec::new();
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, bytes.clone());
        bytes
    }

    // web mercator, in pixels at a zoom — the same arithmetic Leaflet does on
    // the screen, done here so the print needs no browser to do it
    fn reports_px_x(lon: f64, z: u32) -> f64 {
        (lon + 180.0) / 360.0 * 256.0 * ((1u64 << z) as f64)
    }

    fn reports_px_y(lat: f64, z: u32) -> f64 {
        let lat = if lat > 85.05 { 85.05 } else if lat < -85.05 { -85.05 } else { lat };
        let r = lat.to_radians();
        let y = (1.0 - (r.tan() + 1.0 / r.cos()).ln() / std::f64::consts::PI) / 2.0;
        y * 256.0 * ((1u64 << z) as f64)
    }

    // the largest zoom whose pins all fit the frame with room to breathe. One
    // pin has no extent, so it gets a street-level zoom rather than the whole
    // planet.
    fn reports_zoom_for(points: &Vec<serde_json::Value>) -> u32 {
        if points.len() < 2 {
            return 15;
        }
        let mut z: u32 = 17;
        loop {
            let mut minx = f64::MAX;
            let mut maxx = f64::MIN;
            let mut miny = f64::MAX;
            let mut maxy = f64::MIN;
            for p in points.iter() {
                let px = reports_px_x(p["lon"].as_f64().unwrap_or(0.0), z);
                let py = reports_px_y(p["lat"].as_f64().unwrap_or(0.0), z);
                if px < minx { minx = px; }
                if px > maxx { maxx = px; }
                if py < miny { miny = py; }
                if py > maxy { maxy = py; }
            }
            if maxx - minx <= reports_map_w() - 90.0 && maxy - miny <= reports_map_h() - 90.0 {
                return z;
            }
            if z <= 2 {
                return 2;
            }
            z -= 1;
        }
    }

    // the seam a ward-boundary overlay joins at: markup positioned in the
    // mosaic's own pixel space, drawn over the tiles and under the pins. It is
    // handed the zoom and the top-left pixel of the frame, which is everything
    // a projection needs. Nothing by default.
    fn reports_map_overlay(z: u32, left: f64, top: f64) -> String {
        let _ = z;
        let _ = left;
        let _ = top;
        String::new()
    }

    fn reports_map_html(points: &Vec<serde_json::Value>) -> String {
        if points.is_empty() {
            return String::new();
        }
        let w = reports_map_w();
        let h = reports_map_h();
        let z = reports_zoom_for(points);
        let mut minx = f64::MAX;
        let mut maxx = f64::MIN;
        let mut miny = f64::MAX;
        let mut maxy = f64::MIN;
        for p in points.iter() {
            let px = reports_px_x(p["lon"].as_f64().unwrap_or(0.0), z);
            let py = reports_px_y(p["lat"].as_f64().unwrap_or(0.0), z);
            if px < minx { minx = px; }
            if px > maxx { maxx = px; }
            if py < miny { miny = py; }
            if py > maxy { maxy = py; }
        }
        let left = (minx + maxx) / 2.0 - w / 2.0;
        let top = (miny + maxy) / 2.0 - h / 2.0;
        let tx0 = (left / 256.0).floor() as i64;
        let ty0 = (top / 256.0).floor() as i64;
        let tx1 = ((left + w) / 256.0).floor() as i64;
        let ty1 = ((top + h) / 256.0).floor() as i64;
        let mut tiles = String::new();
        let mut got = 0usize;
        let mut ty = ty0;
        while ty <= ty1 {
            let mut tx = tx0;
            while tx <= tx1 {
                let bytes = reports_tile(z, tx, ty);
                if !bytes.is_empty() {
                    got += 1;
                    tiles.push_str(&format!(
                        "<img style=\"left:{:.0}px;top:{:.0}px\" src=\"data:image/png;base64,{}\">",
                        (tx as f64) * 256.0 - left, (ty as f64) * 256.0 - top,
                        reports_b64(&bytes)));
                }
                tx += 1;
            }
            ty += 1;
        }
        // no basemap at all is still a map worth printing — the pins carry the
        // shape of where the team went — but it is worth saying so in the log
        if got == 0 {
            println!("reports: the print basemap could not be fetched; the map is pins only");
        }
        let mut pins = String::new();
        for p in points.iter() {
            let px = reports_px_x(p["lon"].as_f64().unwrap_or(0.0), z) - left;
            let py = reports_px_y(p["lat"].as_f64().unwrap_or(0.0), z) - top;
            if px < 0.0 || py < 0.0 || px > w || py > h {
                continue;
            }
            pins.push_str(&format!("<div class=\"pin\" style=\"left:{:.0}px;top:{:.0}px\"></div>",
                                   px, py));
        }
        // the frame is drawn at print width and the mosaic is built at twice
        // it, then scaled — so the tiles land on paper at roughly 190dpi
        // instead of 96, which is the difference between readable street names
        // and a smear
        let scale = 0.5;
        format!(concat!("<div class=\"mapwrap\"><h2>where these came from</h2>",
                        "<div class=\"mapframe\" style=\"width:{:.0}px;height:{:.0}px\">",
                        "<div class=\"mapscale\" style=\"width:{:.0}px;height:{:.0}px;transform:scale({})\">",
                        "{}{}{}</div></div>",
                        "<p class=\"credit\">{} \u{00b7} {} post(s) with a place</p></div>"),
                w * scale, h * scale, w, h, scale,
                tiles, reports_map_overlay(z, left, top), pins,
                reports_map_credit(), points.len())
    }

    // base64, because a data: URI is how a picture gets into a file:// page
    // with no server behind it, and this node adds no crate to do it
    fn reports_b64(bytes: &Vec<u8>) -> String {
        let abc = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let tbl: Vec<char> = abc.chars().collect();
        let mut out = String::new();
        let mut i = 0usize;
        while i + 2 < bytes.len() {
            let n = ((bytes[i] as u32) << 16) | ((bytes[i + 1] as u32) << 8) | (bytes[i + 2] as u32);
            out.push(tbl[((n >> 18) & 63) as usize]);
            out.push(tbl[((n >> 12) & 63) as usize]);
            out.push(tbl[((n >> 6) & 63) as usize]);
            out.push(tbl[(n & 63) as usize]);
            i += 3;
        }
        let left = bytes.len() - i;
        if left == 1 {
            let n = (bytes[i] as u32) << 16;
            out.push(tbl[((n >> 18) & 63) as usize]);
            out.push(tbl[((n >> 12) & 63) as usize]);
            out.push('=');
            out.push('=');
        } else if left == 2 {
            let n = ((bytes[i] as u32) << 16) | ((bytes[i + 1] as u32) << 8);
            out.push(tbl[((n >> 18) & 63) as usize]);
            out.push(tbl[((n >> 12) & 63) as usize]);
            out.push(tbl[((n >> 6) & 63) as usize]);
            out.push('=');
        }
        out
    }

    // ---- the toolbar and the surface ---------------------------------------

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["reports"]["may"].as_bool().unwrap_or(false) {
            return prev;
        }
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({
                "id": "reports", "label": "reports", "icon": reports_page_svg() }));
        }
        list.to_string()
    }

    fn reports_set() -> Vec<serde_json::Value> {
        let list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "report" {
                continue;
            }
            if !c["from"].is_null() {
                continue;
            }
            out.push(c.clone());
        }
        out.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            b["created"].as_u64().unwrap_or(0).cmp(&a["created"].as_u64().unwrap_or(0))
        });
        out
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if open_tool_read() != "reports" {
            return base;
        }
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        // belt and braces: the toolbar only offers the glyph to support and
        // above, and the surface behind it draws nothing without the same word
        if !s["reports"]["may"].as_bool().unwrap_or(false) {
            return base;
        }
        let set = reports_set();
        let open = browse_open_read();
        if !open.is_empty() {
            for c in set.iter() {
                if c["id"].as_str().unwrap_or("") == open {
                    return format!("{}{}", base, card_page_html(c.to_string()));
                }
            }
            // gone, or not a report: the set is the honest fallback, silently
        }
        format!("{}<div class=\"rep-surface\">{}{}</div>",
                base, reports_make_html(&s), reports_list_html(&set))
    }

    // the make box: a name, a question, and one button that is dim until there
    // is a question to ask. No data-ev on the fields, so typing never repaints
    // #app out from under the caret (/cards' rule for every field it owns).
    fn reports_make_html(s: &serde_json::Value) -> String {
        if !s["reports"]["key"].as_bool().unwrap_or(false) {
            return String::from(concat!(
                "<div class=\"rep-make rep-nokey\">",
                "<div class=\"rep-said\">no API key on the server</div></div>"));
        }
        String::from(concat!(
            "<div class=\"rep-make\">",
            "<input class=\"rep-name\" type=\"text\" placeholder=\"what to call it\" autocomplete=\"off\">",
            "<input class=\"rep-ask\" type=\"text\" placeholder=\"what do you want to know\" autocomplete=\"off\">",
            "<div class=\"rep-go off\" data-rep=\"make\">make</div>",
            "</div>"))
    }

    fn reports_list_html(set: &Vec<serde_json::Value>) -> String {
        if set.is_empty() {
            return String::from("<div class=\"browse-empty\">no reports yet</div>");
        }
        let mut out = String::from("<div class=\"browse-list rep-list\">");
        for c in set.iter() {
            let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
            let st = reports_state_of(c);
            let word = reports_word(&st);
            let when = st["generated"].as_u64().unwrap_or(0);
            let stamp = if when == 0 {
                String::new()
            } else {
                reports_date(when)
            };
            out.push_str(&format!(
                "<div class=\"crow browse-row\" data-ev=\"browse_open:{}\"><span class=\"cnum rep-status {}\">{}</span><div class=\"ctext browse-title\">{}</div><span class=\"browse-when\">{}</span></div>",
                id, reports_word_class(&st), word,
                card_esc(reports_title_of(c)), stamp));
        }
        out.push_str("</div>");
        out
    }

    fn reports_word(st: &serde_json::Value) -> String {
        match st["status"].as_str().unwrap_or("new") {
            "working" => "working".to_string(),
            "ready" => "ready".to_string(),
            "failed" => "stuck".to_string(),
            "nokey" => "no key".to_string(),
            _ => "not run".to_string(),
        }
    }

    fn reports_word_class(st: &serde_json::Value) -> String {
        match st["status"].as_str().unwrap_or("new") {
            "ready" => "on".to_string(),
            "working" => "busy".to_string(),
            _ => String::new(),
        }
    }

    // ---- the report's own page ---------------------------------------------
    // /cards' page, spliced: the picture block a report has no use for is taken
    // out, the two placeholders say what these blocks are here for, and the
    // state and its two controls go INSIDE the page's box, which is fixed and
    // scrolls its contents — anything appended after it lands off-screen (/me's
    // and /projects' reason for the same move).

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if c["type"].as_str().unwrap_or("") != "report" {
            return html;
        }
        let html = html.replacen("<div class=\"card-page",
                                 "<div class=\"card-page report", 1);
        let html = reports_no_pic(html);
        let html = reports_no_place(html);
        let html = html.replace("data-ph=\"your name\"",
                                "data-ph=\"what to call it\"");
        let html = html.replace("data-ph=\"say what you are here to do\"",
                                "data-ph=\"what do you want to know\"");
        reports_inside(html, reports_state_html(&c))
    }

    // a report's picture block is always empty and always unused, so it comes
    // straight out. A block's text is escaped on the way in (/cards' card_esc),
    // so the first `</div>` after the opening tag is the block's own.
    fn reports_no_pic(html: String) -> String {
        let at = match html.find("<div class=\"card-pic") {
            Some(i) => i,
            None => return html,
        };
        let end = match html[at..].find("</div>") {
            Some(j) => at + j + 6,
            None => return html,
        };
        format!("{}{}", &html[..at], &html[end..])
    }

    // /location puts a **map location** pill on every card page, which is the
    // right offer on a post and a control about nothing on a report: a report
    // has no place, and one that acquired one would still print nothing.
    // Taken out here rather than guarded there — this node is newer, so its
    // link is outside /location's and sees the pill already spliced in
    // (/taste 7: a control saying the wrong thing). The pill carries an SVG
    // and no nested span, so the first `</span>` after it is its own.
    fn reports_no_place(html: String) -> String {
        let at = match html.find("<span class=\"card-place") {
            Some(i) => i,
            None => return html,
        };
        let end = match html[at..].find("</span>") {
            Some(j) => at + j + 7,
            None => return html,
        };
        format!("{}{}", &html[..at], &html[end..])
    }

    fn reports_inside(html: String, add: String) -> String {
        if add.is_empty() {
            return html;
        }
        match html.strip_suffix("</div>") {
            Some(h) => format!("{}{}</div>", h, add),
            None => format!("{}{}", html, add),
        }
    }

    fn reports_state_html(c: &serde_json::Value) -> String {
        let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
        let st = reports_state_of(c);
        let status = st["status"].as_str().unwrap_or("new").to_string();
        let note = card_esc(st["note"].as_str().unwrap_or("").to_string());
        let mut line = match status.as_str() {
            "working" => "working on it".to_string(),
            "ready" => {
                let n = st["n"].as_u64().unwrap_or(0);
                let through = st["through"].as_u64().unwrap_or(0);
                let when = st["generated"].as_u64().unwrap_or(0);
                let mut l = format!("{} \u{00b7} {} post(s)", reports_date(when), n);
                if through > 0 {
                    l.push_str(&format!(" \u{00b7} data through {}", reports_date(through)));
                }
                l
            }
            "failed" => "it did not work".to_string(),
            "nokey" => "no API key on the server".to_string(),
            _ => "not run yet".to_string(),
        };
        if !note.is_empty() && status != "ready" {
            line = note;
        }
        let mut row = String::new();
        if status == "ready" {
            // a plain link, deliberately: no data-ev, so the loop never takes
            // this tap, and a new tab is what puts the phone's own share sheet
            // over the PDF instead of trapping a standalone app inside it
            let slug = reports_slug(reports_title_of(c));
            row.push_str(&format!(
                "<a class=\"rep-btn rep-doc\" href=\"reports/{}.pdf?id={}\" target=\"_blank\" rel=\"noopener\">open</a>",
                card_esc(slug), card_esc(reports_urlesc(c["id"].as_str().unwrap_or("").to_string()))));
        }
        let go = if status == "new" { "make it" } else { "again" };
        row.push_str(&format!(
            "<div class=\"rep-btn rep-run\" data-rep=\"run\" data-id=\"{}\">{}</div>", id, go));
        format!("<div class=\"rep-state\"><div class=\"rep-said\">{}</div><div class=\"rep-row\">{}</div></div>",
                card_esc(line), row)
    }

    // the id goes in a query string, and a person's name can carry a space
    fn reports_urlesc(s: String) -> String {
        let mut out = String::new();
        for b in s.into_bytes() {
            let c = b as char;
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == '~' {
                out.push(c);
            } else {
                out.push_str(&format!("%{:02X}", b));
            }
        }
        out
    }

    // ---- the events --------------------------------------------------------

    fn update(state: String, event: String) -> String {
        let was_tool = open_tool_read();
        let was_open = browse_open_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("").to_string();
        // the server's answer to "may I", landed in the loop state — /invite's
        // InviteList exactly, and for the same reason: the rung lives behind
        // the cookie and never in anybody's world
        if kind == "ReportsMay" {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            s["reports"] = e["data"].clone();
            return s.to_string();
        }
        if kind == "ReportNew" {
            reports_new(e["data"].clone());
            return state;
        }
        if kind != "click" {
            return state;
        }
        // the tool's own button with a report open means "back to the set", one
        // level at a time (/tools' grammar): /tools has already closed the tool
        // and /browse has cleared the open card, so both are put back here
        if e["ev"].as_str().unwrap_or("") == "tool_reports"
            && was_tool == "reports" && !was_open.is_empty() {
            open_tool_write("reports".to_string());
        }
        state
    }

    // the card, made here rather than through /new's CardNew, because a report
    // is born with two blocks written and a state block appended — and because
    // the id is then `<owner>.<t>` for a `t` the page half chose, so it knows
    // what it just made and can ask the server to run it without waiting for a
    // paint to tell it.
    fn reports_new(d: serde_json::Value) {
        let owner = d["owner"].as_str().unwrap_or("").trim().to_string();
        let owner = if owner.is_empty() { "you".to_string() } else { owner };
        let title = d["title"].as_str().unwrap_or("").trim().to_string();
        let query = d["query"].as_str().unwrap_or("").trim().to_string();
        let now = d["t"].as_u64().unwrap_or(0);
        if now == 0 || query.is_empty() {
            return;
        }
        let mut card = card_new(owner, "report".to_string(), now);
        let name = if title.is_empty() { query.clone() } else { title };
        card["blocks"][0]["text"] = serde_json::json!(name);
        card["blocks"][2]["text"] = serde_json::json!(query);
        if let Some(arr) = card["blocks"].as_array_mut() {
            arr.push(serde_json::json!({ "kind": "report", "status": "new", "at": now }));
        }
        let id = card["id"].as_str().unwrap_or("").to_string();
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            list = serde_json::json!([]);
        }
        list.as_array_mut().expect("cards is an array").push(card);
        cards_write(list.to_string());
        browse_open_write(id);
    }

    // ---- the glyph ---------------------------------------------------------
    // drawn, in currentColor, per /glyphs: a sheet with a folded corner and two
    // lines of writing on it. Black on /ember's tint, white on the plain
    // button, and no filter working to correct an asset.

    fn reports_page_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "<path d=\"M14 3v5h5\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" ",
            "stroke-linejoin=\"round\"/>",
            "<path d=\"M9 13h6M9 17h4\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.2\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}

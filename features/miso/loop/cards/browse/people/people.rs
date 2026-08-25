struct feature_People;
impl feature_People {
    // ---- which cards, and which words -------------------------------------
    // /browse's two seams, both taken here. The surface, the picker, the two
    // renderers and the two device vars are all /browse's, unchanged.

    fn browse_cards(state: String) -> String {
        let all: serde_json::Value =
            serde_json::from_str(&existing.browse_cards(state.clone()))
                .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut profiles: Vec<serde_json::Value> = Vec::new();
        for c in all.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") == "profile" {
                profiles.push(c.clone());
            }
        }
        people_order(serde_json::Value::Array(profiles).to_string(), state)
    }

    // where /taste 6 puts the number: "profile" on every row of a surface
    // that is all profiles is noise, so how near they are goes there instead.
    fn browse_row_left(card: &serde_json::Value) -> String {
        people_word(card["near"].as_i64().unwrap_or(-1))
    }

    // ---- the order ---------------------------------------------------------
    // the chain the next proximity cue joins at: project membership (#p71's
    // "later") redefines this one function and mixes its own answer into the
    // distance, and the sort is not rewritten.
    //
    // self first, always — the card with no `from` on it, which is /exchange's
    // own test for "you wrote this". Then by (distance, owner name); a card
    // whose owner is not on the guest list has no distance and sorts last.
    fn people_order(cards: String, state: String) -> String {
        let list: serde_json::Value = serde_json::from_str(&cards)
            .unwrap_or(serde_json::json!([]));
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let near = s["near"].clone();
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            let mut c = c.clone();
            let owner = c["owner"].as_str().unwrap_or("").to_string();
            let d = if c["from"].is_null() {
                0i64
            } else {
                near[owner.as_str()].as_i64().unwrap_or(-1)
            };
            // the distance rides on the card so the row renderer, which is
            // /browse's and takes no state, can still say the word
            c["near"] = serde_json::json!(d);
            out.push(c);
        }
        out.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            let ra = people_rank(a["near"].as_i64().unwrap_or(-1));
            let rb = people_rank(b["near"].as_i64().unwrap_or(-1));
            let na = a["owner"].as_str().unwrap_or("").to_string();
            let nb = b["owner"].as_str().unwrap_or("").to_string();
            ra.cmp(&rb).then(na.cmp(&nb))
        });
        serde_json::Value::Array(out).to_string()
    }

    // unknown sorts last, not first: -1 would put every stranger above you.
    fn people_rank(d: i64) -> i64 {
        if d < 0 {
            i64::MAX
        } else {
            d
        }
    }

    // a distance is a number and does not know which way round the edge ran,
    // so "invited" / "invited by" would be guessing on half the rows.
    fn people_word(d: i64) -> String {
        if d < 0 {
            return String::new();
        }
        if d == 0 {
            return "you".to_string();
        }
        format!("{} away", d)
    }

    // ---- the toolbar -------------------------------------------------------
    // the cards tool goes: everything you hold is a person today, and 👤 is
    // where people live. /under-account's idiom — filter the registry chain.

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let kept: Vec<serde_json::Value> = list.as_array().unwrap_or(&empty).iter()
            .filter(|t| t["id"].as_str() != Some("cards"))
            .cloned().collect();
        serde_json::Value::Array(kept).to_string()
    }

    // ---- the surface -------------------------------------------------------

    fn render(state: String) -> String {
        if open_tool_read() != "account" {
            return existing.render(state);
        }
        let own = people_own_id();
        // before the first CardEnsure lands there is nothing to land ON: leave
        // the chain alone and /me's own "making your card…" line is the answer
        if own.is_empty() {
            return existing.render(state);
        }
        let open = browse_open_read();
        if open == own {
            // your own card is /me's page, drawn by /me, with /invite's rows
            // under it — nothing muted, nothing redrawn here
            return format!("{}{}", existing.render(state), browse_picker_html());
        }
        let base = existing.render(people_muted(state.clone()));
        let picker = browse_picker_html();
        let cards: serde_json::Value = serde_json::from_str(&browse_cards(state))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let set = cards.as_array().unwrap_or(&empty);
        if !open.is_empty() {
            for c in set.iter() {
                if c["id"].as_str().unwrap_or("") == open {
                    return format!("{}{}{}", base, picker,
                                   card_page_html(c.to_string()));
                }
            }
            // gone, or not a person: the set is the honest fallback, silently
        }
        format!("{}{}{}", base, picker, browse_set_html(set))
    }

    // the id of your own profile card. /exchange redefines card_of_type so an
    // ownerless ask skips the copies, which is exactly the question here.
    fn people_own_id() -> String {
        let own = card_of_type(cards_read(), String::new(), "profile".to_string());
        if own.is_empty() {
            return String::new();
        }
        let c: serde_json::Value = serde_json::from_str(&own)
            .unwrap_or(serde_json::Value::Null);
        c["id"].as_str().unwrap_or("").to_string()
    }

    // /me draws its page by reading open_tool out of the state STRING, so this
    // is how it is told not to be the landing surface. Every other render link
    // reads the open tool from the live context, so the mute reaches /me and
    // nothing else — see tool_controls below for the one exception.
    fn people_muted(state: String) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["open_tool"] = serde_json::json!("");
        s.to_string()
    }

    // the repair: /under-account decides whether to draw the invite plus by
    // reading open_tool out of the same string. render_toolbar is inside the
    // muted call, so the live value is put back before the chain beneath sees
    // it and the plus behaves exactly as it did.
    fn tool_controls(state: String) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("").is_empty() {
            s["open_tool"] = serde_json::json!(open_tool_read());
        }
        existing.tool_controls(s.to_string())
    }

    // ---- the events --------------------------------------------------------

    fn update(state: String, event: String) -> String {
        // read BEFORE the chain beneath runs: /tools closes the tool and
        // /browse clears `open` on a tool_ tap, so afterwards both are gone
        let was_tool = open_tool_read();
        let was_open = browse_open_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("");
        // the fetched distances, straight into the loop state under this
        // node's own key — /invite's reason: the guest list is the server's,
        // and syncing it to devices as world state would be a lie.
        if kind == "PeopleNear" {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            s["near"] = e["data"].clone();
            return s.to_string();
        }
        if kind != "click" {
            return state;
        }
        // 👤 with a card open means "back to the people", one level at a time
        // (/tools' own grammar, #p88) — the tool is put back, and /browse has
        // already cleared which card was open.
        if e["ev"].as_str().unwrap_or("") == "tool_account"
            && was_tool == "account" && !was_open.is_empty() {
            open_tool_write("account".to_string());
        }
        state
    }

    // ---- the invite tree ---------------------------------------------------
    // the graph lives in the guest list, which no device has. This route is
    // OUTSIDE /gate's wall (this node is the newest, so it is outermost on the
    // route chain), which makes the cookie check its own job.

    fn route(r: request) -> response {
        if r.path == "users/near" && r.method == "GET" {
            return people_near(r);
        }
        existing.route(r)
    }

    fn people_say(status: u16, words: String) -> response {
        json_response(status, format!("{{\"ok\":false,\"error\":\"{}\"}}",
                                      words.replace('"', "'")))
    }

    // the three-way read /invite established: a list, or null meaning
    // "something is wrong". Read here rather than borrowed so this node stands
    // without /invite composed — with no invited_by anywhere, everyone but you
    // is simply unknown.
    fn people_users() -> serde_json::Value {
        let raw = match std::fs::read_to_string(format!("{}/users.json", auth_dir())) {
            Ok(r) => r,
            Err(e) => {
                println!("people: the guest list can't be read: {}", e);
                return serde_json::Value::Null;
            }
        };
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                println!("people: the guest list is not valid JSON ({})", e);
                return serde_json::Value::Null;
            }
        };
        if v.is_array() {
            v
        } else {
            println!("people: the guest list is not a JSON array");
            serde_json::Value::Null
        }
    }

    // the store's health is asked BEFORE the cookie: with the list broken
    // nobody is authed (/harden re-checks it inside token_valid), so an
    // authority-first order would answer a broken box with the wrong sentence.
    fn people_near(r: request) -> response {
        let list = people_users();
        if list.is_null() {
            return people_say(500, "the guest list can't be read".to_string());
        }
        let t = cookie_token(r.cookie.clone());
        if t.is_empty() || !token_valid(t.clone()) {
            return people_say(403, "who are you?".to_string());
        }
        let me = format!("phone:{}", token_phone(t));
        json_response(200, format!("{{\"ok\":true,\"near\":{}}}",
                                   people_bfs(list, me)))
    }

    // breadth-first over the invite edges, in BOTH directions: /invite writes
    // invited_by, and being invited is as near as inviting. Keyed out by name,
    // because a card's `owner` is a name and that is the only thing the two
    // sides share — a world key is a phone number nobody should be handed.
    // Linear scan on purpose: a guest list is tens of rows, and a map would
    // cost a type the chain parser cannot read for nothing.
    fn people_bfs(list: serde_json::Value, me: String) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut keys: Vec<String> = Vec::new();
        let mut ups: Vec<String> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if p.is_empty() {
                continue;
            }
            keys.push(format!("phone:{}", p));
            ups.push(u["invited_by"].as_str().unwrap_or("").to_string());
            names.push(u["name"].as_str().unwrap_or("").to_string());
        }
        let n = keys.len();
        let mut dist: Vec<i64> = vec![-1; n];
        let mut frontier: Vec<usize> = Vec::new();
        for i in 0..n {
            if keys[i] == me {
                dist[i] = 0;
                frontier.push(i);
            }
        }
        let mut d = 0i64;
        while !frontier.is_empty() {
            let mut next: Vec<usize> = Vec::new();
            for at in frontier.iter() {
                let i = *at;
                for j in 0..n {
                    if dist[j] >= 0 {
                        continue;
                    }
                    if ups[i] == keys[j] || ups[j] == keys[i] {
                        dist[j] = d + 1;
                        next.push(j);
                    }
                }
            }
            frontier = next;
            d = d + 1;
        }
        let mut out = serde_json::Map::new();
        for i in 0..n {
            if dist[i] < 0 || names[i].is_empty() {
                continue;
            }
            out.insert(names[i].clone(), serde_json::json!(dist[i]));
        }
        serde_json::Value::Object(out).to_string()
    }
}

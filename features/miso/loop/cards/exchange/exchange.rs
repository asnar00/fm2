struct feature_Exchange;
impl feature_Exchange {
    // ---- the door, and why it is on `route` --------------------------------
    // handing a card over means writing into somebody else's world, and the
    // one thing that must not happen is doing it inside the SENDER's turn.
    // /edit freezes a view of the context when a request opens and every
    // `with_context` inside the turn reads that frozen view; `edit_context`
    // replays its closure against it. Switching this thread's identity halfway
    // through such a turn would read the wrong world and write the recipient's
    // list into the sender's frozen view.
    //
    // This node is the newest in the composition, so its `route` link is the
    // OUTERMOST one — outside /per-user's identity link and outside /edit's
    // turn boundary. Here there is no frozen view and no ambient identity, so
    // a world may be named, read and written honestly, and every read is of
    // the live value. That is why the fan-out watches `POST /msg` from out
    // here rather than extending `handle_msg` from within.
    //
    // Nothing in /context is touched: `context_user_set`, `handle_msg`,
    // `cards_read` and `card_of_type` are the four public seams used.

    // both watches live in this one function because `existing` may only be
    // called from the link that owns the chain: a cards write, whose effect is
    // read on both sides of the chain beneath, and a login, which is what
    // makes an invite an exchange (#p71).
    fn route(r: request) -> response {
        let cards_write = r.path == "msg" && r.method == "POST"
            && exchange_is_cards_op(&r.body);
        let verify = r.path == "auth/verify" && r.method == "POST";
        if !cards_write && !verify {
            return existing.route(r);
        }
        let who = exchange_who(&r);
        let body = r.body.clone();
        let watch = cards_write && !who.is_empty();
        let before = if watch {
            exchange_cards_of(who.clone())
        } else {
            String::new()
        };
        let resp = existing.route(r);
        if resp.status != 200 {
            return resp;
        }
        if watch {
            let after = exchange_cards_of(who.clone());
            if after != before {
                exchange_share(who, before, after);
            }
            return resp;
        }
        if verify {
            exchange_after_verify(body);
        }
        resp
    }

    // the inviter's cards go to the invitee the moment they first get in;
    // the invitee's come back on their first write, through the watch above.
    fn exchange_after_verify(body: String) {
        let v: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if !phone.is_empty() {
            exchange_seed(format!("phone:{}", phone));
        }
    }

    // the caller, off the cookie and nothing else: handing over a card is the
    // owner's own act, so the localhost tooling door has no say in who is
    // doing it.
    fn exchange_who(r: &request) -> String {
        let t = cookie_token(r.cookie.clone());
        if !t.is_empty() && token_valid(t.clone()) {
            return format!("phone:{}", token_phone(t));
        }
        String::new()
    }

    // is this message a write of the cards var? The body may have been
    // truncated at /messaging's cap, in which case it parses as nothing and
    // this is false — a message that will be refused anyway.
    fn exchange_is_cards_op(body: &String) -> bool {
        let m: serde_json::Value = serde_json::from_str(body)
            .unwrap_or(serde_json::Value::Null);
        m["type"].as_str().unwrap_or("") == "CtxOp"
            && m["data"]["path"].as_str().unwrap_or("") == "miso/loop/cards"
            && m["data"]["name"].as_str().unwrap_or("") == "cards"
    }

    // ---- who can see you --------------------------------------------------
    // rung one's whole answer, and it is the invite tree (#p71): the person
    // who invited you, and the people you invited. Shared membership of a
    // project is the second cue and is named for later.

    fn exchange_users() -> serde_json::Value {
        let raw = std::fs::read_to_string(format!("{}/users.json", auth_dir()))
            .unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        if v.is_array() {
            v
        } else {
            serde_json::json!([])
        }
    }

    // the guest list carries `invited_by` as a world key, written by /invite.
    // Without /invite composed the field is simply absent and this node is a
    // no-op — no dependency, and nothing to guard.
    fn exchange_links(key: String) -> Vec<String> {
        let list = exchange_users();
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<String> = Vec::new();
        let mut mine = String::new();
        for u in list.as_array().unwrap_or(&empty) {
            let k = exchange_key_of(u);
            if k == key {
                mine = u["invited_by"].as_str().unwrap_or("").to_string();
            }
        }
        if !mine.is_empty() && mine != key {
            out.push(mine);
        }
        for u in list.as_array().unwrap_or(&empty) {
            let k = exchange_key_of(u);
            if k.is_empty() || k == key {
                continue;
            }
            if u["invited_by"].as_str().unwrap_or("") != key {
                continue;
            }
            let mut seen = false;
            for o in out.iter() {
                if o == &k {
                    seen = true;
                }
            }
            if !seen {
                out.push(k);
            }
        }
        out
    }

    // one guest-list entry's world key, the shape /per-user gives a cookie
    fn exchange_key_of(u: &serde_json::Value) -> String {
        let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
        if p.is_empty() {
            String::new()
        } else {
            format!("phone:{}", p)
        }
    }

    fn exchange_name_of(key: String) -> String {
        match key.strip_prefix("phone:") {
            Some(p) => find_user(p.to_string()),
            None => String::new(),
        }
    }

    // whose pages hear a relayed edit: /messaging's own audience name for a
    // user, which is the last four digits of their number.
    fn exchange_audience_of(key: String) -> String {
        match key.strip_prefix("phone:") {
            Some(p) => tag(p.to_string()),
            None => String::new(),
        }
    }

    // ---- reading and writing a named world ---------------------------------

    // one world's cards, whoever this thread was otherwise acting as. Safe
    // only from outside a turn, which is where both callers are.
    fn exchange_cards_of(key: String) -> String {
        let saved = context_user_now();
        context_user_set(key);
        let list = cards_read();
        context_user_set(saved);
        list
    }

    // put cards into somebody else's world, through the same door a device's
    // own edit comes in by: a `CtxOp` on the cards var, handed to `handle_msg`
    // with this thread acting as the recipient. /guard merges it into what
    // they already hold — so nothing of theirs can be displaced — /converge
    // applies it and relays a CtxUpdate to their open pages, and /remember
    // logs it, so a phone that was off finds the card waiting when it joins.
    //
    // The op is signed with the RECIPIENT's audience, because it is their
    // world that changed and their other devices that need telling.
    fn exchange_give(to: String, cards: Vec<serde_json::Value>) {
        if to.is_empty() || cards.is_empty() {
            return;
        }
        let cards = exchange_not_theirs(to.clone(), cards);
        if cards.is_empty() {
            return;
        }
        let value = serde_json::Value::Array(cards).to_string();
        let msg = serde_json::json!({
            "type": "CtxOp",
            "_from": exchange_audience_of(to.clone()),
            "data": {
                "path": "miso/loop/cards",
                "name": "cards",
                "op": "set",
                "value": value
            }
        }).to_string();
        let saved = context_user_now();
        context_user_set(to.clone());
        let reply = handle_msg(msg);
        context_user_set(saved);
        let r: serde_json::Value = serde_json::from_str(&reply)
            .unwrap_or(serde_json::Value::Null);
        if r["type"].as_str().unwrap_or("") != "CtxUpdate" {
            println!("exchange: {} would not take a card ({})",
                     tag(to), r["error"].as_str().unwrap_or("no reason given"));
        }
    }

    // the last gate before another world is written, and the one that makes
    // this door safe to leave open: a card may not land on an id that world
    // already holds under a DIFFERENT owner.
    //
    // Without it, a linked user could mint a card carrying somebody else's id
    // — ids are `<owner>.<created>` and a device chooses its own — and hand it
    // over; /guard merges by id and takes the newer `edited`, so the forgery
    // would land ON TOP of the real card and the owner's own page would show
    // it. Rig-found, 2026-08-25, and it was a genuine loss: /guard cannot see
    // it, because at its level a card is an id and a timestamp.
    //
    // `exchange_owns_id` is the same rule stated on the way out: a card you
    // hand on must carry an id of your own minting.
    fn exchange_not_theirs(to: String, cards: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        let held: serde_json::Value = serde_json::from_str(&exchange_cards_of(to.clone()))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in cards.iter() {
            let id = c["id"].as_str().unwrap_or("");
            let owner = c["owner"].as_str().unwrap_or("");
            let mut clash = false;
            for h in held.as_array().unwrap_or(&empty) {
                if h["id"].as_str().unwrap_or("") == id
                    && h["owner"].as_str().unwrap_or("") != owner {
                    clash = true;
                }
            }
            if clash {
                println!("exchange: refused a card that would land on {}'s own {} — \
                          a different owner already holds that id", tag(to.clone()), id);
                continue;
            }
            out.push(c.clone());
        }
        out
    }

    // a card's id is `<owner>.<created ms>` (/cards). A card whose id was not
    // minted by its owner is not theirs to hand over, whatever it says.
    fn exchange_owns_id(card: &serde_json::Value, owner: String) -> bool {
        card["id"].as_str().unwrap_or("").starts_with(&format!("{}.", owner))
    }

    // THE copy path: everything that turns an owner's card into somebody
    // else's copy happens here, so a later way of handing a card over — a
    // send-to sheet, a project — marks its copies the same way by calling this
    // and `exchange_give` and writing no marking code of its own.
    //
    // `from` is the owner's name and the whole read-only test on the page.
    // `via` is the world key of the person this copy came through, which for
    // an invite link is the owner themselves; it is what a later surface
    // orders people by — proximity is "how did this reach me". `received` says
    // when it arrived. The id is the owner's already, so nothing can collide.
    fn exchange_copy(card: &serde_json::Value, from: String, via: String) -> serde_json::Value {
        let mut c = card.clone();
        c["from"] = serde_json::json!(from);
        c["via"] = serde_json::json!(via);
        c["received"] = serde_json::json!(now_ms());
        c
    }

    // ---- the two acts ------------------------------------------------------

    // a write happened: give everyone linked to the writer the cards of the
    // writer's that changed. A card that was not there before counts as
    // changed — the first write of one stamps `edited` equal to `created`, and
    // that first write is exactly the one that has to travel.
    fn exchange_share(who: String, before: String, after: String) {
        let to = exchange_links(who.clone());
        if to.is_empty() {
            return;
        }
        let me = exchange_name_of(who.clone());
        let old: serde_json::Value = serde_json::from_str(&before)
            .unwrap_or(serde_json::json!([]));
        let new: serde_json::Value = serde_json::from_str(&after)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut mine: Vec<serde_json::Value> = Vec::new();
        for c in new.as_array().unwrap_or(&empty) {
            if !c["from"].is_null() {
                continue;   // a copy is not yours to hand on
            }
            if !me.is_empty() && c["owner"].as_str().unwrap_or("") != me {
                continue;
            }
            if !exchange_owns_id(c, me.clone()) {
                continue;   // an id you did not mint is not a card you own
            }
            let id = c["id"].as_str().unwrap_or("");
            let mut seen = false;
            let mut was = 0u64;
            for o in old.as_array().unwrap_or(&empty) {
                if o["id"].as_str().unwrap_or("") == id {
                    seen = true;
                    was = o["edited"].as_u64().unwrap_or(0);
                }
            }
            if seen && c["edited"].as_u64().unwrap_or(0) == was {
                continue;
            }
            mine.push(exchange_copy(c, me.clone(), who.clone()));
        }
        if mine.is_empty() {
            return;
        }
        for k in to.iter() {
            exchange_give(k.clone(), mine.clone());
        }
        println!("exchange: {} handed {} card(s) to {} person(s)",
                 me, mine.len(), to.len());
    }

    // a login happened: if this person was invited, their inviter's cards go
    // to them now. Theirs come back on their first write.
    fn exchange_seed(key: String) {
        let list = exchange_users();
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut inviter = String::new();
        for u in list.as_array().unwrap_or(&empty) {
            if exchange_key_of(u) == key {
                inviter = u["invited_by"].as_str().unwrap_or("").to_string();
            }
        }
        if inviter.is_empty() || inviter == key {
            return;
        }
        let name = exchange_name_of(inviter.clone());
        let held: serde_json::Value =
            serde_json::from_str(&exchange_cards_of(inviter.clone()))
            .unwrap_or(serde_json::json!([]));
        let mut theirs: Vec<serde_json::Value> = Vec::new();
        for c in held.as_array().unwrap_or(&empty) {
            if !c["from"].is_null() {
                continue;
            }
            if !name.is_empty() && c["owner"].as_str().unwrap_or("") != name {
                continue;
            }
            if !exchange_owns_id(c, name.clone()) {
                continue;
            }
            theirs.push(exchange_copy(c, name.clone(), inviter.clone()));
        }
        if theirs.is_empty() {
            println!("exchange: {} has no card yet — {} gets it on the next edit",
                     name, tag(key));
            return;
        }
        println!("exchange: seeding {} with {} card(s) from {}",
                 tag(key.clone()), theirs.len(), name);
        exchange_give(key, theirs);
    }

    // ---- the page -----------------------------------------------------------
    // a copy carries `from`, and that is the whole read-only test: a card that
    // carries it is one you did not write. Deliberately not a comparison
    // against the logged-in name — two people may share a name, the name is
    // not in the world at all, and a page half that had to fetch it would race
    // the paint. Decided here, in the renderer, it is structural: the DOM the
    // editing listeners look for is simply not drawn.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let owner = card_esc(c["owner"].as_str().unwrap_or("").to_string());
        let from = c["from"].as_str().unwrap_or("").to_string();
        let mark = "<div class=\"card-page\"";
        let opened = if from.is_empty() {
            format!("<div class=\"card-page\" data-owner=\"{}\"", owner)
        } else {
            format!("<div class=\"card-page foreign\" data-owner=\"{}\"", owner)
        };
        let html = html.replacen(mark, &opened, 1);
        if from.is_empty() {
            return html;
        }
        // with no contenteditable anywhere in the page, /cards' focusout,
        // /keep's input timer and Enter rule, and /frame's chooser have
        // nothing to fire on.
        let html = html.replace(" contenteditable=\"true\"", "");
        let html = exchange_no_dim_place(html);
        let html = exchange_no_empty_pic(html);
        exchange_with_from(html, from)
    }

    // /location's DIM pill means "no place yet — ask the phone". On a card
    // that is not yours there is nothing to ask about, and its page half would
    // stamp somebody else's card with your position. Taken away rather than
    // dimmed further; with /location unticked the mark is not there and this
    // returns the page untouched.
    fn exchange_no_dim_place(html: String) -> String {
        let mark = "<span class=\"card-place dim\"";
        match html.find(mark) {
            Some(i) => match html[i..].find("</span>") {
                Some(j) => format!("{}{}", &html[..i], &html[i + j + 7..]),
                None => html,
            },
            None => html,
        }
    }

    // /cards draws an empty picture block as an invitation — "add a picture".
    // On a card that is not yours that is a control saying it does something
    // it will not do (/taste 7), so the empty block goes entirely: a card with
    // no picture should look like a card with no picture. A picture that IS
    // there stays, of course — that is the whole point of holding the card.
    fn exchange_no_empty_pic(html: String) -> String {
        let mark = "<div class=\"card-pic empty\"";
        match html.find(mark) {
            Some(i) => match html[i..].find("</div>") {
                Some(j) => format!("{}{}", &html[..i], &html[i + j + 6..]),
                None => html,
            },
            None => html,
        }
    }

    // the quiet line under the title: who handed you this
    fn exchange_with_from(html: String, from: String) -> String {
        let line = format!("<div class=\"card-from\">from {}</div>",
                           card_esc(from));
        let mark = "class=\"card-title\"";
        match html.find(mark) {
            Some(i) => match html[i..].find("</div>") {
                Some(j) => format!("{}{}{}",
                                   &html[..i + j + 6], line, &html[i + j + 6..]),
                None => format!("{}{}", html, line),
            },
            None => format!("{}{}", html, line),
        }
    }

    // /me asks /cards for "the profile card" with no owner at all, and its
    // comment said why it could: "a world holds only its owner's cards today;
    // exchange is what earns it". This node earns it. An ownerless ask now
    // skips the copies and answers "you hold none of your own" rather than
    // handing back a neighbour's; asked WITH an owner — which is what
    // CardEnsure does — it is /cards' own answer, untouched.
    fn card_of_type(list: String, owner: String, kind: String) -> String {
        if !owner.is_empty() {
            return existing.card_of_type(list, owner, kind);
        }
        let v: serde_json::Value = serde_json::from_str(&list)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != kind {
                continue;
            }
            if !c["from"].is_null() {
                continue;
            }
            return c.to_string();
        }
        String::new()
    }
}

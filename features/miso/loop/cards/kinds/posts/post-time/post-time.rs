struct feature_PostTime;
impl feature_PostTime {
    // ---- the datum ---------------------------------------------------------
    // `when` is the moment a post RECORDS, not the moment it was typed: the
    // photograph's own time if the file carried one, and otherwise nothing at
    // all — because a post made with no photo already has a moment it records,
    // and that moment is `created`. So absence is not a missing value; it is
    // the answer "the making was the moment", and every card written before
    // this node existed reads correctly without being touched.
    //
    // `when_from` says which it was, for a surface that later wants to show
    // the difference (and for the hand-set time that is parked, whose value
    // would be `hand`). Nothing draws it yet.

    fn post_time_of(card: &serde_json::Value) -> u64 {
        let w = card["when"].as_u64().unwrap_or(0);
        if w > 0 {
            return w;
        }
        card["created"].as_u64().unwrap_or(0)
    }

    // ---- the event ---------------------------------------------------------
    // CardWhen {id, when, source, t}: set one post's time. Read and written
    // through /cards' own cards_read / cards_write, so the var's address stays
    // in one place and cards.rs is never edited — /location's CardPlace is the
    // precedent, and this is the same shape with one field instead of a block.
    //
    // Only a card of type `post` takes one: a profile's date is when you last
    // edited it, which is what /browse already says, and a photograph on a
    // profile should not move the profile into last year.
    //
    // `edited` is stamped, as CardPlace stamps it, because /guard merges two
    // versions of a card by the newer `edited` — a change that did not touch
    // it could be merged away by a device holding the older card.
    //
    // The time is believed as it is given. A camera whose clock is wrong dates
    // its post wrong, and that is the honest answer: the alternative is an app
    // silently disagreeing with the timestamp the user can read on their own
    // photograph. Only a zero or an unreadable number is refused.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "CardWhen" {
            return state;
        }
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let when = e["data"]["when"].as_u64().unwrap_or(0);
        if id.is_empty() || when == 0 {
            return state;
        }
        let source = e["data"]["source"].as_str().unwrap_or("photo").to_string();
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return state;
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            if !posts_is(c) {
                continue;
            }
            c["when"] = serde_json::json!(when);
            c["when_from"] = serde_json::json!(source);
            if now > 0 {
                c["edited"] = serde_json::json!(now);
            }
            changed = true;
        }
        if changed {
            cards_write(list.to_string());
        }
        state
    }

    // ---- the order ---------------------------------------------------------
    // /posts' own set, resorted: newest first by the post's time rather than
    // by when the card was made, with the id still breaking the tie so every
    // device agrees. The chain beneath keeps deciding WHICH cards are in the
    // set; only their order is this node's.

    fn posts_set() -> Vec<serde_json::Value> {
        let mut out = existing.posts_set();
        out.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            let ta = post_time_of(a);
            let tb = post_time_of(b);
            tb.cmp(&ta).then(posts_id_of(b).cmp(&posts_id_of(a)))
        });
        out
    }

    // ---- the date on the row -----------------------------------------------
    // /browse's seam for which of a card's times its row shows. A post says
    // the time it records; everything else keeps /browse's `edited`. Keyed on
    // the card's type, not on which tool is open, so a post carries its own
    // date onto whatever surface draws it — the list, /portrait's row, and any
    // later view that asks the same seam.

    fn browse_when_of(card: &serde_json::Value) -> u64 {
        if !posts_is(card) {
            return existing.browse_when_of(card);
        }
        post_time_of(card)
    }
}

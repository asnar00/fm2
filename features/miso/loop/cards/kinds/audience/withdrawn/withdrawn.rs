struct feature_Withdrawn;
impl feature_Withdrawn {
    // a raise must withdraw. `exchange_give` is THE door into another world
    // and /audience's gate refuses a post whose floor stands above the
    // recipient's rank — but a refusal only declines to send. It cannot take
    // back the copy an earlier, lower floor already handed over, so an undone
    // promote left the post on the phone of everyone the promote had reached.
    //
    // This link runs outside that gate (newer, so outermost): for every card
    // the gate is about to refuse that the recipient ALREADY HOLDS, it hands
    // them a tombstone — /delete's shape, the only write /guard lets remove
    // anything — instead of nothing. The owner's own card is untouched.
    fn exchange_give(to: String, cards: Vec<serde_json::Value>) {
        if to.is_empty() || cards.is_empty() {
            return existing.exchange_give(to, cards);
        }
        let held: serde_json::Value = serde_json::from_str(&exchange_cards_of(to.clone()))
            .unwrap_or(serde_json::Value::Null);
        let mut out: Vec<serde_json::Value> = Vec::new();
        let mut stones: Vec<serde_json::Value> = Vec::new();
        for c in cards.iter() {
            let mine = withdrawn_held(&held, c);
            if audience_in_of(c).is_empty() || audience_may_hold(c, to.clone()) {
                // the gate lets this one through: if they are holding a
                // tombstone of it, this is the revive, and it only lands if it
                // is newer than the stone (/guard merges by `edited`, and
                // /revert's trick is the precedent for saying so).
                out.push(withdrawn_revive(c, &mine));
                continue;
            }
            out.push(c.clone());
            if mine.is_null() || delete_gone(&mine) {
                continue;   // never had it, or already gone: nothing to take back
            }
            println!("audience: withdrawing {} from {} — the floor is above them now",
                     c["id"].as_str().unwrap_or("?"), tag(to.clone()));
            stones.push(delete_tombstone(&mine, withdrawn_stamp(&mine, c)));
        }
        for s in stones.iter() {
            out.push(s.clone());
        }
        existing.exchange_give(to, out)
    }

    // the recipient's own copy of this card, or null. The id is the owner's
    // and a copy keeps it (/exchange's `exchange_copy`), so one field matches.
    fn withdrawn_held(held: &serde_json::Value, card: &serde_json::Value) -> serde_json::Value {
        let id = card["id"].as_str().unwrap_or("").to_string();
        if id.is_empty() {
            return serde_json::Value::Null;
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        for h in held.as_array().unwrap_or(&empty) {
            if h["id"].as_str().unwrap_or("") == id {
                return h.clone();
            }
        }
        serde_json::Value::Null
    }

    // a withdrawal is a new edit in time and must beat the copy it replaces.
    // The arriving card's own stamp normally already does — an undone promote
    // is restamped by /guard/revert — but the two can land in the same
    // millisecond, so the floor is one past what they hold.
    fn withdrawn_stamp(held: &serde_json::Value, card: &serde_json::Value) -> u64 {
        let h = held["edited"].as_u64().unwrap_or(0);
        let c = card["edited"].as_u64().unwrap_or(0);
        if c > h { c } else { h + 1 }
    }

    // the way back. A live copy replaces a tombstone whole (/guard keeps the
    // newer `edited` and takes the card entire, `deleted` and all), so the
    // revive needs nothing but a stamp that wins — and a promote made in the
    // same millisecond as the withdrawal it undoes would otherwise tie and
    // lose. Only a card the recipient holds as a tombstone is touched.
    fn withdrawn_revive(card: &serde_json::Value, held: &serde_json::Value) -> serde_json::Value {
        if held.is_null() || !delete_gone(held) {
            return card.clone();
        }
        let h = held["edited"].as_u64().unwrap_or(0);
        if card["edited"].as_u64().unwrap_or(0) > h {
            return card.clone();
        }
        let mut out = card.clone();
        out["edited"] = serde_json::json!(h + 1);
        println!("audience: reviving {} at {} — a withdrawn copy is theirs again",
                 card["id"].as_str().unwrap_or("?"), h + 1);
        out
    }
}

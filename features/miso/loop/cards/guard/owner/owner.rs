struct feature_Owner;
impl feature_Owner {
    // an id keeps its owner: an incoming card whose id the world already
    // holds under a different owner is a forgery or a collision, and is
    // dropped before the merge sees it. /guard merged by id and stamp alone,
    // which would let any path that can write a list change who a card
    // belongs to (found by /exchange's red-team, 2026-08-25).
    fn cards_guard_merge(current: serde_json::Value, incoming: serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        let cur = current.as_array().unwrap_or(&empty).clone();
        let kept: Vec<serde_json::Value> = incoming.as_array().unwrap_or(&empty).iter()
            .filter(|i| !cards_owner_changed(&cur, i))
            .cloned().collect();
        existing.cards_guard_merge(current, serde_json::Value::Array(kept))
    }

    fn cards_owner_changed(cur: &Vec<serde_json::Value>, card: &serde_json::Value) -> bool {
        let id = card["id"].as_str().unwrap_or("");
        for c in cur {
            if c["id"].as_str().unwrap_or("") == id && c["owner"] != card["owner"] {
                println!("cards: dropped a write that would change the owner of {}", id);
                return true;
            }
        }
        false
    }
}

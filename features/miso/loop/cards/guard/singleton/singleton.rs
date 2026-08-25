struct feature_Singleton;
impl feature_Singleton {
    // only a profile is one-per-owner. /guard drops a BLANK card arriving
    // for an owner who already holds one of its type — right for the ensure
    // that makes a profile, wrong for every other kind, where a new card is
    // blank at the instant it is made (found by /posts: the second `new`
    // silently vanished). A type is a singleton only if it says so here.
    fn cards_guard_has_type(cur: &Vec<serde_json::Value>, card: &serde_json::Value) -> bool {
        if !cards_type_is_singleton(card["type"].as_str().unwrap_or("").to_string()) {
            return false;
        }
        existing.cards_guard_has_type(cur, card)
    }

    // the seam: which types are one-per-owner. Profile, and nothing else
    // until a type asks.
    fn cards_type_is_singleton(kind: String) -> bool {
        kind == "profile"
    }
}

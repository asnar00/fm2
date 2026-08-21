struct feature_Review;
impl feature_Review {
    // the one OK: stamp the accepted build on the user; the declared merge
    // ships it to every instance, whose page halves apply the build on
    // arrival. `js:update_accepted` republishes it at the key both
    // review.index.js and /consent-once already read.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "AcceptUpdate" {
            return state;
        }
        let build = e["data"]["build"].as_i64().unwrap_or(0);
        if build <= 0 {
            return state;
        }
        let prev: i64 = update_accepted_read().parse().unwrap_or(0);
        if build > prev {
            update_accepted_write(build.to_string());
        }
        state
    }

    // the address, written once. The closure clones because `edit_context`
    // replays it against this turn's frozen view and therefore runs it twice.
    fn update_accepted_read() -> String {
        with_context(|c| c.review_update_accepted_get())
    }

    fn update_accepted_write(build: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/update/policy/review", "update_accepted",
                              serde_json::json!(build.clone()));
        });
    }
}

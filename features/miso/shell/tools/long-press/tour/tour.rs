struct feature_Tour;
impl feature_Tour {
    // the end of the tour, as a user-scoped var: the page sends TourSeen
    // when the tour finishes or is skipped, and the declared merge carries
    // it to the person's other devices. /policy's idiom.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "TourSeen" {
            return state;
        }
        tour_seen_write(true);
        state
    }

    // the address, written once. The closure clones nothing because a bool
    // is Copy; edit_context still runs it twice (live, then the frozen view).
    fn tour_seen_write(seen: bool) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/tools/long-press/tour", "tour_seen",
                              serde_json::json!(seen));
        });
    }
}

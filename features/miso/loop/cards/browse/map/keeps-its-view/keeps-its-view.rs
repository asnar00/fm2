struct feature_KeepsItsView;
impl feature_KeepsItsView {
    // the page half is the only half that knows where Leaflet is looking, so
    // the view arrives as an event the way a finger's does — never as a write
    // to a bridged key from a node newer than /payload (misses.md,
    // "navigation from the wrong side").
    //
    // An unchanged view is not rewritten: `moveend` fires for a programmatic
    // setView as well as for a hand, and the restore this node performs would
    // otherwise queue a write for the value it just read.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "MapView" {
            return state;
        }
        let v = e["data"]["v"].as_str().unwrap_or("").to_string();
        if v.is_empty() || v == keeps_its_view_read() {
            return state;
        }
        keeps_its_view_write(v);
        state
    }

    fn keeps_its_view_read() -> String {
        with_context(|c| c.keeps_its_view_map_view_get())
    }

    fn keeps_its_view_write(v: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/browse/map/keeps-its-view",
                              "map_view", serde_json::json!(v.clone()));
        });
    }
}

struct feature_WorldCache;
impl feature_WorldCache {
    // fm:context-snapshot — the linker's hook this node reads through: the
    // generated walker over every declared var is what the device caches, so a
    // var added anywhere in the composition is cached without a line here.
    // (`alive` and `parity` ask for it too; the token is a presence test.)
    //
    // the world rides the PAYLOAD, not the state. `event_payload` is the one
    // place every turn's answer is wrapped — boot's and every event's — so one
    // link catches both. Putting it in the state instead would send the whole
    // var table back INTO the wasm on the next event, doubling the crossing
    // for a value nothing in Rust reads.
    fn event_payload(state: String, html: String) -> String {
        let mut p: serde_json::Value =
            serde_json::from_str(&existing.event_payload(state, html))
                .unwrap_or(serde_json::Value::Null);
        if p.is_object() {
            p["world"] = world_cache_records();
        }
        p.to_string()
    }

    // what is worth writing down, in the shape the hydrate reads back.
    //
    // A record is the var's OWN value, not its resolved one — the same thing
    // an op carries — so hydrating is assignment and cannot invent a value the
    // device never held.
    //
    // Two kinds say nothing. An ABSENT var has never been touched here, and a
    // hydrated record would set its presence bit and stop it inheriting from
    // the layer ever after; silence is what keeps a fresh var resolving like a
    // fresh var. A GLOBAL var's authority is the layer, and this device's own
    // field for it is unread ballast — caching it would be caching nobody's
    // value. (`present` is `overlay`'s column; with that node unticked there
    // are no presence bits and no layer, and every var is cached.)
    //
    // DEVICE scope, which a join skips, is cached: this is the one store that
    // is allowed to know it, and after a reload the cache is the ONLY place a
    // device-scoped value can come back from.
    fn world_cache_records() -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        let mine = with_context(|c| c.snapshot());
        for v in mine.as_array().unwrap_or(&empty) {
            if v["scope"].as_str().unwrap_or("") == "global" {
                continue;
            }
            if !v["present"].as_bool().unwrap_or(true) {
                continue;
            }
            out.push(serde_json::json!({
                "path": v["path"].clone(),
                "name": v["name"].clone(),
                "value": v["value"].clone(),
            }));
        }
        serde_json::Value::Array(out)
    }

    // the arriving half, and the ONE link in this file whose position matters:
    // the records are applied BEFORE the chain beneath it runs, because
    // `payload`'s republish is beneath it. Applied after, every value would be
    // true in the context and absent from the state key the fragments read, so
    // the first paint would show the empty world it is here to prevent.
    //
    // Assignment through `set_from_json` is the same door a join uses: it
    // queues no op, so a hydrate puts nothing on the wire and cannot echo, and
    // it is idempotent, so hydrating twice is hydrating once. A record the
    // composition no longer declares is refused by the setter and skipped —
    // which is what makes the cache survive an update that dropped a var.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") == "WorldHydrate" {
            let empty: Vec<serde_json::Value> = Vec::new();
            for rec in e["data"]["ctx"].as_array().unwrap_or(&empty) {
                let path = rec["path"].as_str().unwrap_or("").to_string();
                let name = rec["name"].as_str().unwrap_or("").to_string();
                let value = rec["value"].clone();
                let _ = edit_context(|c| c.set_from_json(&path, &name, value.clone()));
            }
        }
        existing.update(state, event)
    }
}

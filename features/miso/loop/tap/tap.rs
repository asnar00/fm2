struct feature_Tap;
impl feature_Tap {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap" {
            return state;
        }
        tap_count_bump();
        state
    }

    // ---- the counter seam -------------------------------------------------
    // the count lives in the /context now, and WHICH context var it lives in is
    // a composition choice: this node declares a device-scoped counter, and
    // /sync redefines these three functions to reach a global-scoped one
    // instead. SyncVar chose its scope at the call site, which a declaration
    // cannot do, so the choice moves to a seam three functions wide — and
    // unticking /sync means what it always meant.

    fn tap_count_read() -> u64 {
        with_context(|c| c.tap_tap_count_get().sum)
    }

    fn tap_count_bump() {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/tap", "tap_count", serde_json::json!(1));
        });
    }

    fn tap_count_reset(n: u64) {
        edit_context(|c| {
            let _ = c.edit_reset("miso/loop/tap", "tap_count", serde_json::json!(n));
        });
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        // launcher-aware: when /tools owns the screen (key present), the
        // counter only renders as the open "taps" tool; with no launcher the
        // key is absent and the counter renders as it always did
        if s["open_tool"].is_string() && s["open_tool"].as_str() != Some("taps") {
            return base;
        }
        let count = tap_count_read();
        let label = if count == 0 {
            "tap".to_string()
        } else {
            format!("taps: {}", count)
        };
        format!("{}<div class=\"tap\" data-ev=\"tap\">{}</div>", base, label)
    }
}

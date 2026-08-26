struct feature_Reorder;
impl feature_Reorder {
    // ---- the store -------------------------------------------------------
    // the row's order is a declared /var: a JSON list of tool ids in a string,
    // user-scoped and last-write, so the arrangement is the person's and not
    // the phone's. The address is written once here.

    fn tool_order_read() -> String {
        with_context(|c| c.reorder_tool_order_get())
    }

    fn tool_order_write(order: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/tools/long-press/reorder", "tool_order",
                              serde_json::json!(order.clone()));
        });
    }

    // ---- the seam --------------------------------------------------------
    // /tools asks this before any feature imposes a default order: once this
    // person has arranged the row, theirs is the order. It is a seam rather
    // than a chain position because provenance can put a default-order link
    // either side of this one — /lead's arrived minutes after this ask.

    fn tools_order_chosen() -> bool {
        let order: serde_json::Value = serde_json::from_str(&tool_order_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        !order.as_array().unwrap_or(&empty).is_empty()
    }

    // ---- the sort --------------------------------------------------------
    // outermost on the registry chain, so every tool has registered by the
    // time this runs: the ids the order names come first, in its order; every
    // other tool follows in registration order. A tool that arrives after the
    // drag still shows up, at the end, and a tool the order names but the
    // composition no longer has simply falls out.

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let order: serde_json::Value = serde_json::from_str(&tool_order_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let ids: Vec<String> = order.as_array().unwrap_or(&empty).iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if ids.is_empty() {
            return prev;
        }
        let list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        let items: Vec<serde_json::Value> = list.as_array().unwrap_or(&empty).clone();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for id in ids.iter() {
            for t in items.iter() {
                if t["id"].as_str().unwrap_or("") == id.as_str() {
                    out.push(t.clone());
                }
            }
        }
        for t in items.iter() {
            let id = t["id"].as_str().unwrap_or("").to_string();
            if !ids.contains(&id) {
                out.push(t.clone());
            }
        }
        serde_json::Value::Array(out).to_string()
    }

    // ---- the drop --------------------------------------------------------
    // the page half sends the whole row it just arranged; this writes it. An
    // empty or non-list payload is ignored rather than wiping the order —
    // a dropped drag must never cost someone their arrangement.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "ToolOrder" {
            return state;
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let ids: Vec<serde_json::Value> = e["data"]["order"].as_array().unwrap_or(&empty)
            .iter()
            .filter(|v| v.as_str().is_some())
            .cloned()
            .collect();
        if ids.is_empty() {
            return state;
        }
        tool_order_write(serde_json::Value::Array(ids).to_string());
        state
    }
}

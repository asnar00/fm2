struct feature_InviteTool;
impl feature_InviteTool {
    // the toolbar button: a person silhouette (the + badge is CSS), shown
    // only once the server has said this user may invite — a member never
    // sees the tool at all
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["invite"]["may"].as_bool().unwrap_or(false) {
            return prev;
        }
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({ "id": "invite", "label": "invite", "icon": "👤" }));
        }
        list.to_string()
    }

    // the rows leave the card: nothing sits under it now
    fn me_under(state: String) -> String {
        let _ = state;
        String::new()
    }

    // the tool's page: the invite rows on a card-shaped ground of their own
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        // the live context for which tool is open; `s["invite"]` below stays on
        // the state, because that is the page half's own answer and nothing in
        // the chain rewrites it. The mirror is one turn stale after any tap
        // that closes a tool and re-opens it at a link newer than /payload,
        // and this is a renderer (/browse: a renderer may not read a stale
        // value). No way in has been found that leaves the invite page blank
        // today — /tools opens this tool directly and nothing writes it back —
        // so this is the shape being made safe, not a fault being repaired.
        if open_tool_read() != "invite" {
            return base;
        }
        format!("{}<div class=\"card-page invite-page\">{}</div>",
                base, invite_rows_html(s["invite"].clone()))
    }
}

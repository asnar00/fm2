struct feature_CurrentOnly;
impl feature_CurrentOnly {
    // inside a nested toolset the row shows that toolset's icon, not its
    // parent's. A parent is a registry tool (tools_list) other than the open
    // one; its button reaches the row as a `tool_<id>` control (/under-account
    // put 👤 into the invite row that way). Only a nested tool — one the
    // registry does not list — has a parent to drop; a registry tool's own
    // page is left exactly as it was, plus, bin and all.
    fn tool_controls(state: String) -> String {
        let html = existing.tool_controls(state.clone());
        let open = open_tool_read();
        if open.is_empty() {
            return html;
        }
        let list: serde_json::Value = serde_json::from_str(&tools_list(state))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let ids: Vec<String> = list.as_array().unwrap_or(&empty).iter()
            .filter_map(|t| t["id"].as_str().map(|s| s.to_string()))
            .collect();
        if ids.contains(&open) {
            return html;
        }
        let mut out = html;
        for id in ids.iter() {
            out = current_only_strip(out, format!("data-ev=\"tool_{}\"", id));
        }
        out
    }

    // remove every element carrying the marker, one at a time: the opening
    // <div before it and the first </div> after it (the buttons hold a span,
    // never a nested div) — /plus-at-home's cut, applied until none is left.
    fn current_only_strip(html: String, marker: String) -> String {
        let mut out = html;
        loop {
            let at = match out.find(marker.as_str()) {
                Some(at) => at,
                None => return out,
            };
            let cut = match (out[..at].rfind("<div"), out[at..].find("</div>")) {
                (Some(start), Some(rel)) => (start, at + rel + 6),
                _ => return out,
            };
            out = format!("{}{}", &out[..cut.0], &out[cut.1..]);
        }
    }
}

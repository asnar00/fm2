struct feature_Tools;
impl feature_Tools {
    // the registry chain: each tool redefines this to append {id, label}.
    fn tools_list(state: String) -> String {
        let _ = state;
        "[]".to_string()
    }

    // launcher-mode marker: the key existing (even empty) means the launcher
    // owns the screen; if this feature is unticked the key never appears and
    // tools render unconditionally as before.
    fn init() -> String {
        let state = existing.init();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        Var::<String>::local("open_tool").put(&mut s, &String::new());
        s.to_string()
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if ev == "tools_home" {
            Var::<String>::local("open_tool").put(&mut s, &String::new());
            return s.to_string();
        }
        if let Some(id) = ev.strip_prefix("tool_") {
            Var::<String>::local("open_tool").put(&mut s, &id.to_string());
            return s.to_string();
        }
        state
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let open = Var::<String>::local("open_tool").get(&s);
        if !open.is_empty() {
            return format!("{}<div class=\"home-chip\" data-ev=\"tools_home\">‹ tools</div>", base);
        }
        format!("{}{}", base, render_grid(state))
    }

    fn render_grid(state: String) -> String {
        let list: serde_json::Value = serde_json::from_str(&tools_list(state))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut grid = String::from("<div class=\"tool-grid\">");
        for t in list.as_array().unwrap_or(&empty) {
            let id = t["id"].as_str().unwrap_or("");
            let label = t["label"].as_str().unwrap_or(id);
            grid.push_str(&format!(
                "<div class=\"tool-button\" data-ev=\"tool_{}\">{}</div>", id, label));
        }
        grid.push_str("</div>");
        grid
    }
}

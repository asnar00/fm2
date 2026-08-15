struct feature_Tools;
impl feature_Tools {
    // the registry chain: each tool redefines this to append {id, label, icon}.
    fn tools_list(state: String) -> String {
        let _ = state;
        "[]".to_string()
    }

    // the controls chain: the open tool redefines this to put its own
    // buttons into the toolbar, right of its icon.
    fn tool_controls(state: String) -> String {
        let _ = state;
        String::new()
    }

    // the colour seam: what colour a tool's button wears (empty = the base
    // monochrome discipline). A styling feature redefines this per tool id.
    fn tool_colour(id: String) -> String {
        let _ = id;
        String::new()
    }

    // launcher-mode marker: the key existing (even empty) means the toolbar
    // owns navigation; if this feature is unticked the key never appears and
    // tools render unconditionally as before.
    fn init() -> String {
        let state = existing.init();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        Var::<String>::local("open_tool").put(&mut s, &String::new());
        // the composed catalog as data: which tools exist here, whatever the
        // toolbar happens to be showing (the page reads this — the DOM only
        // renders the open tool's button in open mode)
        let catalog = tools_list(s.to_string());
        Var::<String>::local("tools_catalog").put(&mut s, &catalog);
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
            // tapping the open tool's own button returns home (#p88 — no
            // separate back button); tapping any other tool opens it
            let open = Var::<String>::local("open_tool").get(&s);
            let next = if open == id { String::new() } else { id.to_string() };
            Var::<String>::local("open_tool").put(&mut s, &next);
            return s.to_string();
        }
        state
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        format!("{}{}", base, render_toolbar(state))
    }

    fn render_toolbar(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let open = Var::<String>::local("open_tool").get(&s);
        let list: serde_json::Value = serde_json::from_str(&tools_list(state.clone()))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut bar = String::from("<div class=\"toolbar\">");
        for t in list.as_array().unwrap_or(&empty) {
            let id = t["id"].as_str().unwrap_or("");
            // open mode: only the open tool's icon remains, leftmost after
            // the back chevron; the others have slid away
            if !open.is_empty() && open != id {
                continue;
            }
            let icon = t["icon"].as_str().unwrap_or("·");
            let label = t["label"].as_str().unwrap_or(id);
            let sel = if open == id { " sel" } else { "" };
            let colour = tool_colour(id.to_string());
            let tint = if colour.is_empty() {
                String::new()
            } else {
                format!(" tinted\" style=\"--tool-colour:{}", colour)
            };
            bar.push_str(&format!(
                "<div class=\"tool-button{}{}\" data-ev=\"tool_{}\" title=\"{}\"><span class=\"icon\">{}</span></div>",
                sel, tint, id, label, icon));
        }
        if !open.is_empty() {
            bar.push_str(&tool_controls(state));
        }
        bar.push_str("</div>");
        bar
    }
}

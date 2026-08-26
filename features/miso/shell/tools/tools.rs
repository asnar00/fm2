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

    // the order seam: has this person arranged the row themselves? The base
    // answer is no, so whatever default order the composition produces stands.
    // A feature that lets someone order the row redefines this, and a feature
    // that imposes a default order asks before imposing it — which is how a
    // chosen order beats a default whichever way round provenance puts them.
    fn tools_order_chosen() -> bool {
        false
    }

    // the colour seam: what colour a tool's button wears (empty = the base
    // monochrome discipline). A styling feature redefines this per tool id.
    fn tool_colour(id: String) -> String {
        let _ = id;
        String::new()
    }

    // launcher-mode marker: the key existing (even empty) means the toolbar
    // owns navigation; if this feature is unticked the key never appears and
    // tools render unconditionally as before. Both keys are `js:` columns on
    // this node's declarations now, so the key exists exactly when this node
    // is composed — which is what the marker always meant.
    fn init() -> String {
        let state = existing.init();
        open_tool_write(String::new());
        // the composed catalog as data: which tools exist here, whatever the
        // toolbar happens to be showing (the page reads this — the DOM only
        // renders the open tool's button in open mode)
        let catalog = tools_list(state.clone());
        tools_catalog_write(catalog);
        state
    }

    // ---- the navigation seam ----------------------------------------------
    // which tool is open lives in the /context now: a DEVICE-scoped var, which
    // is the declaration the old `local` scope always meant — navigation
    // is per-instance and never travels. The declared scope is what stops the
    // op: `set_at` queues nothing when the scope tag is "device", so opening a
    // tool puts nothing on the wire.
    //
    // the closure handed to edit_context runs TWICE (once against the live
    // world, once replayed against this turn's frozen view, so a later link in
    // the same turn reads what this one wrote), so it clones rather than moves.

    fn open_tool_read() -> String {
        with_context(|c| c.tools_open_tool_get())
    }

    fn open_tool_write(id: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/tools", "open_tool",
                              serde_json::json!(id.clone()));
        });
    }

    fn tools_catalog_write(catalog: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/tools", "tools_catalog",
                              serde_json::json!(catalog.clone()));
        });
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        if ev == "tools_home" {
            open_tool_write(String::new());
            return state;
        }
        if let Some(id) = ev.strip_prefix("tool_") {
            // tapping the open tool's own button returns home (#p88 — no
            // separate back button); tapping any other tool opens it
            let open = open_tool_read();
            let next = if open == id { String::new() } else { id.to_string() };
            open_tool_write(next);
            return state;
        }
        state
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        format!("{}{}", base, render_toolbar(state))
    }

    fn render_toolbar(state: String) -> String {
        let open = open_tool_read();
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

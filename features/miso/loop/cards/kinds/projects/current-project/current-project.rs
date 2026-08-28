struct feature_CurrentProject;
impl feature_CurrentProject {
    // ---- the var -------------------------------------------------------------
    fn current_project_read() -> String {
        with_context(|c| c.current_project_current_get())
    }

    fn current_project_write(id: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/kinds/projects/current-project",
                              "current", serde_json::json!(id));
        });
    }

    // the chosen project as a card — held, a project, not a tombstone. Null
    // means "none": a project you no longer hold cannot hide the world.
    fn current_project_card() -> serde_json::Value {
        let id = current_project_read();
        if id.is_empty() {
            return serde_json::Value::Null;
        }
        let v: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") == id
                && c["type"].as_str().unwrap_or("") == "project"
                && !delete_gone(c) {
                return c.clone();
            }
        }
        serde_json::Value::Null
    }

    // ---- the one test --------------------------------------------------------
    // related: written by somebody in the project (its owner, or a role), or
    // filed in it — an `in` link, the shape /cards reserved and nothing writes
    // yet. Honoured now so the day a post is filed the filter already knows.
    fn current_project_related(card: &serde_json::Value, proj: &serde_json::Value) -> bool {
        let pid = proj["id"].as_str().unwrap_or("");
        let empty: Vec<serde_json::Value> = Vec::new();
        for l in card["links"].as_array().unwrap_or(&empty) {
            if l["kind"].as_str().unwrap_or("") == "in"
                && l["to"].as_str().unwrap_or("") == pid {
                return true;
            }
        }
        let owner = card["owner"].as_str().unwrap_or("");
        if owner.is_empty() {
            return false;
        }
        if proj["owner"].as_str().unwrap_or("") == owner {
            return true;
        }
        for l in projects_members(proj).iter() {
            if projects_link_name(l) == owner || l["name"].as_str().unwrap_or("") == owner {
                return true;
            }
        }
        false
    }

    // ---- the two sifts -------------------------------------------------------
    // posts: the tool, its list and the map draw from this one set.
    fn posts_set() -> Vec<serde_json::Value> {
        let all = existing.posts_set();
        let proj = current_project_card();
        if proj.is_null() {
            return all;
        }
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in all.iter() {
            if current_project_related(c, &proj) {
                out.push(c.clone());
            }
        }
        out
    }

    // people: only under 👤, and never dropping your own card — a list of
    // people that does not contain you is somebody else's list. Every other
    // tool's set passes through: you must see the other projects to switch.
    fn browse_cards(state: String) -> String {
        let list = existing.browse_cards(state);
        if open_tool_read() != "account" {
            return list;
        }
        let proj = current_project_card();
        if proj.is_null() {
            return list;
        }
        let v: serde_json::Value = serde_json::from_str(&list)
            .unwrap_or(serde_json::Value::Null);
        if !v.is_array() {
            return list;
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["from"].is_null() || current_project_related(c, &proj) {
                out.push(c.clone());
            }
        }
        serde_json::Value::Array(out).to_string()
    }

    // ---- the ring ------------------------------------------------------------
    // on any project's page, own or copy: being in it is what makes it yours
    // to work in. Before /undo, through /projects' inserter; filled and in the
    // tool's colour when this is the chosen one, a plain ring otherwise.
    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state);
        if open_tool_read() != "projects" {
            return row;
        }
        let open = browse_open_read();
        if open.is_empty() || !current_project_is_project(open.clone()) {
            return row;
        }
        let on = current_project_read() == open;
        projects_before_undo(row, current_project_button(on))
    }

    fn current_project_is_project(id: String) -> bool {
        let v: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") == id {
                return c["type"].as_str().unwrap_or("") == "project" && !delete_gone(c);
            }
        }
        false
    }

    fn current_project_button(on: bool) -> String {
        let colour = tool_colour("projects".to_string());
        let tint = if on && !colour.is_empty() {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        } else {
            String::new()
        };
        let title = if on { "selected — tap to unselect" } else { "select this project" };
        format!("<div class=\"tool-button ctrl proj-select{}\" data-ev=\"proj_select\" data-on=\"{}\" title=\"{}\">{}</div>",
                tint, if on { "1" } else { "0" }, title, current_project_ring_svg(on))
    }

    // drawn, in currentColor (/glyphs): a ring, and a filled centre when chosen.
    fn current_project_ring_svg(on: bool) -> String {
        let dot = if on {
            "<circle cx=\"12\" cy=\"12\" r=\"4.2\" fill=\"currentColor\"/>"
        } else {
            ""
        };
        format!(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"12\" r=\"8.2\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\"/>",
            "{}</svg>"), dot)
    }

    // ---- the chip ------------------------------------------------------------
    // the sign on every card surface's picker: which project you are in, in
    // the one colour that means chosen. Tapping it is the same toggle.
    fn browse_picker_html() -> String {
        let pill = existing.browse_picker_html();
        let proj = current_project_card();
        if proj.is_null() {
            return pill;
        }
        let title = card_esc(browse_title_of(&proj));
        let title = if title.is_empty() { "a project".to_string() } else { title };
        let chip = format!("<div class=\"proj-chip\" data-ev=\"proj_select:{}\" title=\"tap to leave\">in <b>{}</b></div>",
                           card_esc(proj["id"].as_str().unwrap_or("").to_string()), title);
        match pill.strip_suffix("</div>") {
            Some(p) => format!("{}{}</div>", p, chip),
            None => format!("{}{}", pill, chip),
        }
    }

    // ---- the event -----------------------------------------------------------
    // proj_select toggles on the open project; proj_select:<id> (the chip)
    // toggles on that id — choosing what is chosen unchooses it.
    fn update(state: String, event: String) -> String {
        let was_open = browse_open_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let target = if ev == "proj_select" {
            was_open
        } else if let Some(id) = ev.strip_prefix("proj_select:") {
            id.to_string()
        } else {
            return state;
        };
        if target.is_empty() {
            return state;
        }
        if current_project_read() == target {
            current_project_write(String::new());
        } else {
            current_project_write(target);
        }
        state
    }
}

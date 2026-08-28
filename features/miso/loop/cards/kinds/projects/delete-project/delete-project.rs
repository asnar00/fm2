struct feature_DeleteProject;
impl feature_DeleteProject {
    // ---- the tombstone keeps its audience ----------------------------------
    // /delete empties a tombstone's links with its words. A project's role
    // links are not content: they are the people the card was handed to, and
    // /projects' hand-over and its exchange_give filter both read them. With
    // them, the tombstone travels to every member by the ordinary path and
    // /guard takes it over their copy; without them it reaches nobody.

    fn delete_tombstone(card: &serde_json::Value, now: u64) -> serde_json::Value {
        let mut out = existing.delete_tombstone(card, now);
        if card["type"].as_str().unwrap_or("") == "project" {
            out["links"] = serde_json::Value::Array(delete_project_roles(card));
        }
        out
    }

    fn delete_project_roles(card: &serde_json::Value) -> Vec<serde_json::Value> {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for l in card["links"].as_array().unwrap_or(&empty) {
            if l["kind"].as_str().unwrap_or("") == "role" {
                out.push(l.clone());
            }
        }
        out
    }

    // a deleted project's roles are not roles: off every profile page.
    fn projects_roles_from() -> Vec<serde_json::Value> {
        let mut out: Vec<serde_json::Value> = Vec::new();
        for p in existing.projects_roles_from().iter() {
            if !delete_gone(p) {
                out.push(p.clone());
            }
        }
        out
    }

    // ---- the control -------------------------------------------------------
    // on a project of your own, and nowhere else: in front of /undo's button
    // through /projects' own inserter, in the projects tool's colour.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state);
        if open_tool_read() != "projects" {
            return row;
        }
        let open = browse_open_read();
        if open.is_empty() || !delete_project_own(open) {
            return row;
        }
        projects_before_undo(row, delete_project_button())
    }

    fn delete_project_own(id: String) -> bool {
        let v: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            return c["type"].as_str().unwrap_or("") == "project"
                && c["from"].is_null() && !delete_gone(c);
        }
        false
    }

    fn delete_project_button() -> String {
        let colour = tool_colour("projects".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"projects_delete\" title=\"delete\">{}</div>",
                tint, delete_bin_svg())
    }
}

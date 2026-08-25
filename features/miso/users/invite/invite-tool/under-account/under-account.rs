struct feature_UnderAccount;
impl feature_UnderAccount {
    // no launcher button: the invite tool is reached from inside 👤
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let kept: Vec<serde_json::Value> = list.as_array().unwrap_or(&empty).iter()
            .filter(|t| t["id"].as_str() != Some("invite"))
            .cloned().collect();
        serde_json::Value::Array(kept).to_string()
    }

    // the sub-tool in 👤's control row, and the way back from its page:
    // with 👤 open, a person-with-a-plus opens the invite page; with the
    // invite page open, 👤 leads back to the card and the plus shows selected
    fn tool_controls(state: String) -> String {
        let mut html = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let open = s["open_tool"].as_str().unwrap_or("");
        let may = s["invite"]["may"].as_bool().unwrap_or(false);
        let mut mine = String::new();
        if open == "account" && may {
            mine.push_str(&invite_sub_button(false));
        }
        if open == "invite" {
            mine.push_str("<div class=\"tool-button ctrl\" data-ev=\"tool_account\" title=\"account\"><span class=\"icon\">👤</span></div>");
            mine.push_str(&invite_sub_button(true));
        }
        before_undo(html, mine)
    }

    // /undo's button is the last in every control row — an invariant every
    // later node must keep, since provenance puts newer links after it. So
    // anything this node adds goes in front of the undo button, not after.
    fn before_undo(row: String, add: String) -> String {
        if add.is_empty() {
            return row;
        }
        match row.find("data-ev=\"ctx_undo\"") {
            Some(at) => match row[..at].rfind("<div") {
                Some(start) => format!("{}{}{}", &row[..start], add, &row[start..]),
                None => format!("{}{}", row, add),
            },
            None => format!("{}{}", row, add),
        }
    }

    fn invite_sub_button(sel: bool) -> String {
        let s = if sel { " sel" } else { "" };
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"tool_invite\" title=\"invite\"><span class=\"icon\">👤</span></div>", s)
    }
}

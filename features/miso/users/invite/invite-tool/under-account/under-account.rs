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
        if open == "account" && may {
            html.push_str(&invite_sub_button(false));
        }
        if open == "invite" {
            html.push_str("<div class=\"tool-button ctrl\" data-ev=\"tool_account\" title=\"account\"><span class=\"icon\">👤</span></div>");
            html.push_str(&invite_sub_button(true));
        }
        html
    }

    fn invite_sub_button(sel: bool) -> String {
        let s = if sel { " sel" } else { "" };
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"tool_invite\" title=\"invite\"><span class=\"icon\">👤</span></div>", s)
    }
}

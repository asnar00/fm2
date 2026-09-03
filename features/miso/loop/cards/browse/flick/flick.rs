struct feature_Flick;
impl feature_Flick {
    // next or previous card in the list the open tool draws
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let step: i64 = if ev == "browse_next" { 1 } else if ev == "browse_prev" { -1 } else { return state; };
        let open = browse_open_read();
        if open.is_empty() {
            return state;
        }
        let list: Vec<serde_json::Value> = if open_tool_read() == "posts" {
            posts_set()
        } else {
            let v: serde_json::Value = serde_json::from_str(&browse_cards(state.clone()))
                .unwrap_or(serde_json::json!([]));
            v.as_array().cloned().unwrap_or_default()
        };
        let mut at: i64 = -1;
        let mut i: i64 = 0;
        for c in list.iter() {
            if c["id"].as_str().unwrap_or("") == open {
                at = i;
            }
            i = i + 1;
        }
        if at < 0 {
            return state;
        }
        let to = at + step;
        if to < 0 || to >= list.len() as i64 {
            return state;
        }
        let id = list[to as usize]["id"].as_str().unwrap_or("").to_string();
        if !id.is_empty() {
            browse_open_write(id);
        }
        state
    }
}

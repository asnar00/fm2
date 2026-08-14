struct feature_Dictate;
impl feature_Dictate {
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({ "id": "dictate", "label": "dictate", "icon": "🎤" }));
        }
        list.to_string()
    }

    // intent lives in state; the page half watches it and drives the mic
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let t = e["type"].as_str().unwrap_or("").to_string();
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if t == "click" && ev == "dict_rec" {
            s["dict_recording"] = serde_json::json!(true);
            return s.to_string();
        }
        if t == "click" && ev == "dict_stop" {
            s["dict_recording"] = serde_json::json!(false);
            return s.to_string();
        }
        if t == "RecSaved" {
            if !s["dict_files"].is_array() {
                s["dict_files"] = serde_json::json!([]);
            }
            s["dict_files"].as_array_mut().expect("dict_files is array")
                .push(e["data"].clone());
            return s.to_string();
        }
        if t == "RecList" {
            s["dict_files"] = e["data"]["items"].clone();
            return s.to_string();
        }
        state
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "dictate" {
            return base;
        }
        format!("{}{}{}", base, render_files(state.clone()), render_control(state))
    }

    fn render_files(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let files = s["dict_files"].as_array().unwrap_or(&empty);
        let mut grid = String::from("<div class=\"file-grid\">");
        for f in files {
            let label = f["label"].as_str().unwrap_or("note");
            grid.push_str(&format!(
                "<div class=\"file-icon\"><span class=\"icon\">🎤</span><div class=\"file-label\">{}</div></div>",
                label));
        }
        grid.push_str("</div>");
        grid
    }

    fn render_control(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["dict_recording"].as_bool().unwrap_or(false) {
            "<div class=\"rec-control recording\" data-ev=\"dict_stop\"><span class=\"rec-dot\"></span>stop</div>".to_string()
        } else {
            "<div class=\"rec-control\" data-ev=\"dict_rec\">record</div>".to_string()
        }
    }
}

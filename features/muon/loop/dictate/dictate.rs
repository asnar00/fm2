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
            return transcribe(s.to_string());
        }
        if t == "RecList" {
            s["dict_files"] = e["data"]["items"].clone();
            return transcribe(s.to_string());
        }
        if t == "Transcribed" {
            let id = e["data"]["id"].as_str().unwrap_or("").to_string();
            if let Some(files) = s["dict_files"].as_array_mut() {
                for f in files {
                    if f["id"].as_str().unwrap_or("") == id {
                        f["transcript"] = e["data"]["text"].clone();
                        f["t_rung"] = e["data"]["rung"].clone();
                        f["t_grade"] = e["data"]["grade"].clone();
                    }
                }
            }
            return transcribe(s.to_string());
        }
        if t == "click" {
            if let Some(id) = ev.strip_prefix("dict_play_") {
                let current = s["dict_playing"].as_str().unwrap_or("").to_string();
                s["dict_playing"] = if current == id {
                    serde_json::json!("")   // tapping the playing note stops it
                } else {
                    serde_json::json!(id)
                };
                return s.to_string();
            }
        }
        if t == "PlayEnded" {
            s["dict_playing"] = serde_json::json!("");
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
        format!("{}{}", base, render_files(state))
    }

    // dictate's toolbar controls: record when idle, stop (with the pulsing
    // dot) while recording — they sit in the toolbar, right of the mic.
    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "dictate" {
            return prev;
        }
        if s["dict_recording"].as_bool().unwrap_or(false) {
            format!("{}<div class=\"tool-button ctrl recording\" data-ev=\"dict_stop\">■<span class=\"rec-dot\"></span></div>", prev)
        } else {
            format!("{}<div class=\"tool-button ctrl\" data-ev=\"dict_rec\">●</div>", prev)
        }
    }

    fn render_files(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let files = s["dict_files"].as_array().unwrap_or(&empty);
        let playing = s["dict_playing"].as_str().unwrap_or("").to_string();
        let mut grid = String::from("<div class=\"file-grid\">");
        for f in files {
            let id = f["id"].as_str().unwrap_or("");
            let label = f["label"].as_str().unwrap_or("note");
            let cls = if playing == id { " playing" } else { "" };
            grid.push_str(&format!(
                "<div class=\"file-icon{}\" data-ev=\"dict_play_{}\"><span class=\"icon\">🔊</span><div class=\"file-label\">{}</div>{}</div>",
                cls, id, label, dict_file_extra(f.to_string())));
        }
        grid.push_str("</div>");
        grid
    }

    // ---- transcription: the graded-derivation slots (fm-spec-2 #p39) ----
    // each slot is one rung's reachability: the base returns "" (unreachable);
    // a rung subfeature redefines its slot to return "ready".
    fn transcribe_local(state: String) -> String {
        let _ = state;
        String::new()
    }
    fn transcribe_server(state: String) -> String {
        let _ = state;
        String::new()
    }
    fn transcribe_api(state: String) -> String {
        let _ = state;
        String::new()
    }

    // the scheduler: queue the best reachable rung for the first recording
    // that needs it. Intent lives in state (dict_transcribe = {id, rung,
    // grade}); the rung's page half does the work and reports a Transcribed
    // event; results are stamped with their grade and upgraded in place when
    // a better rung comes into reach. No rung reachable = no intent — with
    // every rung unticked dictate behaves exactly as before.
    fn transcribe(state: String) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let mut best_rung = String::new();
        let mut best_grade: i64 = 0;
        if !transcribe_local(state.clone()).is_empty() {
            best_rung = "local".to_string();
            best_grade = 1;
        }
        if !transcribe_server(state.clone()).is_empty() {
            best_rung = "server".to_string();
            best_grade = 2;
        }
        if !transcribe_api(state.clone()).is_empty() {
            best_rung = "api".to_string();
            best_grade = 3;
        }
        if let Some(o) = s.as_object_mut() {
            o.remove("dict_transcribe");
        }
        if best_grade == 0 {
            return s.to_string();
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut queued = serde_json::Value::Null;
        for f in s["dict_files"].as_array().unwrap_or(&empty) {
            let here = f["here"].as_bool() != Some(false);
            let done = f["t_grade"].as_i64().unwrap_or(0) >= best_grade;
            if here && !done {
                queued = serde_json::json!({ "id": f["id"], "rung": best_rung, "grade": best_grade });
                break;
            }
        }
        if !queued.is_null() {
            s["dict_transcribe"] = queued;
        }
        s.to_string()
    }

    // per-tile extras seam: every grid (this one and /mirror's replacement)
    // routes through this. The base shows a recording's transcript when it
    // has one, stamped with the rung that made it — inert until a rung exists.
    fn dict_file_extra(file: String) -> String {
        let f: serde_json::Value = serde_json::from_str(&file)
            .unwrap_or(serde_json::json!({}));
        let text = f["transcript"].as_str().unwrap_or("");
        if text.is_empty() {
            return String::new();
        }
        let rung = f["t_rung"].as_str().unwrap_or("");
        let safe = text.chars().take(160).collect::<String>()
            .replace('&', "&amp;").replace('<', "&lt;");
        format!("<div class=\"file-text\" title=\"{} transcript\">{}</div>", rung, safe)
    }

}

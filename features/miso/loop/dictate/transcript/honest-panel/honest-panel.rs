struct feature_Honest_Panel;
impl feature_Honest_Panel {
    // keep the failure reason /dictate's stamping drops: the phone's own
    // screen becomes the diagnostic readout for its own engine
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "Transcribed" {
            return state;
        }
        if e["data"]["failed"].as_bool() != Some(true) {
            return state;
        }
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if let Some(files) = s["dict_files"].as_array_mut() {
            for f in files {
                if f["id"].as_str().unwrap_or("") == id {
                    f["t_err"] = e["data"]["err"].clone();
                }
            }
        }
        s.to_string()
    }

    // the states the parent's panel doesn't cover: queued-behind and failed
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "dictate" {
            return base;
        }
        let playing = s["dict_playing"].as_str().unwrap_or("").to_string();
        if playing.is_empty() {
            return base;
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let files = s["dict_files"].as_array().unwrap_or(&empty);
        let mut ahead: i64 = 0;
        let mut target: Option<&serde_json::Value> = None;
        for f in files {
            let id = f["id"].as_str().unwrap_or("");
            if id == playing {
                target = Some(f);
                break;
            }
            let here = f["here"].as_bool() != Some(false);
            if here && f["t_grade"].as_i64().unwrap_or(0) < 1 {
                ahead += 1;
            }
        }
        let f = match target {
            Some(f) => f,
            None => return base,
        };
        if !f["transcript"].as_str().unwrap_or("").is_empty() {
            return base; // the parent showed the text
        }
        if s["dict_transcribe"]["id"].as_str().unwrap_or("") == playing {
            return base; // the parent said "transcribing…"
        }
        let err = f["t_err"].as_str().unwrap_or("");
        let body = if !err.is_empty() || f["t_grade"].as_i64().unwrap_or(0) >= 1 {
            let safe = err.replace('&', "&amp;").replace('<', "&lt;");
            format!("<div class=\"transcript-stamp\">transcription failed</div><div class=\"transcript-text\">{}</div>", safe)
        } else {
            format!("<div class=\"transcript-stamp\">waiting to transcribe ({} ahead)</div>", ahead)
        };
        format!("{}<div class=\"transcript-panel\">{}</div>", base, body)
    }
}

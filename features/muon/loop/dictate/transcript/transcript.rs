struct feature_Transcript;
impl feature_Transcript {
    // the reading view rides the render chain: playback state decides
    // everything, so there are no new events and no page half.
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
        let mut text = String::new();
        let mut rung = String::new();
        for f in s["dict_files"].as_array().unwrap_or(&empty) {
            if f["id"].as_str().unwrap_or("") == playing {
                text = f["transcript"].as_str().unwrap_or("").to_string();
                rung = f["t_rung"].as_str().unwrap_or("").to_string();
            }
        }
        if text.is_empty() {
            let queued = s["dict_transcribe"]["id"].as_str().unwrap_or("");
            if queued == playing {
                return format!("{}<div class=\"transcript-panel\"><div class=\"transcript-stamp\">transcribing…</div></div>", base);
            }
            return base;
        }
        let safe = text.replace('&', "&amp;").replace('<', "&lt;");
        let stamp = if rung.is_empty() { String::from("transcript") } else { format!("{} draft", rung) };
        format!("{}<div class=\"transcript-panel\"><div class=\"transcript-stamp\">{}</div><div class=\"transcript-text\">{}</div></div>", base, stamp, safe)
    }
}

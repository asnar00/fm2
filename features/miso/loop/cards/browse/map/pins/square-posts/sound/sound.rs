struct feature_Sound;
impl feature_Sound {
    fn map_surface_html(cards: &Vec<serde_json::Value>) -> String {
        let html = existing.map_surface_html(cards);
        let head = "data-pins=\"";
        let start = match html.find(head) {
            Some(i) => i + head.len(),
            None => return html,
        };
        let end = match html[start..].find('"') {
            Some(i) => start + i,
            None => return html,
        };
        let json = square_posts_unesc(html[start..end].to_string());
        let mut rows: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(_) => return html,
        };
        let n = match rows.as_array() {
            Some(a) => a.len(),
            None => return html,
        };
        for i in 0..n {
            let id = rows[i]["id"].as_str().unwrap_or("").to_string();
            for c in cards.iter() {
                if c["id"].as_str().unwrap_or("") == id && sound_only(c) {
                    rows[i]["sound"] = serde_json::Value::Bool(true);
                }
            }
        }
        format!("{}{}{}", &html[..start], card_esc(rows.to_string()), &html[end..])
    }

    // a recording and nothing to look at
    fn sound_only(card: &serde_json::Value) -> bool {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut audio = false;
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            let kind = b["kind"].as_str().unwrap_or("");
            if kind == "audio" {
                audio = true;
            }
            if kind == "video" {
                return false;
            }
            if kind == "picture" && !b["data"].as_str().unwrap_or("").is_empty() {
                return false;
            }
        }
        audio
    }
}

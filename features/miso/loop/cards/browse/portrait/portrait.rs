struct feature_Portrait;
impl feature_Portrait {
    // a picture-led row: the face on the left (the tile's face — picture or
    // initial), the name and an excerpt of the card's words on the right,
    // with the row's left-cell word and the date kept small beside the name
    fn browse_list_html(cards: &Vec<serde_json::Value>) -> String {
        let mut newest = 0u64;
        for c in cards.iter() {
            let t = browse_when_of(c);
            if t > newest {
                newest = t;
            }
        }
        let this_year = browse_year(newest);
        let mut out = String::from("<div class=\"browse-list portrait\">");
        for c in cards.iter() {
            let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
            let word = browse_row_left(c);
            let title = browse_title_of(c);
            let when = browse_when(browse_when_of(c), this_year);
            let face = portrait_face(c);
            let excerpt = portrait_excerpt(c);
            out.push_str(&format!(
                "<div class=\"crow browse-row portrait-row\" data-ev=\"browse_open:{}\">{}<div class=\"portrait-body\"><div class=\"portrait-line\"><span class=\"portrait-title\">{}</span><span class=\"portrait-word\">{}</span><span class=\"browse-when\">{}</span></div><div class=\"portrait-excerpt\">{}</div></div></div>",
                id, face, title, word, when, excerpt));
        }
        out.push_str("</div>");
        out
    }

    // the face: the first picture block, else the title's initial, dimmed
    fn portrait_face(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "picture" {
                let data = card_esc(b["data"].as_str().unwrap_or("").to_string());
                if !data.is_empty() {
                    return format!("<div class=\"portrait-face\"><img src=\"{}\" alt=\"\"></div>", data);
                }
            }
        }
        let initial: String = browse_title_of(card).chars().take(1).collect();
        format!("<div class=\"portrait-face empty\">{}</div>", initial)
    }

    // the excerpt: the first text block with words, whitespace folded, cut
    // at a word boundary near 80 characters with an ellipsis
    fn portrait_excerpt(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") != "text" {
                continue;
            }
            let words: Vec<&str> = b["text"].as_str().unwrap_or("").split_whitespace().collect();
            if words.is_empty() {
                continue;
            }
            let mut out = String::new();
            for w in words {
                if out.chars().count() + w.chars().count() + 1 > 80 {
                    out.push_str("…");
                    break;
                }
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(w);
            }
            return card_esc(out);
        }
        String::new()
    }
}

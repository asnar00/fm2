struct feature_Titled;
impl feature_Titled {
    // a post has a title after all, like a person and a project: the block
    // stays on the page (empty until given), and where a title exists it
    // captions the tile and leads the row; without one the author still does
    fn posts_no_title(html: String) -> String {
        html.replace("data-ph=\"your name\"", "data-ph=\"a title\"")
    }

    fn browse_title_of(card: &serde_json::Value) -> String {
        let own = titled_title(card);
        if !own.is_empty() && posts_is(card) {
            return own;
        }
        existing.browse_title_of(card)
    }

    fn card_tile_html(card: String) -> String {
        let html = existing.card_tile_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let own = titled_title(&c);
        if own.is_empty() || !posts_is(&c) {
            return html;
        }
        match (html.find("<div class=\"card-tile-title\">"), html.rfind("</div></div>")) {
            (Some(a), Some(b)) if b > a => {
                let open = "<div class=\"card-tile-title\">";
                format!("{}{}{}", &html[..a + open.len()], card_esc(own), &html[b..])
            }
            _ => html,
        }
    }

    fn titled_title(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "title" {
                return b["text"].as_str().unwrap_or("").trim().to_string();
            }
        }
        String::new()
    }
}

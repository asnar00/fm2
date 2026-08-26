struct feature_TileWords;
impl feature_TileWords {
    // a post's tile is captioned with a bit of what it says, not who said it:
    // the words are the post, and the author is one tap away on the page.
    // /portrait's excerpt is the one rule for "a bit of the words".
    fn card_tile_html(card: String) -> String {
        let html = existing.card_tile_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !posts_is(&c) {
            return html;
        }
        let words = portrait_excerpt(&c);
        if words.is_empty() {
            return html;
        }
        match (html.find("<div class=\"card-tile-title\">"), html.rfind("</div></div>")) {
            (Some(a), Some(b)) if b > a => {
                let open = "<div class=\"card-tile-title\">";
                format!("{}{}{}", &html[..a + open.len()], words, &html[b..])
            }
            _ => html,
        }
    }
}

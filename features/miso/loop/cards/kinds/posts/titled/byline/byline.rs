struct feature_Byline;
impl feature_Byline {
    // who and when, under the title of a post; the page marked as a post
    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !posts_is(&c) {
            return html;
        }
        let who = card_esc(c["owner"].as_str().unwrap_or("").to_string());
        let ms = post_time_of(&c);
        let when = browse_when(ms, browse_year(ms));
        let words = if when.is_empty() { who } else { format!("{} · {}", who, when) };
        let line = format!("<div class=\"post-byline\">{}</div>", words);
        let mark = "class=\"card-title\"";
        match html.find(mark) {
            Some(i) => match html[i..].find("</div>") {
                Some(j) => format!("{}{}{}", &html[..i + j + 6], line, &html[i + j + 6..]),
                None => format!("{}{}", html, line),
            },
            None => match html.find('>') {
                Some(i) => format!("{}{}{}", &html[..i + 1], line, &html[i + 1..]),
                None => html,
            },
        }
    }
}

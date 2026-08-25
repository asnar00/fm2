struct feature_Tag;
impl feature_Tag {
    // a card's type, as a small rounded tag in the page's top-right corner.
    // Inserted after the page's opening div so the ground carries it.
    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let kind = c["type"].as_str().unwrap_or("").to_string();
        if kind.is_empty() {
            return html;
        }
        let tag = format!(
            "<span class=\"card-tag\" style=\"background:{}\">{}</span>",
            card_tag_colour(kind.clone()), card_esc(kind));
        match html.find('>') {
            Some(i) => format!("{}{}{}", &html[..i + 1], tag, &html[i + 1..]),
            None => html,
        }
    }

    // one colour per type name, the same on every device: a fixed palette of
    // dusty tones (/taste 3 — desaturated, never neon) chosen by a hash of
    // the name, so a new type gets a colour without anyone assigning one
    fn card_tag_colour(kind: String) -> String {
        let palette = ["#9db7d8", "#d9c9a4", "#a4d9b8", "#d9a4a4", "#c2a4d9", "#a4d0d9", "#d9b8a4", "#b8d9a4"];
        let mut h: u32 = 5381;
        for b in kind.bytes() {
            h = h.wrapping_mul(33) ^ (b as u32);
        }
        palette[(h % palette.len() as u32) as usize].to_string()
    }
}

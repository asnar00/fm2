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
            card_tag_colour(kind.clone()), card_esc(card_tag_word(&c)));
        match html.find('>') {
            Some(i) => format!("{}{}{}", &html[..i + 1], tag, &html[i + 1..]),
            None => html,
        }
    }

    // the WORD the tag shows. An /extension point/: the default is the card's
    // own type, which is what this node was written for and what every card
    // still wears, and a node with a truer word for a kind of card redefines
    // it. Opened for /role-in-the-tag, whose people wear their role in the
    // project instead — with the default unchanged, so this node alone draws
    // exactly what it drew before.
    //
    // The COLOUR is still taken from the type, deliberately: the word varies
    // and the kind does not, so every person's tag stays one colour and the
    // tag reads as "the same kind of card, differently labelled".
    fn card_tag_word(card: &serde_json::Value) -> String {
        card["type"].as_str().unwrap_or("").to_string()
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

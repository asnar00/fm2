struct feature_OneMedium;
impl feature_OneMedium {
    // ---- one piece of media per page ---------------------------------------
    // /cards mints every card with an empty `picture` block and draws it as
    // the dashed "add a picture" invitation; /as-posts appends a recording
    // after it. So a post made by recording offered a second medium it should
    // never have offered (#p16a). The invitation goes when a medium is already
    // there — and since the dashed block IS the add road (/cards' delegated
    // click opens the chooser on `.card-pic`), taking it out of the page takes
    // the road with it. A rendering rule: no block moves, no index changes.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if one_medium_carried(&c).is_empty() {
            return html;
        }
        one_medium_no_empty_pic(html)
    }

    // which medium this card already carries — the one test the rule turns on,
    // and the extension point a fourth medium (or the swap that replaces one
    // with another) grows from. It answers WHICH rather than whether, because
    // a swap has to know what it is replacing. The empty picture slot is not a
    // medium; a filled one is.
    fn one_medium_carried(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            let kind = b["kind"].as_str().unwrap_or("");
            if kind == "audio" || kind == "video" {
                return kind.to_string();
            }
            if kind == "picture" && !b["data"].as_str().unwrap_or("").is_empty() {
                return "picture".to_string();
            }
        }
        String::new()
    }

    // the dashed block out of the drawn page. A block's text is escaped on the
    // way in (/cards' `card_esc`), so the first `</div>` after the opening tag
    // is its own. The same cut /exchange makes for a card that is not yours,
    // for its own reason; whichever runs first, the second finds nothing and
    // returns the page untouched.
    fn one_medium_no_empty_pic(html: String) -> String {
        let mark = "<div class=\"card-pic empty\"";
        match html.find(mark) {
            Some(i) => match html[i..].find("</div>") {
                Some(j) => format!("{}{}", &html[..i], &html[i + j + 6..]),
                None => html,
            },
            None => html,
        }
    }
}

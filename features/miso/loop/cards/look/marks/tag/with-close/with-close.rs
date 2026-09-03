struct feature_WithClose;
impl feature_WithClose {
    // the card's own close, sending what ‹ sends
    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card);
        let close = concat!(
            "<div class=\"card-close\" data-ev=\"tools_home\" title=\"close\">",
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M7 7l10 10M17 7L7 17\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>",
            "</svg></div>");
        match html.find('>') {
            Some(i) => format!("{}{}{}", &html[..i + 1], close, &html[i + 1..]),
            None => html,
        }
    }
}

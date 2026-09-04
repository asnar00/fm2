struct feature_OnEveryPost;
impl feature_OnEveryPost {
    // ---- the line goes on a copy too -----------------------------------------
    // /audience drew "visible to …" only under your own post, because the line
    // arrived beside the arrow that changes it and the arrow is the author's
    // alone. Ash asked for the fact on every post you open (#p120): who a post
    // reaches is worth knowing whoever wrote it.
    //
    // A copy carries its own `floor` — /exchange copies the card whole — so
    // there is nothing to fetch and nothing to guess: the same reading, the
    // same words, the same place. Only the button stays the author's, and that
    // is /visibility's own gate, untouched here.
    //
    // Whole redefinition rather than a wrapper: the base's answer for a copy is
    // "no line", which is the thing being replaced, and calling it first would
    // draw the author's line twice on the author's own post.
    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !posts_is(&c) {
            return html;
        }
        // the author's own post already has its line from /audience — adding a
        // second one is what this test is for.
        if c["from"].is_null() {
            return html;
        }
        if audience_in_of(&c).is_empty() {
            return html;
        }
        // a post filed before /audience has no floor and no honest answer.
        // The card's OWN field is read, not `audience_floor_of`, because that
        // reader answers `team` for a card that carries nothing — the right
        // default for deciding who may hold a post, and a level invented out of
        // nothing if it were put on the screen as a fact. Nothing is drawn.
        let floor = c["floor"].as_str().unwrap_or("").to_string();
        if !audience_is_grade(floor.clone()) {
            return html;
        }
        let line = format!("<div class=\"card-audience\">{}</div>",
                           card_esc(audience_line(floor)));
        projects_inside(html, line)
    }
}

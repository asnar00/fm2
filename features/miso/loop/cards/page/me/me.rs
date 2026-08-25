struct feature_Me;
impl feature_Me {
    // whether this page is what 👤 lands on. A seam (2026-08-25, for /people):
    // the default is yes; a feature that puts another surface under 👤 says
    // no while that surface is showing, without touching this file.
    fn me_landing() -> bool {
        true
    }

    // the 👤 tool's surface: your own card, rendered as a page of blocks by
    // /cards. The owner test is empty — a world holds only its owner's cards
    // today; exchange is what earns it.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "account" || !me_landing() {
            return base;
        }
        let card = card_of_type(cards_read(), String::new(), "profile".to_string());
        if card.is_empty() {
            return format!(
                "{}<div class=\"card-page card-waiting\">making your card…</div>",
                base);
        }
        let page = card_page_html(card);
        let under = me_under(state);
        if under.is_empty() {
            return format!("{}{}", base, page);
        }
        // inside the card page's own box, not after it: .card-page is fixed
        // and scrolls its own contents, so a sibling would land off-screen
        let inner = page.strip_suffix("</div>").unwrap_or(page.as_str()).to_string();
        format!("{}{}{}</div>", base, inner, under)
    }

    // the seam for things that belong UNDER your card on this page. The
    // default is nothing at all, so with no one filling it 👤 renders exactly
    // as it did; /invite is its first filler.
    fn me_under(state: String) -> String {
        let _ = state;
        String::new()
    }
}

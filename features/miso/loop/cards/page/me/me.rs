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
        // the live context, not the bridged mirror in `state`: /payload
        // republishes `open_tool` part-way down the update chain and /people
        // writes it back at a later link (👤 tapped over an open card means
        // "back to the people", which closes the tool and re-opens it), so
        // `s["open_tool"]` is one turn stale on exactly that tap — and this is
        // a renderer, which /browse says may not read a stale value. Your own
        // card would have been drawn a turn late, with the toolbar already
        // showing 👤 open.
        if open_tool_read() != "account" || !me_landing() {
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

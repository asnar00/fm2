struct feature_NotPeople;
impl feature_NotPeople {
    // ---- a person is never hidden by a clock -------------------------------
    // /since cuts a browsed set to a slice of time, which is right for things
    // that HAPPEN — a post has a moment — and wrong for people, who are not
    // events. Ash filed it from the field: everyone you hold on the project
    // shows, under every pill.
    //
    // The test is the card's type rather than which tool is open, because it
    // is truer: a person is not a thing that happened, wherever they are
    // drawn. /since's own exemption for your OWN profile card becomes a
    // special case of this one and is left where it is — unticking this node
    // gives it back exactly.

    fn since_keep(card: &serde_json::Value) -> bool {
        if card["type"].as_str().unwrap_or("") == "profile" {
            return true;
        }
        existing.since_keep(card)
    }

    // ---- and the slot goes quiet where it means nothing ---------------------
    // With people never cut, the four words change nothing on 👤 — and a
    // control that does nothing is noise (/taste 7: no explaining beside a
    // thing that shows what it does; /taste 8: one channel speaks at a time).
    // So the slot is empty on the people tool and unchanged on posts and
    // projects, where the filter still means what it says.
    //
    // The period itself is untouched: it is a user var, it keeps whatever it
    // held, and walking to posts finds the same word lit as before.

    fn browse_slot_html() -> String {
        if open_tool_read() == "account" {
            return String::new();
        }
        existing.browse_slot_html()
    }
}

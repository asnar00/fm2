struct feature_Explained;
impl feature_Explained {
    // ---- one level per line, and a line saying who sees it -------------------
    // /armed drew the levels as pills that wrap: seven words in three rows,
    // and nothing to tell you what any of them costs. Ash asked for a column
    // with a sentence each (#p30). The column is CSS; the sentence is this.
    //
    // Spliced INTO the element /armed drew rather than drawn instead of it:
    // the lit class, the `data-ev` and whatever a later sibling puts on that
    // element all survive, and this node only adds words (misses.md, "siblings
    // at one anchor" — a node redefines the narrowest thing that will do).
    fn armed_pill(ev: String, label: String, on: bool) -> String {
        let base = existing.armed_pill(ev.clone(), label, on);
        let says = armed_says(ev);
        if says.is_empty() {
            return base;
        }
        let line = format!("<span class=\"armed-says\">{}</span>", says);
        match base.rfind("</span>") {
            Some(at) => format!("{}{}{}", &base[..at], line, &base[at..]),
            None => base,
        }
    }

    // who a post at this level reaches, in the app's own words for it: a floor
    // is the lowest rank that holds the post, so every level except the last
    // reads "<them> and up" — which is exactly the sentence /audience already
    // writes under a post ("visible to the team and up"). The two surfaces say
    // the same fact the same way, which is the point (learned 9).
    //
    // The words are held here rather than asked of /audience for the reason
    // /armed holds its own list: this node must not fall over when /audience
    // is not composed. The same cost is named there — the tables have to agree.
    //
    // The level is read off the event name, because that is where /armed put
    // it: `armed_lvl_<word>`, and `armed_lvl_` with nothing after it is the
    // "same as me" entry.
    fn armed_says(ev: String) -> String {
        let word = match ev.strip_prefix("armed_lvl_") {
            Some(w) => w.to_string(),
            None => return String::new(),
        };
        match word.as_str() {
            "" => "your own role".to_string(),
            "admin" => "the project's admins only".to_string(),
            "candidate" => "candidates and up".to_string(),
            "team" => "the team and up".to_string(),
            "volunteer" => "volunteers and up".to_string(),
            "supporter" => "supporters and up".to_string(),
            "public" => "everyone in the project".to_string(),
            _ => String::new(),
        }
    }
}

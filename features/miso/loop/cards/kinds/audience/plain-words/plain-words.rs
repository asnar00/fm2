struct feature_PlainWords;
impl feature_PlainWords {
    // ---- "and up" goes ------------------------------------------------------
    // "visible to candidates and up" made a reader work out a ladder before
    // they could read a sentence. Ash asked for the plain thing: "visible to
    // candidates". The rank order has not changed and neither has who actually
    // receives a post — only the words.
    //
    // Two surfaces say this fact and there is now ONE table for both: the line
    // under a post (this node's parent) and the publish-level column in the
    // recording row (`/armed/explained`). This node answers both from
    // `plain_words_of`, so the words cannot drift apart — which is the whole
    // reason it is one node and not two.

    fn audience_line(grade: String) -> String {
        let says = plain_words_of(grade.clone());
        if says.is_empty() {
            return existing.audience_line(grade);
        }
        format!("visible to {}", says)
    }

    // /armed/explained's own seam: the sentence under a level in the column.
    // Redefined whole rather than wrapped — the base's answer is the wording
    // being replaced, not a fallback to fall through to — and with
    // /explained absent this is simply a function nobody calls, which is how
    // /armed's own list keeps its dependencies one-directional.
    //
    // The level is read off the event name, which is where /armed put it:
    // `armed_lvl_<word>`, and `armed_lvl_` with nothing after it is the
    // "same as me" entry.
    fn armed_says(ev: String) -> String {
        let word = match ev.strip_prefix("armed_lvl_") {
            Some(w) => w.to_string(),
            None => return String::new(),
        };
        if word.is_empty() {
            return "your own rank".to_string();
        }
        plain_words_of(word)
    }

    // who a post at this level reaches, said once. `admin` keeps its own
    // sentence because "visible to admins" would read as "any admin anywhere"
    // — /audience's two ladders collide on that word, and the project's is the
    // one meant. `public` is the widest rung and says so.
    fn plain_words_of(grade: String) -> String {
        match grade.as_str() {
            "admin" => "the project's admins only".to_string(),
            "candidate" => "candidates".to_string(),
            "team" => "the team".to_string(),
            "volunteer" => "volunteers".to_string(),
            "supporter" => "supporters".to_string(),
            "public" => "everyone in the project".to_string(),
            _ => String::new(),
        }
    }
}

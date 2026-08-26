struct feature_PlusAtHome;
impl feature_PlusAtHome {
    // the + belongs to the set of posts, not to a post you are reading: with
    // a card open under the posts tool, the row loses the new button
    fn tool_controls(state: String) -> String {
        let html = existing.tool_controls(state);
        if open_tool_read() != "posts" || browse_open_read().is_empty() {
            return html;
        }
        plus_at_home_strip(html)
    }

    // remove the whole posts_new button element from the row
    fn plus_at_home_strip(html: String) -> String {
        match html.find("data-ev=\"posts_new\"") {
            Some(at) => match (html[..at].rfind("<div"), html[at..].find("</div>")) {
                (Some(start), Some(rel)) => format!("{}{}", &html[..start], &html[at + rel + 6..]),
                _ => html,
            },
            None => html,
        }
    }
}

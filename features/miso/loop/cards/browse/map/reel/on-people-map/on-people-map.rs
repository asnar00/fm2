struct feature_OnPeopleMap;
impl feature_OnPeopleMap {
    // the posts set's ids ride #mapData too, so a map that drew people can
    // still say which posts the band should hold
    fn map_surface_html(cards: &Vec<serde_json::Value>) -> String {
        let html = existing.map_surface_html(cards);
        let ids: Vec<String> = posts_set().iter()
            .map(|c| c["id"].as_str().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let mark = "<div id=\"mapData\"";
        match html.find(mark) {
            Some(at) => format!("{}{} data-post-ids=\"{}\"{}", &html[..at], mark,
                                card_esc(ids.join(",")), &html[at + mark.len()..]),
            None => html,
        }
    }
}

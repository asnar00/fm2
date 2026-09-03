struct feature_Reel;
impl feature_Reel {
    // the set the map shows, by id, on the same element as the pins: the reel
    // lists exactly what the tool's set holds — a post the current project
    // sifts out is not in the band either (#p22)
    fn map_surface_html(cards: &Vec<serde_json::Value>) -> String {
        let html = existing.map_surface_html(cards);
        let ids: Vec<String> = cards.iter()
            .map(|c| c["id"].as_str().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let mark = "<div id=\"mapData\"";
        match html.find(mark) {
            Some(at) => format!("{}{} data-ids=\"{}\"{}", &html[..at], mark,
                                card_esc(ids.join(",")), &html[at + mark.len()..]),
            None => html,
        }
    }
}

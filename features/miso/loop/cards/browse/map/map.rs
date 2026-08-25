struct feature_Map;
impl feature_Map {
    // ---- the third view in the picker --------------------------------------
    // /browse drew the row as "grid unless list", which cannot be extended by
    // appending: with the map chosen, grid would light up beside it. So the
    // row is restated here — three buttons, each lit by its own name — rather
    // than appended to. A fourth view does the same thing again, or /browse
    // grows a `browse_view_is(which)` predicate if that ever gets tiresome.

    fn browse_views() -> String {
        let view = browse_view_read();
        format!("{}{}{}",
                browse_view_button("grid".to_string(),
                                   view != "list" && view != "map"),
                browse_view_button("list".to_string(), view == "list"),
                browse_view_button("map".to_string(), view == "map"))
    }

    // the map's own button, drawn like its siblings; every other name goes on
    // down the chain to /browse's two. The data-ev is a literal string, not a
    // placeholder, so /sub-tool-cards' long-press can read it out of the
    // source.
    fn browse_view_button(which: String, on: bool) -> String {
        if which != "map" {
            return existing.browse_view_button(which, on);
        }
        let lit = if on { " browse-on" } else { "" };
        format!("<div class=\"browse-view{}\" data-ev=\"browse_map\" title=\"map\">{}</div>",
                lit, map_fold_svg())
    }

    // ---- choosing it --------------------------------------------------------
    // /browse's own two clicks are unchanged; this is the third, and it does
    // exactly what they do — write the view, and put the set back if a card
    // was open, because the picker is on screen over a card page too.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        if e["ev"].as_str().unwrap_or("") != "browse_map" {
            return state;
        }
        browse_view_write("map".to_string());
        if !browse_open_read().is_empty() {
            browse_open_write(String::new());
        }
        state
    }

    // ---- the surface --------------------------------------------------------
    // `browse_set_html` is the grid/list switch, so it is also the map/not-map
    // switch: one link, and /browse's own two views are untouched behind it.
    // Note this runs BEFORE the empty-set check, so a map with no pins is
    // still a map — which is right: an empty grid has nothing to show, an
    // empty map still shows where you are.

    fn browse_set_html(cards: &Vec<serde_json::Value>) -> String {
        if browse_view_read() != "map" {
            return existing.browse_set_html(cards);
        }
        map_surface_html(cards)
    }

    // Rust draws no map. It draws the ARGUMENT: one empty element carrying the
    // located cards as JSON, which the page half reads after every repaint.
    // The map itself lives outside #app and survives the repaint that replaces
    // this element — /keep's idiom for furniture a repaint must not destroy.
    fn map_surface_html(cards: &Vec<serde_json::Value>) -> String {
        let mut pins: Vec<serde_json::Value> = Vec::new();
        for c in cards.iter() {
            let at = card_place_of(c.to_string());
            if at.is_null() {
                continue;
            }
            pins.push(serde_json::json!({
                "id": c["id"].as_str().unwrap_or(""),
                "lat": at["lat"].as_f64().unwrap_or(0.0),
                "lon": at["lon"].as_f64().unwrap_or(0.0),
                "face": map_face_of(c),
                "initial": map_initial_of(c),
                "title": map_title_of(c)
            }));
        }
        let json = serde_json::Value::Array(pins).to_string();
        format!("<div id=\"mapData\" data-pins=\"{}\"></div>", card_esc(json))
    }

    // the card's own face, as the tile draws it: the first picture block, or
    // nothing and then the title's first character. The pin wears one or the
    // other, so a person is recognisable on the map at a glance.
    fn map_face_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "picture" {
                return b["data"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    fn map_title_of(card: &serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") == "title" {
                return b["text"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    fn map_initial_of(card: &serde_json::Value) -> String {
        // a card with no title (a post) pins with its owner's initial, as
        // /browse's row does (found by /posts, same day)
        let title = map_title_of(card);
        let from = if title.is_empty() {
            card["owner"].as_str().unwrap_or("").to_string()
        } else {
            title
        };
        from.chars().take(1).collect()
    }

    // ---- the glyph -----------------------------------------------------------
    // a folded map, drawn in currentColor (/glyphs). NOT a pin: the pins are
    // the things ON this view, and a screen carrying the same shape as both
    // its control and its content reads as a mistake — /browse made the same
    // call when it refused four squares for the tool that holds the grid.

    fn map_fold_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M9 4 3 6v14l6-2 6 2 6-2V4l-6 2z\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "<path d=\"M9 4v14M15 6v14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }
}

struct feature_SquarePosts;
impl feature_SquarePosts {
    // ---- the kind travels with the row -------------------------------------
    // /map's row says where a card is and what face it wears, and nothing about
    // what kind of card it is — so the page half cannot tell a post from a
    // person. The rows are not rebuilt here: /map's own builder runs, and its
    // one element is opened, one field added per row, and closed again. A row
    // keeps every field /map gives it, including any /map grows later, and a
    // shape this node does not recognise is handed back untouched — pins as
    // /map drew them, which is circles.

    fn map_surface_html(cards: &Vec<serde_json::Value>) -> String {
        let html = existing.map_surface_html(cards);
        square_posts_kinded(html, cards)
    }

    // the rows are matched to their cards by id, never by position, so the
    // filtering /map does (a card with no place is not a row) needs no
    // restating here and cannot drift out of step with it.
    fn square_posts_kinded(html: String, cards: &Vec<serde_json::Value>) -> String {
        let head = "data-pins=\"";
        let start = match html.find(head) {
            Some(i) => i + head.len(),
            None => return html,
        };
        let end = match html[start..].find('"') {
            Some(i) => start + i,
            None => return html,
        };
        let json = square_posts_unesc(html[start..end].to_string());
        let mut rows: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(_) => return html,
        };
        let n = match rows.as_array() {
            Some(a) => a.len(),
            None => return html,
        };
        for i in 0..n {
            let id = rows[i]["id"].as_str().unwrap_or("").to_string();
            if id.is_empty() {
                continue;
            }
            let kind = square_posts_kind_of(cards, id);
            if kind.is_empty() {
                continue;
            }
            rows[i]["kind"] = serde_json::Value::String(kind);
        }
        format!("{}{}{}",
                &html[..start],
                card_esc(rows.to_string()),
                &html[end..])
    }

    fn square_posts_kind_of(cards: &Vec<serde_json::Value>, id: String) -> String {
        for c in cards.iter() {
            if c["id"].as_str().unwrap_or("") == id {
                return c["type"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    // the exact inverse of /cards' `card_esc`, ampersand last, so a title or a
    // picture path carrying one of these characters survives the round trip
    // unchanged.
    fn square_posts_unesc(s: String) -> String {
        s.replace("&quot;", "\"")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&amp;", "&")
    }
}

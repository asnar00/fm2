struct feature_Poster;
impl feature_Poster {
    // ---- the poster is the card's picture ----------------------------------
    // a frame off the clip, shrunk and stored in the picture block the card
    // was minted with. Nothing about the tile, the map pin or /picture-first
    // changes: they all read the first picture block and now find one. What
    // marks it is `poster` on the block — the card's picture is the video's
    // face, not a second thing the user put there.

    // the poster block, or null. A picture block with data AND the mark: a
    // picture someone chose by hand is not a poster and never merges.
    fn poster_block(card: &serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") != "picture" {
                continue;
            }
            if b["poster"].as_bool().unwrap_or(false)
                && !b["data"].as_str().unwrap_or("").is_empty() {
                return b.clone();
            }
        }
        serde_json::Value::Null
    }

    // ---- which medium ------------------------------------------------------
    // /one-medium asks this to decide whether the dashed invitation is drawn.
    // A poster would answer "picture" and be right about the block and wrong
    // about the card: the medium is the video, and the picture is its face.
    fn one_medium_carried(card: &serde_json::Value) -> String {
        if !poster_block(card).is_null() && !video_block(card).is_null() {
            return "video".to_string();
        }
        existing.one_medium_carried(card)
    }

    // ---- the page draws one thing ------------------------------------------
    // composed last of `card_page_html` — this prompt is the newest in the
    // tree — so the finished page is what arrives: /cards' filled picture and
    // /capture/video's player row, two media presences for one medium. Both
    // are cut and one is put back in the picture's place: the frame, with a
    // play glyph over it. The tap is the page half's.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let p = poster_block(&c);
        let v = video_block(&c);
        if p.is_null() || v.is_null() {
            return html;
        }
        let row = if c["from"].is_null() {
            poster_row(&p, &v)
        } else {
            poster_foreign(&p, &v)
        };
        // the poster stands exactly where the picture stood, so /picture-first
        // and /titled/above order it as they ordered the picture. It goes in by
        // replacement rather than by remembered index: /posts moves the words
        // ahead of the picture and /capture/video mounts its player between
        // them, so a position found before the cuts is no longer that position
        // after them (this landed the poster inside /location's pin).
        if !html.contains("<div class=\"card-pic\"") {
            return html;
        }
        let html = poster_swap(html, "<div class=\"card-pic\"", row);
        let html = poster_cut(html, "<div class=\"post-video\" data-vid=");
        poster_cut(html, "<div class=\"post-video dim\">")
    }

    // one element replaced by another, in place.
    fn poster_swap(html: String, mark: &str, row: String) -> String {
        match html.find(mark) {
            Some(i) => match html[i..].find("</div>") {
                Some(j) => format!("{}{}{}", &html[..i], row, &html[i + j + 6..]),
                None => html,
            },
            None => html,
        }
    }

    // one element out of the drawn page: find the opening tag, find the
    // `</div>` that closes it, splice the rest. Both elements hold spans and
    // an img and no nested div, and every stored string was escaped on the way
    // in (/cards' `card_esc`), so that first `</div>` is the right one. A mark
    // that is not there leaves the page untouched — which is how the foreign
    // and own rows share this without either knowing about the other.
    fn poster_cut(html: String, mark: &str) -> String {
        match html.find(mark) {
            Some(i) => match html[i..].find("</div>") {
                Some(j) => format!("{}{}", &html[..i], &html[i + j + 6..]),
                None => html,
            },
            None => html,
        }
    }

    // the frame with the play glyph over it. `data-vid` is the page half's
    // handle and `data-rec` is what /as-posts' "transcribing…" hint looks
    // for — the same two the player row carried, so nothing downstream of
    // either notices the swap.
    fn poster_row(poster: &serde_json::Value, video: &serde_json::Value) -> String {
        let data = card_esc(poster["data"].as_str().unwrap_or("").to_string());
        let id = card_esc(video["id"].as_str().unwrap_or("").to_string());
        let dur = as_posts_mmss(video["dur"].as_u64().unwrap_or(0));
        format!(concat!("<div class=\"post-poster\" data-vid=\"{}\" data-rec=\"{}\">",
                        "<span class=\"poster-frame\"><img src=\"{}\" alt=\"\">",
                        "<span class=\"poster-play\">{}</span></span>",
                        "<span class=\"post-dur\">{}</span></div>"),
                id, id, data, poster_play_svg(), dur)
    }

    // a copy carries the card, so it carries the poster; the bytes stay with
    // their owner. The frame shows, and the row says why it will not play
    // rather than offering a glyph that does nothing (/taste 7).
    fn poster_foreign(poster: &serde_json::Value, video: &serde_json::Value) -> String {
        let data = card_esc(poster["data"].as_str().unwrap_or("").to_string());
        let dur = as_posts_mmss(video["dur"].as_u64().unwrap_or(0));
        format!(concat!("<div class=\"post-poster dim\">",
                        "<span class=\"poster-frame\"><img src=\"{}\" alt=\"\"></span>",
                        "<span class=\"post-dur\">{}</span>",
                        "<span class=\"post-play-note\">video stays with its owner</span>",
                        "</div>"),
                data, dur)
    }

    // drawn ink, currentColor, per /glyphs: a triangle inside the same thin
    // ring the map pin wears. The ring carries the only ground the glyph puts
    // over the frame — no scrim across the picture.
    fn poster_play_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"12\" r=\"10.4\" fill=\"rgba(16,16,18,0.55)\" ",
            "stroke=\"currentColor\" stroke-width=\"1.5\"/>",
            "<path d=\"M9.8 7.9l6.4 4.1 -6.4 4.1z\" fill=\"currentColor\"/>",
            "</svg>"))
    }

    // ---- the poster arrives ------------------------------------------------
    // the page half sends this once, on the device that recorded, after the
    // card exists. It names the RECORDING, not the card: /as-posts mints the
    // card id from the owner and the recording's moment, and the page half
    // would have to guess both. `rec` is on the card already and survives a
    // delete (/delete's tombstone clones it), so a deleted video's poster
    // lands nowhere rather than resurrecting anything.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "CardPoster" {
            return state;
        }
        let rec = e["data"]["rec"].as_str().unwrap_or("").to_string();
        let data = e["data"]["data"].as_str().unwrap_or("").to_string();
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        if rec.is_empty() || data.is_empty() {
            return state;
        }
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return state;
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["rec"].as_str().unwrap_or("") != rec {
                continue;
            }
            let mut hit = false;
            if let Some(blocks) = c["blocks"].as_array_mut() {
                for b in blocks.iter_mut() {
                    if b["kind"].as_str().unwrap_or("") != "picture" {
                        continue;
                    }
                    // a picture already there is the user's own and outranks a
                    // poster: the face is only ever written into an empty slot.
                    if !b["data"].as_str().unwrap_or("").is_empty() {
                        break;
                    }
                    b["data"] = serde_json::json!(data);
                    b["poster"] = serde_json::json!(true);
                    hit = true;
                    break;
                }
            }
            if hit {
                c["edited"] = serde_json::json!(now);
                changed = true;
            }
        }
        if changed {
            cards_write(list.to_string());
        }
        state
    }
}

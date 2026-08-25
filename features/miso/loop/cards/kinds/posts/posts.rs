struct feature_Posts;
impl feature_Posts {
    // ---- the set -----------------------------------------------------------
    // the posts you hold — yours and the copies /exchange put in your world —
    // newest first. Read straight from /cards' store rather than through
    // /browse's `browse_cards` seam: that chain has already been narrowed to
    // profiles for 👤 (/people), and a second surface asking it a different
    // question would have to undo that. The subset lives here; the picker and
    // the two renderers stay /browse's.
    //
    // `created`, not `edited`: a post is dated by when it was written, so
    // fixing a typo tomorrow does not move it to the top of the list.

    fn posts_set() -> Vec<serde_json::Value> {
        let list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") == "post" {
                out.push(c.clone());
            }
        }
        out.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            let ta = a["created"].as_u64().unwrap_or(0);
            let tb = b["created"].as_u64().unwrap_or(0);
            tb.cmp(&ta).then(posts_id_of(b).cmp(&posts_id_of(a)))
        });
        out
    }

    // the tie-break, so two posts minted in the same millisecond keep a stable
    // order on every device rather than whichever the sort happened to see.
    fn posts_id_of(card: &serde_json::Value) -> String {
        card["id"].as_str().unwrap_or("").to_string()
    }

    // who wrote it. `owner` is the author's name on your own posts and on the
    // copies alike (/exchange's copy keeps it), so one field answers for both.
    fn posts_author(card: &serde_json::Value) -> String {
        card_esc(card["owner"].as_str().unwrap_or("").to_string())
    }

    fn posts_is(card: &serde_json::Value) -> bool {
        card["type"].as_str().unwrap_or("") == "post"
    }

    // ---- a second post is not a duplicate of the first ----------------------
    // /guard drops a card that arrives BLANK for an owner who already holds
    // one of its type: that shape is /me's ensure racing an empty world, and
    // discarding it is what keeps a profile from being doubled. The premise —
    // "you hold exactly one of these" — is true of a profile and false of a
    // post: every post is blank at the moment `new` makes it, so with one post
    // already written the rule threw the next one away before it could be
    // typed into (rig-found, 2026-08-25; the second post simply never
    // appeared). This node answers the question the discard rule is really
    // asking — *is this a copy of a card that should be unique* — with `no`
    // for a post, and leaves every other type to /guard's own answer. Nothing
    // is dropped that was not dropped before; only a card that would have been
    // lost survives.
    fn cards_guard_has_type(cur: &Vec<serde_json::Value>, card: &serde_json::Value) -> bool {
        if posts_is(card) {
            return false;
        }
        existing.cards_guard_has_type(cur, card)
    }

    // ---- the row -----------------------------------------------------------
    // a post has no title, so /browse's two row seams are re-aimed at the
    // author: the bold cell — which /portrait made the row's identity — says
    // who wrote it, and the left cell says nothing, because "post" on a
    // surface of nothing but posts is the redundancy /people already ruled on.
    // The date is /browse's own right-hand cell and the words are /portrait's
    // excerpt; neither is touched. Keyed on the card's type, not on which tool
    // is open, so a post says who wrote it wherever it is drawn.

    fn browse_title_of(card: &serde_json::Value) -> String {
        if !posts_is(card) {
            return existing.browse_title_of(card);
        }
        posts_author(card)
    }

    fn browse_row_left(card: &serde_json::Value) -> String {
        if !posts_is(card) {
            return existing.browse_row_left(card);
        }
        String::new()
    }

    // the grid tile: /cards draws the caption from the card's own title block
    // and the empty face from its initial, and a post's title is empty — so
    // both come out as exactly empty elements, which is what makes these two
    // marks unambiguous. The author goes into them; the tile's look is
    // untouched.
    fn card_tile_html(card: String) -> String {
        let html = existing.card_tile_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !posts_is(&c) {
            return html;
        }
        let who = posts_author(&c);
        let initial: String = who.chars().take(1).collect();
        let html = html.replace(
            "<div class=\"card-tile-face empty\"></div>",
            &format!("<div class=\"card-tile-face empty\">{}</div>", initial));
        html.replace("<div class=\"card-tile-title\"></div>",
                     &format!("<div class=\"card-tile-title\">{}</div>", who))
    }

    // ---- the page ----------------------------------------------------------
    // the same three-block body every card has — so /location, /frame, /keep
    // and /guard work on a post untouched — drawn differently: no title (a
    // post is not named), the words above the picture (a post is words), and
    // the page marked `post` so the words can carry the weight the name
    // carries on a profile.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !posts_is(&c) {
            return html;
        }
        let html = html.replacen("<div class=\"card-page",
                                 "<div class=\"card-page post", 1);
        let html = posts_no_title(html);
        let html = posts_text_first(html);
        html.replace("data-ph=\"say what you are here to do\"",
                     "data-ph=\"say something\"")
    }

    // the title block, out of the page the chain beneath drew. A block's text
    // is escaped on the way in (/cards' `card_esc`), so the first `</div>`
    // after the opening tag is its own — nothing can nest inside it.
    fn posts_no_title(html: String) -> String {
        let at = match html.find("<div class=\"card-title") {
            Some(i) => i,
            None => return html,
        };
        let end = match html[at..].find("</div>") {
            Some(j) => at + j + 6,
            None => return html,
        };
        format!("{}{}", &html[..at], &html[end..])
    }

    // the words above the picture. A rendering move, not a change to the
    // object: the blocks keep their order and their indices, so every
    // `data-block` the page half sends still names the block it always did.
    // A page with no picture block — a foreign post, where /exchange takes the
    // empty one away — is left as it is.
    fn posts_text_first(html: String) -> String {
        let pic = match html.find("<div class=\"card-pic") {
            Some(i) => i,
            None => return html,
        };
        let pic_end = match html[pic..].find("</div>") {
            Some(j) => pic + j + 6,
            None => return html,
        };
        let text = match html[pic_end..].find("<div class=\"card-text") {
            Some(i) => pic_end + i,
            None => return html,
        };
        let text_end = match html[text..].find("</div>") {
            Some(j) => text + j + 6,
            None => return html,
        };
        format!("{}{}{}{}{}",
                &html[..pic],
                &html[text..text_end],
                &html[pic_end..text],
                &html[pic..pic_end],
                &html[text_end..])
    }

    // ---- the toolbar -------------------------------------------------------

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({
                "id": "posts", "label": "posts", "icon": posts_bubble_svg() }));
        }
        list.to_string()
    }

    // the one control this tool adds: make one. In front of /undo's button,
    // never after it — undo is last in every row, and a newer node's links
    // land after undo's by provenance, so keeping the invariant is the
    // newcomer's job (/glyphs). Written out here rather than borrowed from
    // /under-account's `before_undo`, so this node stands without /invite.
    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state);
        if open_tool_read() != "posts" {
            return row;
        }
        posts_before_undo(row, posts_new_button())
    }

    // the tool's own colour, not a colour of its own: /ember's pick for "new"
    // is byte-summed to the same blue it gives "undo", and two controls side
    // by side in one colour read as one pair (/taste 3 — a colour is a word).
    // Wearing the posts tool's pink says which tool the act belongs to, and
    // the lit tool button beside it is the same hue with more light.
    fn posts_new_button() -> String {
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"posts_new\" title=\"new\">{}</div>",
                tint, posts_plus_svg())
    }

    fn posts_before_undo(row: String, add: String) -> String {
        if add.is_empty() {
            return row;
        }
        match row.find("data-ev=\"ctx_undo\"") {
            Some(at) => match row[..at].rfind("<div") {
                Some(start) => format!("{}{}{}", &row[..start], add, &row[start..]),
                None => format!("{}{}", row, add),
            },
            None => format!("{}{}", row, add),
        }
    }

    // ---- the events --------------------------------------------------------
    // making a post is the page half's act, because the author's name lives
    // behind the cookie and never in the world: `posts.js` sends /new's own
    // `CardNew`, which appends the card and opens its page. What is left here
    // is the way back — /tools closes a tool when its own button is tapped,
    // and with a post showing that tap means "back to the posts" instead
    // (/browse's grammar for `tool_cards`, #p88). Both vars are read BEFORE
    // the chain beneath runs, because /tools and /browse have cleared them by
    // the time it returns.

    fn update(state: String, event: String) -> String {
        let was_tool = open_tool_read();
        let was_open = browse_open_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        if e["ev"].as_str().unwrap_or("") == "tool_posts"
            && was_tool == "posts" && !was_open.is_empty() {
            open_tool_write("posts".to_string());
        }
        state
    }

    // ---- the surface -------------------------------------------------------
    // /browse's surface, aimed at the posts: the same picker, the same grid
    // and list, the same open-a-card path. Only the set and the empty line
    // belong to this node.

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if open_tool_read() != "posts" {
            return base;
        }
        let set = posts_set();
        let picker = browse_picker_html();
        let open = browse_open_read();
        if !open.is_empty() {
            for c in set.iter() {
                if c["id"].as_str().unwrap_or("") == open {
                    return format!("{}{}{}", base, picker,
                                   card_page_html(c.to_string()));
                }
            }
            // the post is gone, or what is open is not a post: the set is the
            // honest fallback, silently — /browse's own rule.
        }
        // nothing written yet: one quiet line where the set would be. Not in
        // the map view — /map's ruling is that an empty map is still a map,
        // and that is its call to make, not this node's.
        if set.is_empty() && browse_view_read() != "map" {
            return format!("{}{}<div class=\"browse-empty\">say something</div>",
                           base, picker);
        }
        format!("{}{}{}", base, picker, browse_set_html(&set))
    }

    // ---- the glyphs --------------------------------------------------------
    // drawn, in currentColor, per /glyphs — never a character with an emoji
    // presentation. The bubble is what the tool is; the plus is what the
    // control does.

    fn posts_bubble_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M4 6.5A2.5 2.5 0 0 1 6.5 4h11A2.5 2.5 0 0 1 20 6.5v7a2.5 2.5 0 0 1-2.5 2.5H11l-4.5 4v-4h-.5A2.5 2.5 0 0 1 4 13.5z\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }

    fn posts_plus_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M12 5v14M5 12h14\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.6\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}

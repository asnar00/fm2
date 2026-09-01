struct feature_Projects;
impl feature_Projects {
    // ---- the toolbar -------------------------------------------------------
    // one more tool in the registry, with a drawn flag for a glyph: a shape,
    // so /glyphs says draw it in currentColor rather than reach for an emoji.

    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({
                "id": "projects", "label": "projects",
                "icon": projects_flag_svg() }));
        }
        list.to_string()
    }

    // the control row: **new**, and only while the set is showing — on a
    // project's own page it would be a control about something else.
    // /undo's button is last in every row, so this one goes in FRONT of it.
    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "projects" {
            return row;
        }
        if !browse_open_read().is_empty() {
            return row;
        }
        projects_before_undo(row, projects_new_button())
    }

    // no data-ev: the page half makes the card, because the owner's name
    // lives behind the cookie and not in the world (/cards' own reason).
    fn projects_new_button() -> String {
        // the tool's own colour, /plus-tinted's rule: a control belonging to
        // a tool wears that tool's word. Asking /ember for "new" gives the
        // same blue it gives "undo", and two identical buttons side by side
        // read as one control split in two (/taste 3).
        let colour = tool_colour("projects".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!(
            "<div class=\"tool-button ctrl{}\" data-proj=\"new\" title=\"new\"><span class=\"icon\">{}</span></div>",
            tint, projects_plus_svg())
    }

    // /undo's button stays LAST in every control row — an invariant every
    // later node keeps, since provenance puts newer links after it.
    fn projects_before_undo(row: String, add: String) -> String {
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

    // ---- /browse's two seams -----------------------------------------------
    // taken by gate rather than by filter: /people redefines browse_cards to
    // the profiles and this link is OUTSIDE it (#p87 is after #p76), so a
    // filter here would see the profiles and answer with nothing. Asking
    // whose tool is open first leaves /people's surface exactly as it was.

    fn browse_cards(state: String) -> String {
        if open_tool_read() != "projects" {
            return existing.browse_cards(state);
        }
        let all: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in all.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") == "project" {
                out.push(c.clone());
            }
        }
        serde_json::Value::Array(out).to_string()
    }

    // where /taste 6 puts the number: how many people are in it. Nothing at
    // all for a project nobody is in yet — a "0 people" is a fact about an
    // empty thing that the title already tells you.
    fn browse_row_left(card: &serde_json::Value) -> String {
        if card["type"].as_str().unwrap_or("") != "project" {
            return existing.browse_row_left(card);
        }
        let n = projects_members(card).len();
        if n == 0 {
            return String::new();
        }
        if n == 1 {
            return "1 person".to_string();
        }
        format!("{} people", n)
    }

    // ---- the surface -------------------------------------------------------
    // /browse's own shape, because it is /browse's surface: the picker, then
    // the open card's page or the set. Only the tool it answers to differs.

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if open_tool_read() != "projects" {
            return base;
        }
        let list: serde_json::Value = serde_json::from_str(&browse_cards(state))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let cards = list.as_array().unwrap_or(&empty);
        let picker = browse_picker_html();
        let open = browse_open_read();
        if !open.is_empty() {
            for c in cards.iter() {
                if c["id"].as_str().unwrap_or("") == open {
                    return format!("{}{}{}", base, picker,
                                   card_page_html(c.to_string()));
                }
            }
            // gone, or not a project: the set is the honest fallback
        }
        if cards.is_empty() {
            return format!("{}{}<div class=\"browse-empty\">no projects yet</div>",
                           base, picker);
        }
        format!("{}{}{}", base, picker, browse_set_html(cards))
    }

    // ---- the page ----------------------------------------------------------
    // the /tag idiom: splice into the page `existing` returns, so /cards,
    // /exchange, /tag and /location all keep their say and this node adds
    // one section rather than drawing a page of its own.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let kind = c["type"].as_str().unwrap_or("").to_string();
        if kind == "project" {
            let html = projects_placeholders(html);
            return projects_inside(html, projects_people_html(&c));
        }
        if kind == "profile" {
            return projects_inside(html, projects_roles_html(&c));
        }
        html
    }

    // the page's own box is `position: fixed` and scrolls its contents, so a
    // section appended AFTER it lands off-screen — /me's reason for the same
    // move. Inside the closing div, then.
    fn projects_inside(html: String, add: String) -> String {
        if add.is_empty() {
            return html;
        }
        match html.strip_suffix("</div>") {
            Some(h) => format!("{}{}</div>", h, add),
            None => format!("{}{}", html, add),
        }
    }

    // /cards seeds a card with a profile's words in its empty blocks. On a
    // project they are a control saying the wrong thing (/taste 7), so the
    // two placeholders are rewritten — the blocks themselves are untouched.
    fn projects_placeholders(html: String) -> String {
        let html = html.replace("data-ph=\"your name\"",
                                "data-ph=\"what is it called\"");
        html.replace("data-ph=\"say what you are here to do\"",
                     "data-ph=\"what are we trying to get done\"")
    }

    // the people section. The owner gets the rows, an ✕ on each and **add**;
    // everybody else gets the rows and nothing to press — and, if there are
    // no rows, no heading over an empty box either.
    fn projects_people_html(c: &serde_json::Value) -> String {
        let mine = c["from"].is_null();
        let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
        let members = projects_members(c);
        if members.is_empty() && !mine {
            return String::new();
        }
        let mut out = String::from("<div class=\"proj-people\"><div class=\"proj-head\">people</div>");
        for l in members.iter() {
            let to = card_esc(l["to"].as_str().unwrap_or("").to_string());
            let name = card_esc(projects_link_name(l));
            let role = projects_people_role(l);
            // data-proj, not data-ev: the page half sends it, because the
            // event needs a clock and a wasm build has none — `now_ms` is
            // SystemTime, which panics in the browser half of the loop.
            let x = if mine {
                format!("<span class=\"proj-x\" data-proj=\"drop\" data-card=\"{}\" data-to=\"{}\">✕</span>",
                        id, to)
            } else {
                String::new()
            };
            out.push_str(&format!(
                "<div class=\"crow proj-row\"><span class=\"cnum proj-role\">{}</span><div class=\"ctext proj-name\">{}</div>{}</div>",
                role, name, x));
        }
        if mine {
            out.push_str(&format!(
                "<div class=\"proj-add\" data-proj=\"add\" data-card=\"{}\">add</div>", id));
        }
        out.push_str("</div>");
        out
    }

    // what a row on THIS page says a person does. An extension point, not a
    // decoration: the words are this node's answer, and a later node may say
    // more about the same person here without touching the row's shape — or
    // the role lines on a person's own card, which are a different question
    // and read the link themselves.
    fn projects_people_role(l: &serde_json::Value) -> String {
        card_esc(l["role"].as_str().unwrap_or("").to_string())
    }

    // "person P has role R in project X", asked from P's side: every project
    // card in the reader's OWN world with a link to this profile. That is the
    // whole of #p8 — you see the roles somebody handed you, and nothing else.
    fn projects_roles_html(c: &serde_json::Value) -> String {
        let pid = c["id"].as_str().unwrap_or("").to_string();
        let name = c["owner"].as_str().unwrap_or("").to_string();
        if pid.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for p in projects_roles_from().iter() {
            let title = browse_title_of(p);
            let pjid = card_esc(p["id"].as_str().unwrap_or("").to_string());
            for l in projects_members(p).iter() {
                let hit = l["to"].as_str().unwrap_or("") == pid
                    || (!name.is_empty() && projects_link_name(l) == name);
                if !hit {
                    continue;
                }
                out.push_str(&format!(
                    "<div class=\"crow proj-rolerow\" data-ev=\"proj_open:{}\"><span class=\"cnum proj-role\">{}</span><div class=\"ctext\">for {}</div></div>",
                    pjid, card_esc(l["role"].as_str().unwrap_or("").to_string()), title));
            }
        }
        if out.is_empty() {
            return String::new();
        }
        format!("<div class=\"proj-roles\">{}</div>", out)
    }

    // the project cards a role line may come from: every project in the
    // reader's own world. A seam — a later node says which of them still
    // count (a deleted project's role is not a role).
    fn projects_roles_from() -> Vec<serde_json::Value> {
        let all: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for p in all.as_array().unwrap_or(&empty) {
            if p["type"].as_str().unwrap_or("") == "project" {
                out.push(p.clone());
            }
        }
        out
    }

    // ---- reading a card's links --------------------------------------------

    // the role links of a card, in the order they were written. `links` is
    // /cards' field, declared empty on the very first card so this is a read
    // and not a migration.
    fn projects_members(c: &serde_json::Value) -> Vec<serde_json::Value> {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for l in c["links"].as_array().unwrap_or(&empty) {
            if l["kind"].as_str().unwrap_or("") != "role" {
                continue;
            }
            if l["to"].as_str().unwrap_or("").is_empty() {
                continue;
            }
            out.push(l.clone());
        }
        out
    }

    // whose role this is. A card id is `<owner>.<created ms>` (/cards), so
    // the id carries the name and is the authority; `name` is the fallback,
    // and the reason a link to a profile you have never held still renders.
    fn projects_link_name(l: &serde_json::Value) -> String {
        let to = l["to"].as_str().unwrap_or("").to_string();
        match to.rfind('.') {
            Some(i) => to[..i].to_string(),
            None => l["name"].as_str().unwrap_or("").to_string(),
        }
    }

    // ---- the events --------------------------------------------------------

    fn update(state: String, event: String) -> String {
        // read BEFORE the chain beneath runs: /tools closes the tool and
        // /browse clears `open` on a tool_ tap, so afterwards both are gone
        let was_tool = open_tool_read();
        let was_open = browse_open_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("").to_string();
        if kind == "RoleAdd" || kind == "RoleDrop" {
            projects_role(kind, e["data"].clone());
            return state;
        }
        if kind != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        // a role line on somebody's card: the project opens, wherever the tap
        // was. This link is the outermost `update`, so nothing after it can
        // put the tool or the open card back.
        if let Some(id) = ev.strip_prefix("proj_open:") {
            open_tool_write("projects".to_string());
            browse_open_write(id.to_string());
            return state;
        }
        // the tool's own button with a project open means "back to the set",
        // one level at a time (/tools' grammar, #p88): /tools has already
        // closed the tool, so it is put back here.
        if ev == "tool_projects" && was_tool == "projects" && !was_open.is_empty() {
            open_tool_write("projects".to_string());
        }
        state
    }

    // a role written, or taken away. Both are writes of the project card's
    // own `links`, through `cards_write` — so /guard merges them, /exchange
    // relays them and this node's hand-over sees an ordinary card write.
    //
    // `edited` moves on every one of them, which is what makes an add travel:
    // the add and the edit are one path and not two.
    fn projects_role(kind: String, d: serde_json::Value) {
        let card_id = d["card"].as_str().unwrap_or("").to_string();
        let to = d["to"].as_str().unwrap_or("").to_string();
        let name = d["name"].as_str().unwrap_or("").trim().to_string();
        let role = d["role"].as_str().unwrap_or("").trim().to_string();
        // the time comes off the page, as every other card event's does:
        // `render` and `update` both run in the wasm half too, where there
        // is no clock at all.
        let now = d["t"].as_u64().unwrap_or(0);
        if now == 0 || card_id.is_empty() || to.is_empty() {
            return;
        }
        if kind == "RoleAdd" && role.is_empty() {
            return;
        }
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return;
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != card_id {
                continue;
            }
            if c["type"].as_str().unwrap_or("") != "project" {
                continue;
            }
            // a copy is not yours to write on — /exchange's own test, and the
            // same one the renderer uses to decide who sees the ✕
            if !c["from"].is_null() {
                println!("projects: refused a role on a card that is not yours ({})",
                         card_id);
                continue;
            }
            let empty: Vec<serde_json::Value> = Vec::new();
            let was = c["links"].as_array().unwrap_or(&empty).len();
            let mut links: Vec<serde_json::Value> = Vec::new();
            for l in c["links"].as_array().unwrap_or(&empty) {
                if l["kind"].as_str().unwrap_or("") == "role"
                    && l["to"].as_str().unwrap_or("") == to {
                    continue;   // adding the same person again REPLACES
                }
                links.push(l.clone());
            }
            let dropped = was != links.len();
            if kind == "RoleAdd" {
                links.push(projects_role_link(d.clone(), to.clone(),
                                              name.clone(), role.clone(), now));
            } else if !dropped {
                continue;   // nothing to take away: not a write
            }
            c["links"] = serde_json::Value::Array(links);
            c["edited"] = serde_json::json!(now);
            changed = true;
        }
        if changed {
            cards_write(list.to_string());
        }
    }

    // ---- the hand-over ------------------------------------------------------
    // being in a project is holding its card, so a role is only half done
    // until the card is in that person's world. /exchange built the door for
    // exactly this (#p71) and this node is newer, so its `route` link is the
    // OUTERMOST one — outside /edit's turn and outside /per-user's identity,
    // which is the only place another world may be named and written.

    fn route(r: request) -> response {
        let watch = r.path == "msg" && r.method == "POST"
            && exchange_is_cards_op(&r.body);
        if !watch {
            return existing.route(r);
        }
        let who = exchange_who(&r);
        if who.is_empty() {
            return existing.route(r);
        }
        let before = exchange_cards_of(who.clone());
        let resp = existing.route(r);
        if resp.status != 200 {
            return resp;
        }
        let after = exchange_cards_of(who.clone());
        if after != before {
            projects_hand(who, before, after);
        }
        resp
    }

    // every project of the writer's that changed goes to everyone in it. The
    // diff is /exchange's: a card that was not there before counts as
    // changed, because a project's first write is exactly the one that has to
    // travel — and it will not, since a brand-new project has nobody in it.
    fn projects_hand(who: String, before: String, after: String) {
        let me = exchange_name_of(who.clone());
        let old: serde_json::Value = serde_json::from_str(&before)
            .unwrap_or(serde_json::json!([]));
        let new: serde_json::Value = serde_json::from_str(&after)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut sent = 0usize;
        for c in new.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "project" {
                continue;
            }
            if !c["from"].is_null() {
                continue;   // a copy is not yours to hand on
            }
            if !me.is_empty() && c["owner"].as_str().unwrap_or("") != me {
                continue;
            }
            if !exchange_owns_id(c, me.clone()) {
                continue;
            }
            let id = c["id"].as_str().unwrap_or("");
            let mut seen = false;
            let mut was = 0u64;
            for o in old.as_array().unwrap_or(&empty) {
                if o["id"].as_str().unwrap_or("") == id {
                    seen = true;
                    was = o["edited"].as_u64().unwrap_or(0);
                }
            }
            if seen && c["edited"].as_u64().unwrap_or(0) == was {
                continue;
            }
            let copy = exchange_copy(c, me.clone(), who.clone());
            for l in projects_members(c).iter() {
                let key = projects_key_for_name(projects_link_name(l));
                if key.is_empty() || key == who {
                    continue;
                }
                let mut one: Vec<serde_json::Value> = Vec::new();
                one.push(copy.clone());
                exchange_give(key, one);
                sent += 1;
            }
        }
        if sent > 0 {
            println!("projects: {} handed a project to {} member world(s)", me, sent);
        }
    }

    // the role link a `RoleAdd` writes. The whole event's data is handed in
    // so a later node may take a field of its own off it without this one
    // learning what the field means: what a role link SAYS is extensible,
    // where it is stored is not.
    fn projects_role_link(d: serde_json::Value, to: String, name: String, role: String, now: u64) -> serde_json::Value {
        let _ = d;
        serde_json::json!({
            "kind": "role", "to": to, "name": name,
            "role": role, "t": now })
    }

    // a name to a world key, off the guest list — the one lookup this node
    // does. A link names a person, and a person may not be a user at all; an
    // empty answer means "there is no world to hand to", which is not an
    // error.
    fn projects_key_for_name(name: String) -> String {
        if name.is_empty() {
            return String::new();
        }
        let raw = std::fs::read_to_string(format!("{}/users.json", auth_dir()))
            .unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for u in v.as_array().unwrap_or(&empty) {
            if u["name"].as_str().unwrap_or("") != name {
                continue;
            }
            let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if !p.is_empty() {
                return format!("phone:{}", p);
            }
        }
        String::new()
    }

    // ---- and only to them ---------------------------------------------------
    // /exchange's rung one gives every card you own to everybody your invite
    // links reach. That is right for a profile — being invited is what makes
    // you visible — and wrong for a project, which is not about the people
    // who happen to have invited you. So the door is narrowed for one type of
    // card and left exactly as it was for every other.
    //
    // This is also what makes the surface honest: a project you make is not
    // on your invitees' phones until you put them in it.

    fn exchange_give(to: String, cards: Vec<serde_json::Value>) {
        let mut keep: Vec<serde_json::Value> = Vec::new();
        for c in cards.iter() {
            if c["type"].as_str().unwrap_or("") != "project" {
                keep.push(c.clone());
                continue;
            }
            if projects_is_member(c, to.clone()) {
                keep.push(c.clone());
            }
        }
        existing.exchange_give(to, keep);
    }

    // is this world in this project? The link's `to` carries the owner name
    // in front of the dot, which is the only thing a card and a guest-list
    // entry share — a world key is a phone number and never leaves the box.
    fn projects_is_member(card: &serde_json::Value, key: String) -> bool {
        let name = exchange_name_of(key);
        if name.is_empty() {
            return false;
        }
        for l in projects_members(card).iter() {
            if projects_link_name(l) == name {
                return true;
            }
            if l["name"].as_str().unwrap_or("") == name {
                return true;
            }
        }
        false
    }

    // ---- the glyphs ---------------------------------------------------------
    // drawn, in currentColor, per /glyphs: black on /ember's tint and white
    // on the plain button, with no filter working to correct an asset.

    fn projects_flag_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M6 21V4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>",
            "<path d=\"M6 4h11l-2.6 4.2L17 12.5H6z\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }

    fn projects_plus_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M12 5v14M5 12h14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.6\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}

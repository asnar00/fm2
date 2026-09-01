struct feature_Audience;
impl feature_Audience {
    // ---- the ladder ---------------------------------------------------------
    // six words in one order, highest first. Everything below compares two
    // NUMBERS: "the same or higher rank" is the lower or equal number, and a
    // word is never compared with a word.
    //
    // This is not /authority's ladder. That one (admin / support / member)
    // says what you may do to the app; this one says where you stand inside
    // one project. Nothing here reads /authority and nothing there reads this.

    fn audience_grades() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        out.push("admin".to_string());
        out.push("candidate".to_string());
        out.push("team".to_string());
        out.push("volunteer".to_string());
        out.push("supporter".to_string());
        out.push("public".to_string());
        out
    }

    fn audience_default_grade() -> String {
        "team".to_string()
    }

    // an unknown word ranks as the default rather than throwing: a hand-made
    // op cannot invent a rank it did not have.
    fn audience_rank(grade: String) -> usize {
        let mut at = 0usize;
        for g in audience_grades().iter() {
            if g == &grade {
                return at;
            }
            at += 1;
        }
        audience_rank_of_default()
    }

    fn audience_rank_of_default() -> usize {
        let d = audience_default_grade();
        let mut at = 0usize;
        for g in audience_grades().iter() {
            if g == &d {
                return at;
            }
            at += 1;
        }
        0
    }

    fn audience_grade_at(rank: usize) -> String {
        let all = audience_grades();
        if rank < all.len() {
            return all[rank].clone();
        }
        all[all.len() - 1].clone()
    }

    fn audience_is_grade(word: String) -> bool {
        for g in audience_grades().iter() {
            if g == &word {
                return true;
            }
        }
        false
    }

    // ---- saying it in words -------------------------------------------------
    // the plain kind (/taste 7): a sentence a person reads, not a field name.

    fn audience_words(grade: String) -> String {
        match grade.as_str() {
            "admin" => "admins".to_string(),
            "candidate" => "candidates".to_string(),
            "team" => "the team".to_string(),
            "volunteer" => "volunteers".to_string(),
            "supporter" => "supporters".to_string(),
            _ => "everyone".to_string(),
        }
    }

    fn audience_line(grade: String) -> String {
        if grade == "public" {
            return "visible to everyone in the project".to_string();
        }
        format!("visible to {} and up", audience_words(grade))
    }

    // ---- reading a card -----------------------------------------------------

    // the project a post was filed in — /cards' reserved `in` link — or empty
    // for a post that belongs to no project.
    fn audience_in_of(card: &serde_json::Value) -> String {
        if !posts_is(card) {
            return String::new();
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        for l in card["links"].as_array().unwrap_or(&empty) {
            if l["kind"].as_str().unwrap_or("") != "in" {
                continue;
            }
            let to = l["to"].as_str().unwrap_or("").to_string();
            if !to.is_empty() {
                return to;
            }
        }
        String::new()
    }

    // the lowest rank this post reaches. Absent is the default grade, so a
    // post minted before this node existed reads as team without a rewrite.
    fn audience_floor_of(card: &serde_json::Value) -> String {
        let f = card["floor"].as_str().unwrap_or("").to_string();
        if audience_is_grade(f.clone()) {
            return f;
        }
        audience_default_grade()
    }

    // where one person stands in one project, or empty for "not in it". The
    // owner is admin by being the owner — there is no role link to yourself
    // and none is invented. A role link with no grade is the default, which
    // is what makes every role written before today a team role.
    fn audience_grade_in(proj: &serde_json::Value, name: String) -> String {
        if name.is_empty() {
            return String::new();
        }
        if proj["owner"].as_str().unwrap_or("") == name {
            return "admin".to_string();
        }
        for l in projects_members(proj).iter() {
            let hit = projects_link_name(l) == name
                || l["name"].as_str().unwrap_or("") == name;
            if !hit {
                continue;
            }
            let g = l["grade"].as_str().unwrap_or("").to_string();
            if audience_is_grade(g.clone()) {
                return g;
            }
            return audience_default_grade();
        }
        String::new()
    }

    // everyone named on a project: its owner and every role link. The list a
    // "who saw this" surface would ask for (/anticipation) — not built.
    fn audience_people_of(proj: &serde_json::Value) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let owner = proj["owner"].as_str().unwrap_or("").to_string();
        if !owner.is_empty() {
            out.push(owner);
        }
        for l in projects_members(proj).iter() {
            let n = projects_link_name(l);
            if n.is_empty() {
                continue;
            }
            let mut seen = false;
            for o in out.iter() {
                if o == &n {
                    seen = true;
                }
            }
            if !seen {
                out.push(n);
            }
        }
        out
    }

    // one project card out of a cards list, by id. Null for "that world does
    // not hold it", which is the whole of "you are not in this project".
    fn audience_project_in(list: String, pid: String) -> serde_json::Value {
        if pid.is_empty() {
            return serde_json::Value::Null;
        }
        let v: serde_json::Value = serde_json::from_str(&list)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") == pid
                && c["type"].as_str().unwrap_or("") == "project"
                && !delete_gone(c) {
                return c.clone();
            }
        }
        serde_json::Value::Null
    }

    fn audience_card_by_id(id: String) -> serde_json::Value {
        let v: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") == id {
                return c.clone();
            }
        }
        serde_json::Value::Null
    }

    // ---- the stamp ----------------------------------------------------------
    // /cards' `card_new` is the ONE door every capture road goes through —
    // typed, photo, video, audio, and /as-posts' recording, which builds its
    // card with it too. So the filing is done here once and no road carries a
    // copy of it. Only a post, and only with a project actually held: a
    // project you no longer hold cannot file anything.

    fn card_new(owner: String, kind: String, now: u64) -> serde_json::Value {
        let mut card = existing.card_new(owner.clone(), kind.clone(), now);
        if kind != "post" {
            return card;
        }
        let pid = current_project_read();
        if pid.is_empty() {
            return card;
        }
        let proj = audience_project_in(cards_read(), pid.clone());
        if proj.is_null() {
            return card;
        }
        let grade = audience_grade_in(&proj, owner);
        if grade.is_empty() {
            // a project you hold but are not on: nothing to file it under and
            // no floor to give it. It stays a post that belongs to no project.
            return card;
        }
        card["links"] = serde_json::json!([
            { "kind": "in", "to": pid, "t": now }
        ]);
        card["floor"] = serde_json::json!(grade);
        card
    }

    // ---- and only when that project is chosen -------------------------------
    // /current-project narrows the posts to those RELATED to the chosen one —
    // filed in it, or written by anybody in it. That was the right rule while
    // nothing filed anything; the ask is an equality. Outside its link, so it
    // tightens rather than replaces: the map follows for nothing, since
    // /browse hands `browse_set_html` this same set.
    fn posts_set() -> Vec<serde_json::Value> {
        let all = existing.posts_set();
        let sel = current_project_read();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in all.iter() {
            if audience_in_of(c) == sel {
                out.push(c.clone());
            }
        }
        out
    }

    // ---- promote ------------------------------------------------------------
    // one rung down the floor is one rung up the audience. Only the author:
    // a copy carries `from` (/exchange), which is the structural test /delete
    // set — never a comparison against the logged-in name, which is not in
    // the world at all.

    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "PostPromote" {
            return existing.update(state, event);
        }
        // the pre-event world, taken at the top of the link — the same instant
        // /undo takes its own. See `audience_record`.
        let before = with_context(|c| c.snapshot());
        let tool = open_tool_read();
        let state = existing.update(state, event.clone());
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        if id.is_empty() || now == 0 {
            return state;
        }
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return state;
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            if !c["from"].is_null() {
                println!("audience: refused a promote of {} — that post is a copy", id);
                continue;
            }
            if !posts_is(c) || audience_in_of(c).is_empty() {
                continue;
            }
            let was = audience_rank(audience_floor_of(c));
            let last = audience_grades().len() - 1;
            if was >= last {
                continue;   // already everyone in the project: not a write
            }
            c["floor"] = serde_json::json!(audience_grade_at(was + 1));
            c["edited"] = serde_json::json!(now);
            changed = true;
        }
        if !changed {
            return state;
        }
        cards_write(list.to_string());
        audience_record(before, tool);
        state
    }

    // /undo scans the outbox for what a turn wrote and /undo/late moved that
    // scan to the end of the chain; this node is newer, so its write lands
    // after the scan and undo would never see it. /delete met this first and
    // files its own step the same way — the general fix is /late's own named
    // rung, not this ask's.
    fn audience_record(before: serde_json::Value, tool: String) {
        if tool.is_empty() {
            return;
        }
        let rec = undo_var_before(before, "miso/loop/cards".to_string(),
                                  "cards".to_string());
        if rec.is_null() {
            return;
        }
        undo_push(serde_json::json!({
            "tool": tool,
            "changes": [rec]
        }));
    }

    // ---- the control --------------------------------------------------------
    // on your own post that is in a project and is not already everyone's. In
    // front of /undo's button through /posts' own inserter (/glyphs — undo is
    // last in every row), wearing the posts tool's colour.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state);
        if open_tool_read() != "posts" {
            return row;
        }
        let open = browse_open_read();
        if open.is_empty() {
            return row;
        }
        let c = audience_card_by_id(open);
        if c.is_null() || !posts_is(&c) || !c["from"].is_null() {
            return row;
        }
        if audience_in_of(&c).is_empty() {
            return row;
        }
        if audience_floor_of(&c) == "public" {
            return row;
        }
        posts_before_undo(row, audience_button(&c))
    }

    fn audience_button(card: &serde_json::Value) -> String {
        let colour = tool_colour("posts".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        let next = audience_grade_at(audience_rank(audience_floor_of(card)) + 1);
        format!("<div class=\"tool-button ctrl{}\" data-ev=\"posts_promote\" title=\"{}\">{}</div>",
                tint, card_esc(audience_line(next)), audience_arrow_svg())
    }

    // drawn, in currentColor, per /glyphs: an arrow raised to a line — up, and
    // as far as this tap takes it. Never a character with an emoji
    // presentation.
    fn audience_arrow_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M5 4.5h14\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>",
            "<path d=\"M12 20V9\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>",
            "<path d=\"M7.2 13.6L12 8.8l4.8 4.8\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.4\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }

    // ---- the one quiet line -------------------------------------------------
    // under your own post, saying the rung plainly. Spliced INSIDE the page's
    // scrolling box (/projects' `projects_inside`) — appended after it, the
    // box is `position: fixed` and the line lands off-screen.
    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        if !posts_is(&c) || !c["from"].is_null() {
            return html;
        }
        if audience_in_of(&c).is_empty() {
            return html;
        }
        let line = format!("<div class=\"card-audience\">{}</div>",
                           card_esc(audience_line(audience_floor_of(&c))));
        projects_inside(html, line)
    }

    // ---- the second lane ----------------------------------------------------
    // /exchange already carries a write into every linked world the moment it
    // lands, and /converge repaints their open pages. Nothing new is
    // transported here. What is added is one audience — the project — and one
    // gate, on the door every road already goes through.

    // the gate. `exchange_give` is THE way a card enters another world: both
    // lanes and the login-time seeding call it, so a rule stated here cannot
    // be gone round. A post filed in a project reaches a world only if that
    // world holds the project and holds it at or above the post's floor;
    // every other card is /exchange's business and passes untouched.
    fn exchange_give(to: String, cards: Vec<serde_json::Value>) {
        let mut keep: Vec<serde_json::Value> = Vec::new();
        for c in cards.iter() {
            if audience_in_of(c).is_empty() {
                keep.push(c.clone());
                continue;
            }
            if audience_may_hold(c, to.clone()) {
                keep.push(c.clone());
            }
        }
        existing.exchange_give(to, keep);
    }

    // asked of the RECIPIENT's own world, not of the sender's belief: they
    // hold the project card because /projects hands a project to everyone in
    // it, and their copy carries the role links. A world that holds no such
    // card is not in the project and is refused with no special case — which
    // is also what makes a forged `in` link inert.
    fn audience_may_hold(card: &serde_json::Value, to: String) -> bool {
        let name = exchange_name_of(to.clone());
        if name.is_empty() {
            return false;
        }
        let proj = audience_project_in(exchange_cards_of(to), audience_in_of(card));
        if proj.is_null() {
            return false;
        }
        let grade = audience_grade_in(&proj, name);
        if grade.is_empty() {
            return false;
        }
        audience_rank(grade) <= audience_rank(audience_floor_of(card))
    }

    // the lane itself: after /exchange has walked the invite tree, walk the
    // project. This is what reaches a project-mate who never invited you and
    // whom you never invited — bob and carol, of ash's two invitees.
    fn exchange_share(who: String, before: String, after: String) {
        existing.exchange_share(who.clone(), before.clone(), after.clone());
        audience_hand(who, before, after);
    }

    // every post of the writer's that changed and carries an `in` link goes to
    // everyone named on that project. The diff is /exchange's own: a card that
    // was not there before counts as changed, which is exactly the write that
    // has to travel — a post's first save. The gate above decides who keeps
    // it, so this pass names the audience and judges nobody.
    fn audience_hand(who: String, before: String, after: String) {
        let me = exchange_name_of(who.clone());
        let old: serde_json::Value = serde_json::from_str(&before)
            .unwrap_or(serde_json::json!([]));
        let new: serde_json::Value = serde_json::from_str(&after)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut sent = 0usize;
        for c in new.as_array().unwrap_or(&empty) {
            if !posts_is(c) {
                continue;
            }
            if !c["from"].is_null() {
                continue;   // a copy is not yours to hand on
            }
            if !me.is_empty() && c["owner"].as_str().unwrap_or("") != me {
                continue;
            }
            if !exchange_owns_id(c, me.clone()) {
                continue;   // an id you did not mint is not a card you own
            }
            let pid = audience_in_of(c);
            if pid.is_empty() {
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
            let proj = audience_project_in(after.clone(), pid);
            if proj.is_null() {
                continue;
            }
            let copy = exchange_copy(c, me.clone(), who.clone());
            for name in audience_people_of(&proj).iter() {
                let key = projects_key_for_name(name.clone());
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
            println!("audience: {} offered a post to {} project world(s)", me, sent);
        }
    }

    // ---- the grade on a role ------------------------------------------------
    // /projects cut two seams for this (agents.md's refactor rule): the link a
    // RoleAdd writes, and the role cell of a row on the project page. Neither
    // changed what it returned.

    fn projects_role_link(d: serde_json::Value, to: String, name: String, role: String, now: u64) -> serde_json::Value {
        let mut l = existing.projects_role_link(d.clone(), to, name, role, now);
        let g = d["grade"].as_str().unwrap_or("").to_string();
        if audience_is_grade(g.clone()) {
            l["grade"] = serde_json::json!(g);
        }
        l
    }

    // the grade under the role word, on the project page only — the words
    // still say what somebody does, and the grade is a quieter second line
    // (/taste 2). Never on a person's own card: nobody asked for a badge on a
    // person, and the role lines there read the link themselves.
    fn projects_people_role(l: &serde_json::Value) -> String {
        let base = existing.projects_people_role(l);
        let g = l["grade"].as_str().unwrap_or("").to_string();
        let g = if audience_is_grade(g.clone()) { g } else { audience_default_grade() };
        format!("{}<span class=\"proj-grade\">{}</span>", base, card_esc(g))
    }
}

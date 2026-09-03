struct feature_CoMembers;
impl feature_CoMembers {
    // who receives this person's cards and whose cards seed them: the base's
    // invite links, plus everyone with a role in a project this person holds
    fn exchange_links(key: String) -> Vec<String> {
        let mut out = existing.exchange_links(key.clone());
        for k in co_members_of(key.clone()).iter() {
            if k == &key || out.contains(k) {
                continue;
            }
            out.push(k.clone());
        }
        out
    }

    // the world keys named by role links on every project card in this
    // person's world, own or copy
    fn co_members_of(key: String) -> Vec<String> {
        let raw = if key == context_user_now() {
            cards_read()
        } else {
            exchange_cards_of(key.clone())
        };
        let held: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<String> = Vec::new();
        for c in held.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "project" {
                continue;
            }
            for l in projects_members(c).iter() {
                let k = projects_key_for_name(projects_link_name(l));
                if k.is_empty() || k == key || out.contains(&k) {
                    continue;
                }
                out.push(k);
            }
        }
        out
    }

    // a person's own profile cards, as copies for somebody else
    fn co_members_profiles(key: String) -> Vec<serde_json::Value> {
        let name = exchange_name_of(key.clone());
        let held: serde_json::Value = serde_json::from_str(&exchange_cards_of(key.clone()))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in held.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "profile" || !c["from"].is_null() {
                continue;
            }
            if name.is_empty() || c["owner"].as_str().unwrap_or("") != name {
                continue;
            }
            if !exchange_owns_id(c, name.clone()) {
                continue;
            }
            out.push(exchange_copy(c, name.clone(), key.clone()));
        }
        out
    }

    // the base seeds from the inviter; every other link's profile follows
    fn exchange_seed(key: String) {
        existing.exchange_seed(key.clone());
        let mut n = 0usize;
        for k in exchange_links(key.clone()).iter() {
            let theirs = co_members_profiles(k.clone());
            if theirs.is_empty() {
                continue;
            }
            n = n + theirs.len();
            exchange_give(key.clone(), theirs);
        }
        if n > 0 {
            println!("co-members: {} seeded with {} profile(s)", tag(key), n);
        }
    }

    // the moment of joining: the newcomer sees everyone, and everyone sees
    // the newcomer — nobody else has to write anything
    fn invited_into_stamp(who: String) {
        existing.invited_into_stamp(who.clone());
        exchange_seed(who.clone());
        let mine = co_members_profiles(who.clone());
        if mine.is_empty() {
            return;
        }
        let links = exchange_links(who.clone());
        for k in links.iter() {
            exchange_give(k.clone(), mine.clone());
        }
        println!("co-members: {}'s profile handed to {} person(s)", tag(who), links.len());
    }
}

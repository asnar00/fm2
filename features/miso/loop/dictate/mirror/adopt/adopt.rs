struct feature_Adopt;
impl feature_Adopt {
    // the two doors into a user's blob namespace: the /blob route (which asks
    // blob_user for the directory) and the RecShared/RecIndex messages (which
    // use the stamped identity). Both adopt before anything reads or writes, so
    // a returning user finds their recordings wherever they touch first.
    fn blob_user(cookie: String, tunnel: bool) -> String {
        let who = existing.blob_user(cookie, tunnel);
        blob_adopt(who.clone());
        who
    }

    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        let t = m["type"].as_str().unwrap_or("").to_string();
        if t == "RecShared" || t == "RecIndex" {
            blob_adopt(m["_from"].as_str().unwrap_or("").to_string());
        }
        existing.handle_msg(msg)
    }

    // one legacy directory, claimed once. The rename is the claim: it is atomic,
    // so two colliding users racing produce one winner and one fresh namespace,
    // and it leaves nothing for a third touch to find.
    fn blob_adopt(who: String) {
        let phone = whole_number_of(who.clone());
        if phone.is_empty() {
            return;   // no session, or a key that never had a legacy form
        }
        let legacy = format!("{}/{}", blob_root(), tag(phone.clone()));
        if !std::path::Path::new(&legacy).exists() {
            return;   // the common case: nothing to migrate
        }
        let mine = format!("{}/{}", blob_root(), who);
        if std::path::Path::new(&mine).exists() {
            announce_forfeit(phone);
            return;
        }
        match std::fs::rename(&legacy, &mine) {
            Ok(()) => announce_adoption(phone),
            Err(e) => eprintln!("miso: blob namespace {} could not be adopted \
                                 ({}) — recordings stay where they are",
                                tag(phone), e),
        }
    }

    // the identity is `phone:+<digits>`; only that shape ever had a four-digit
    // directory. `_local` and the empty identity are left alone.
    fn whole_number_of(who: String) -> String {
        match who.strip_prefix("phone:") {
            Some(p) if p.len() > 4 => p.to_string(),
            _ => String::new(),
        }
    }

    // loud, and honest about what cannot be known: a tag-keyed directory may
    // hold two people's recordings and nothing in it says which are whose.
    fn announce_adoption(phone: String) {
        eprintln!("miso: BLOB MIGRATION — {} ({}) has adopted the old \
                   four-digit recording store {}. If another guest's number \
                   ends in the same four digits, some of those recordings may \
                   be theirs; they cannot be told apart automatically.",
                  guest_name(phone.clone()), tag(phone.clone()), tag(phone));
    }

    // the second claimant: already has a whole-number store, so the legacy one
    // is somebody else's and stays put.
    fn announce_forfeit(phone: String) {
        if migration_announced(format!("forfeit:{}", phone)) {
            return;   // once per user per run: this case repeats on every touch
        }
        eprintln!("miso: BLOB MIGRATION — {} ({}) already has its own \
                   recording store, so the old four-digit store {} was left \
                   for its first claimant.",
                  guest_name(phone.clone()), tag(phone.clone()), tag(phone));
    }

    // a repeating condition says itself once per run, so a store that is never
    // claimed does not fill the log one request at a time.
    fn migration_announced(key: String) -> bool {
        static SEEN: std::sync::OnceLock<std::sync::Mutex<std::collections::HashSet<String>>>
            = std::sync::OnceLock::new();
        let seen = SEEN.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()));
        let mut held = seen.lock().unwrap_or_else(|p| p.into_inner());
        !held.insert(key)
    }

    // a name is what an operator can act on; the number is not printed.
    fn guest_name(phone: String) -> String {
        let name = find_user(phone);
        if name.is_empty() { "an unlisted guest".to_string() } else { name }
    }
}

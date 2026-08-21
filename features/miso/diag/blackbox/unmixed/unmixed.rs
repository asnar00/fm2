struct feature_Unmixed;
impl feature_Unmixed {
    // an event stream is labelled with the person it came from, not with four
    // digits of their phone number. The label is two halves, joined by a colon
    // and carrying no space, because the log's reader splits on the first two:
    // a name an operator can act on, and the collision-free id the relay is
    // already addressed by.
    fn blackbox_who(cookie: String) -> String {
        let who = sender_of(cookie.clone());
        if who.is_empty() {
            return existing.blackbox_who(cookie);
        }
        format!("{}:{}", stream_name(who.clone()), stream_id(who))
    }

    // the guest list's name for this identity, reduced to one word. An identity
    // that is not on the list any more still gets a stable label from its id.
    fn stream_name(who: String) -> String {
        let phone = who.strip_prefix("phone:").unwrap_or("").to_string();
        let name = find_user(phone);
        if name.is_empty() {
            return "guest".to_string();
        }
        let mut out = String::new();
        for c in name.to_lowercase().chars() {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                out.push(c);
            } else {
                out.push('-');
            }
        }
        out
    }

    // the same opaque per-user id the relay is addressed by, shortened: this
    // log is a debugging aid on a shared machine, and 48 bits is far past the
    // guest list's chance of a collision without putting a phone number in it.
    fn stream_id(who: String) -> String {
        sender_audience(who).chars().take(12).collect()
    }
}

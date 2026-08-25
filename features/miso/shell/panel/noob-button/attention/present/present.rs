struct feature_Present;
impl feature_Present {
    // a wait's owner as a world key, `phone:<number>` — the same key
    // /per-user gives a cookie-bearing request, so it matches the owner
    // /attention pushes to. Localhost tooling has no cookie and no key.
    fn presence_key(cookie: String) -> String {
        let t = cookie_token(cookie);
        if !t.is_empty() && token_valid(t.clone()) {
            format!("phone:{}", token_phone(t))
        } else {
            String::new()
        }
    }

    // every long-poll marks its user present on the way in and on the way
    // out: a page re-waits the moment a wait returns, so a present user's
    // mark is never older than one wait cycle (~25s).
    fn msg_wait(r: request) -> response {
        let key = presence_key(r.cookie.clone());
        presence_touch(key.clone());
        let resp = existing.msg_wait(r);
        presence_touch(key);
        resp
    }

    // the focused rung of the attention rule, decided where the wire starts:
    // a user whose page is listening gets the screen update the relay already
    // gives them, and no notification at all.
    fn attention_push_to_user(owner: String, body: String) {
        let who = tag(owner.trim_start_matches("phone:").to_string());
        if presence_recent(owner.clone(), 30000) {
            println!("attention: {} is present — screen only, no notification", who);
            return;
        }
        println!("attention: {} is away — notifying", who);
        existing.attention_push_to_user(owner, body)
    }
}

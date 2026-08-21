struct feature_PerUser;
impl feature_PerUser {
    // the storage seam, redefined rather than edited: rung 2's held_context()
    // is still the process's single world, and it is what an empty identity
    // reads — startup, and the whole wasm place, which never sets one. With an
    // identity, the table answers instead. Every caller of held_context() —
    // both accessors and all the gates — is untouched, because the signature
    // is untouched.
    fn held_context() -> &'static std::sync::RwLock<Context> {
        let who = context_user_now();
        if who.is_empty() {
            return existing.held_context();
        }
        context_of(&who)
    }

    // identity is established before anything routes, and cleared after, so a
    // pooled or reused thread can never inherit the last caller's world. This
    // link is outermost, which puts it outside rung 3's turn boundary: the
    // turn that `route` opens therefore freezes THE REQUESTER'S context.
    fn route(r: request) -> response {
        let who = context_user_of(r.cookie.clone(), r.tunnel, r.query.clone());
        if who.is_empty() && !r.tunnel && r.path == "diag/context" {
            return json_response(400, "{\"ok\":false,\"error\":\"?user= must be 1-64 chars of a-z A-Z 0-9 . _ -\"}".to_string());
        }
        context_user_set(who);
        let resp = existing.route(r);
        context_user_clear();
        resp
    }

    // whose world this request touches. The cookie is asked first and always
    // wins: a logged-in caller cannot name someone else by parameter, through
    // the tunnel or on localhost. Only when there is no valid session does the
    // localhost tooling path get to choose, and what it chooses lands in a
    // separate `local:` namespace, so no parameter can ever spell a real
    // user's key.
    fn context_user_of(cookie: String, tunnel: bool, query: String) -> String {
        let token = cookie_token(cookie);
        if !token.is_empty() && token_valid(token.clone()) {
            return format!("phone:{}", token_phone(token));
        }
        if tunnel {
            // an unauthenticated tunnel caller has no world of its own; the
            // context routes reject it before it can read one anyway.
            return String::new();
        }
        let name = query_param(query, "user".to_string());
        if name.is_empty() {
            return "local:_local".to_string();
        }
        if !context_user_name_ok(&name) {
            return String::new();
        }
        format!("local:{}", name)
    }

    // a tooling user name is bounded and plain. The key never reaches the
    // filesystem — it addresses a HashMap entry — so traversal shapes are not
    // dangerous, merely nonsense; the bound is here to stop the table being
    // grown by garbage, and to keep an error message readable.
    fn context_user_name_ok(name: &String) -> bool {
        !name.is_empty() && name.len() <= 64
            && name.chars().all(|c| c.is_ascii_alphanumeric()
                                || c == '-' || c == '.' || c == '_')
    }

    // one parameter out of a raw query string. No percent-decoding: the names
    // this rung accepts have no character that needs it, and decoding would be
    // a second parser to keep honest.
    fn query_param(query: String, name: String) -> String {
        for part in query.split('&') {
            let mut kv = part.splitn(2, '=');
            if kv.next().unwrap_or("") == name {
                return kv.next().unwrap_or("").to_string();
            }
        }
        String::new()
    }
}

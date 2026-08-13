struct feature_Gate;
impl feature_Gate {
    // extends the serve route chain: auth endpoints answer everywhere; the app
    // SHELL is public (it's just code — gating it froze logged-out installed
    // PWAs solid: their service workers could never fetch updates, since the
    // sw only caches 2xx); DATA routes through the tunnel need the cookie.
    // local/LAN traffic stays frictionless. the shell decides to show login
    // by asking auth/whoami.
    fn route(r: request) -> response {
        if r.path == "auth/request" && r.method == "POST" {
            return auth_request(r);
        }
        if r.path == "auth/verify" && r.method == "POST" {
            return auth_verify(r);
        }
        if r.path == "auth/whoami" {
            return auth_whoami(r);
        }
        if r.path == "auth/logout" && r.method == "POST" {
            return auth_logout(r);
        }
        if is_public(r.path.clone()) {
            return existing.route(r);
        }
        if !r.tunnel {
            return existing.route(r);
        }
        if authed(r.cookie.clone()) {
            return existing.route(r);
        }
        login_page()
    }

    // base: nothing is public. the /public subfeature owns the actual policy
    // (shell + feature tree) as an extension of this chain.
    fn is_public(path: String) -> bool {
        let _ = path;
        false
    }

    fn login_page() -> response {
        let html = std::fs::read("site/login.html").unwrap_or_default();
        // no-store: Safari happily reuses a cached 401 after a successful
        // login, showing the login page to someone who just got a cookie
        response { status: 401, ctype: "text/html; charset=utf-8".to_string(),
                   body: html, set_cookie: String::new(),
                   cache: "no-store, must-revalidate".to_string() }
    }
}

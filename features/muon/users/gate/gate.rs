struct feature_Gate;
impl feature_Gate {
    // extends the serve route chain: auth endpoints answer everywhere; traffic
    // through the tunnel (cf-connecting-ip) needs the cookie or gets the login
    // page; local/LAN traffic stays frictionless.
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
        if !r.tunnel {
            return existing.route(r);
        }
        if authed(r.cookie.clone()) {
            return existing.route(r);
        }
        login_page()
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

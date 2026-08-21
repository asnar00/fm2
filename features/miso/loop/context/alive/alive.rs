struct feature_Alive;
impl feature_Alive {
    // the place's one Context, constructed on first ask and held for the life
    // of the process. `static` inside a fn body because the composition
    // machinery only carries fns; OnceLock rather than thread_local! because
    // the same composed body is compiled for both places, and OnceLock is
    // correct in the native server and free in the single-threaded wasm client.
    fn held_context() -> &'static Context {
        static HELD: std::sync::OnceLock<Context> = std::sync::OnceLock::new();
        HELD.get_or_init(Context::fresh)
    }

    // server startup: build the Context before the accept loop begins, so a
    // running server always has one whether or not a request has arrived.
    fn serve() {
        let _ = held_context();
        existing.serve();
    }

    // client startup: the wasm place holds a Context too, and exposes nothing
    // yet — no UI, no readout extension this rung.
    fn boot() -> String {
        let _ = held_context();
        existing.boot()
    }

    fn route(r: request) -> response {
        if r.path == "diag/context" {
            return context_get(r);
        }
        existing.route(r)
    }

    // fm:context-snapshot — the linker's hook: this token in a composed node's
    // source is what asks emit_context() for Context::snapshot(). Untick this
    // node and the walker is not emitted at all.
    fn context_get(r: request) -> response {
        // a context carries user-scoped state, so it is screened exactly as
        // diag/readout screens: open on localhost for tooling, cookie-gated
        // through the tunnel.
        if r.tunnel && !authed(r.cookie.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        json_response(200, held_context().snapshot().to_string())
    }
}

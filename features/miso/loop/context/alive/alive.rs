struct feature_Alive;
impl feature_Alive {
    // the place's one Context, constructed on first ask and held for the life
    // of the process. `static` inside a fn body because the composition
    // machinery only carries fns; OnceLock rather than thread_local! because
    // the same composed body is compiled for both places, and OnceLock is
    // correct in the native server and free in the single-threaded wasm client.
    // The RwLock inside it is the seam a later rung writes through — the cell
    // is still constructed once and held forever; only its contents may move.
    //
    // The cell is COUNTED (`Arc`), and every caller takes a handle by value
    // rather than a reference with the process's lifetime. This one is held
    // forever by its OnceLock, which is the point: this is the world an empty
    // identity reads. The per-user cells beside it are the ones that have to be
    // droppable, and a handle is what makes dropping them possible.
    fn held_context() -> std::sync::Arc<std::sync::RwLock<Context>> {
        static HELD: std::sync::OnceLock<std::sync::Arc<std::sync::RwLock<Context>>> =
            std::sync::OnceLock::new();
        HELD.get_or_init(|| std::sync::Arc::new(
            std::sync::RwLock::new(Context::fresh()))).clone()
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
        json_response(200, context_snapshot_json())
    }

    // the snapshot, rendered from the live Context. A separate function so the
    // read path is a chain a later rung can extend (fm.md's refactoring rule:
    // behaviour intact, extension point extracted).
    fn context_snapshot_json() -> String {
        // the handle is bound, and so is the answer: a guard taken in the
        // tail expression would outlive the handle it borrows from.
        let cell = held_context();
        let snapshot = match cell.read() {
            Ok(c) => c.snapshot().to_string(),
            Err(p) => p.into_inner().snapshot().to_string(),
        };
        snapshot
    }
}

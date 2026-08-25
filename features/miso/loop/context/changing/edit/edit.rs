struct feature_Edit;
impl feature_Edit {
    // the server's turn boundary: one request is one turn. The turn opens
    // before anything routes and closes after the response is built, so every
    // read inside a request sees the context the request arrived under, and an
    // edit — this request's own, or another thread's — is first visible to the
    // next request. This is the outermost link of the route chain because this
    // node is the newest in the composition; a future node wrapping route with
    // a newer anchor would sit outside the boundary (edit.md names this risk).
    fn route(r: request) -> response {
        context_turn_begin();
        if r.path == "diag/context" && r.method == "POST" {
            let resp = context_set(r);
            context_turn_end();
            return resp;
        }
        let resp = existing.route(r);
        context_turn_end();
        resp
    }

    // the client's turn boundary: one event is one turn, matching the Elm
    // update the loop already runs. Nothing here changes what the turn does.
    //
    // Between the event and the close there is now one named moment,
    // `context_turn_close`. It is the only place in the composition that is
    // guaranteed to run after EVERY link of the update chain, whatever its
    // provenance, because the update chain is nested inside this one by
    // construction rather than by position. The base below is the identity,
    // so with nothing extending it this link is what it always was.
    fn on_event(input: String) -> String {
        context_turn_begin();
        let out = existing.on_event(input);
        let out = context_turn_close(out);
        context_turn_end();
        out
    }

    // the end-of-turn seam: the event's `{state, html}` payload on its way out.
    // /turn-end extends this; on its own it changes nothing.
    fn context_turn_close(out: String) -> String {
        out
    }

    // the snapshot read joins the boundary: inside a turn it reports the
    // frozen view, so GET diag/context tells the truth about what the request
    // is running under. Outside a turn the chain beneath still answers.
    fn context_snapshot_json() -> String {
        if in_context_turn() {
            return with_context(|c| c.snapshot().to_string());
        }
        existing.context_snapshot_json()
    }

    // fm:context-set — the linker's hook: this token in a composed node's
    // source is what asks emit_context() for Context::set_from_json() and the
    // Clone impl the turn freeze needs. Untick this node and neither is
    // emitted, and no var type has to be Deserialize or Clone.
    //
    // POST diag/context — screened exactly as the GET is: open on localhost
    // for tooling, cookie-gated through the tunnel.
    fn context_set(r: request) -> response {
        if r.tunnel && !authed(r.cookie.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let parsed: Result<serde_json::Value, serde_json::Error> =
            serde_json::from_str(&r.body);
        let body = match parsed {
            Ok(v) => v,
            Err(e) => return context_edit_error(400, format!("body is not JSON: {}", e)),
        };
        let path = body["path"].as_str().unwrap_or("").to_string();
        let name = body["name"].as_str().unwrap_or("").to_string();
        if path.is_empty() || name.is_empty() {
            return context_edit_error(400,
                "expected a body of {\"path\": .., \"name\": .., \"value\": ..}".to_string());
        }
        if body.get("value").is_none() {
            return context_edit_error(400,
                format!("no \"value\" given for {}/{}", path, name));
        }
        let value = body["value"].clone();
        let outcome = edit_context(|c| c.set_from_json(&path, &name, value.clone()));
        match outcome {
            Ok(_) => json_response(200, "{\"ok\":true}".to_string()),
            Err(e) => context_edit_error(400, e),
        }
    }

    fn context_edit_error(status: u16, msg: String) -> response {
        json_response(status,
            serde_json::json!({ "ok": false, "error": msg }).to_string())
    }
}

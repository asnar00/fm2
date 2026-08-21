struct feature_TurnEnd;
impl feature_TurnEnd {
    // fm:turn-end-phase — the linker's ninth hook, and the only one that asks for
    // nothing to be emitted. It exists so that a node which has handed its
    // end-of-turn work over (fm:turn-end-required) cannot be composed without
    // this one: every other dependency in this family fails in rustc, and this
    // one would fail in silence.
    //
    // the turn's last word. `/edit` calls this after the whole event — every
    // link of the update chain, then the paint — and before the freeze is
    // dropped, so it is the one moment guaranteed to come after every update
    // link no matter who wrote it or when. The work that used to sit at
    // /converge's and /overlay's own links happens here instead: the same two
    // functions, called from a place whose position is structural rather than
    // provenance-ordered.
    //
    // A turn that minted nothing pays one integer read and returns the payload
    // it was handed, untouched.
    fn context_turn_close(out: String) -> String {
        let out = existing.context_turn_close(out);
        if context_op_pending() == 0 {
            return out;
        }
        let mut p: serde_json::Value = serde_json::from_str(&out)
            .unwrap_or(serde_json::Value::Null);
        if !p["state"].is_string() {
            return out;   // not a loop payload: leave it exactly as it came
        }
        let state = p["state"].as_str().unwrap_or("{}").to_string();
        p["state"] = serde_json::json!(ctx_stamp_outbox(ctx_ship_ops(state)));
        p.to_string()
    }
}

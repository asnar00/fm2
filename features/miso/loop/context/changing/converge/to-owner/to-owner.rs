struct feature_ToOwner;
impl feature_ToOwner {
    // the relay's audience is the EDITED WORLD's owner, not the edit's signer.
    // For someone editing their own world those are the same person, so the
    // case this chain was written for is untouched; for a bench edit through
    // the diag door, which signs nothing, it is the difference between the
    // update arriving and the update going nowhere.
    fn ctx_relay_audience(from: String) -> String {
        let owner = ctx_owner_audience();
        if owner.is_empty() {
            return existing.ctx_relay_audience(from);
        }
        owner
    }

    // a request runs under one world key (/per-user), and since /whole-number a
    // person's identity on this server IS that key: `sender_of` answers
    // `phone:<number>`, and the audience `msg_wait` filters on is
    // `user.<that key>` — opaque on the wire, but spelled identically at both
    // ends. So the owner's audience is the world key, unchanged.
    //
    // Only a `phone:` world has a listener. A `local:` world is tooling's own,
    // `_global` is the shared layer /overlay borrows this seat for, and the
    // wasm place never sets a key at all. All three answer empty, which hands
    // the decision back to the chain beneath rather than inventing an audience.
    fn ctx_owner_audience() -> String {
        let who = context_user_now();
        if !who.starts_with("phone:") {
            return String::new();
        }
        format!("user.{}", who)
    }
}

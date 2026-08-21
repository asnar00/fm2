struct feature_Sync;
impl feature_Sync {
    // the whole shared-counter feature, post-migration: this node declares a
    // GLOBAL counter and redefines /tap's three-function seam to reach it. With
    // this node unticked the seam's own definitions stand and the count is the
    // device-scoped one /tap declares — which is exactly what unticking meant
    // before, when the choice was `SyncVar::local` versus `SyncVar::global` on
    // one key.
    //
    // the reads and the edits both go to the LAYER, because a global var's
    // authority is the layer and its resolver never looks at a user's own
    // field. Editing the layer locally is what keeps the count optimistic: the
    // tap shows immediately, and the server's authoritative total arrives a
    // moment later and replaces it.

    fn tap_count_read() -> u64 {
        with_context(|c| c.sync_tap_count_shared_get().sum)
    }

    fn tap_count_bump() {
        edit_layer(|c| {
            let _ = c.edit_op("miso/loop/tap/sync", "tap_count_shared",
                              serde_json::json!(1));
        });
    }

    fn tap_count_reset(n: u64) {
        edit_layer(|c| {
            let _ = c.edit_reset("miso/loop/tap/sync", "tap_count_shared",
                                 serde_json::json!(n));
        });
    }
}

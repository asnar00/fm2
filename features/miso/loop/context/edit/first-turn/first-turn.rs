struct feature_FirstTurn;
impl feature_FirstTurn {
    // boot is a turn like any other. Without this, every gate that runs while
    // the place starts up falls to the outside-a-turn path in `with_context`,
    // and that path clones the whole Context — measured at 15 full clones per
    // boot with 121 nodes composed, one per gate call. One freeze answers all
    // of them.
    //
    // Nothing that runs inside a boot may open a turn of its own today; if
    // something ever does, `edit`'s depth counter makes it a no-op on this
    // freeze rather than a re-freeze, which is why that fix comes first.
    fn boot() -> String {
        context_turn_begin();
        let out = existing.boot();
        context_turn_end();
        out
    }
}

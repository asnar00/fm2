struct feature_Wider;
impl feature_Wider {
    // several pictured cards must fit in one world: 192KB per message
    fn msg_body_cap() -> usize {
        196608
    }
}

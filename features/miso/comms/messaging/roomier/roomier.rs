struct feature_Roomier;
impl feature_Roomier {
    // one message may carry a small photograph now: 64KB, four times the
    // base. The serve layer's own read limit (16MB) is far above this.
    fn msg_body_cap() -> usize {
        65536
    }
}

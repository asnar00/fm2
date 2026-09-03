struct feature_RoomForATeam;
impl feature_RoomForATeam {
    // a canvassing team's worth of pictured cards must cross in one op:
    // 1MB per message, with ~62% headroom over a full 640,000-char list
    // once the op's envelope and JSON escaping are counted. The serve
    // layer's own read limit (16MB) is still far above this.
    fn msg_body_cap() -> usize {
        1048576
    }
}

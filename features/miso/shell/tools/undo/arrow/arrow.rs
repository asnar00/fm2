struct feature_Arrow;
impl feature_Arrow {
    // the undo glyph becomes the traditional back-curving arrow: the control
    // row is /undo's, so the swap is made on the chain's output rather than
    // by re-emitting the button
    fn tool_controls(state: String) -> String {
        existing.tool_controls(state).replace("\u{21b6}", "\u{21a9}")
    }
}

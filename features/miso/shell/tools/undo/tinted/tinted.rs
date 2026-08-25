struct feature_Tinted;
impl feature_Tinted {
    // the undo control joins the house style: black glyph on a palette
    // colour, the way /ember tints every tool button. The colour comes from
    // tool_colour("undo") — /ember's stable pick for a name it never met —
    // so it is one of the six and the same on every build.
    fn tool_controls(state: String) -> String {
        let html = existing.tool_controls(state);
        let colour = tool_colour("undo".to_string());
        if colour.is_empty() {
            return html;
        }
        let style = format!(" tinted\" style=\"--tool-colour:{}\" data-ev=\"ctx_undo\"", colour);
        html.replace("\" data-ev=\"ctx_undo\"", &style)
    }
}

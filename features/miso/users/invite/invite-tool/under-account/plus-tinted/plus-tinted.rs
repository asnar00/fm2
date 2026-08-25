struct feature_PlusTinted;
impl feature_PlusTinted {
    // the person-with-a-plus joins the house style: black on a palette
    // colour, /ember's stable pick for "invite"
    fn tool_controls(state: String) -> String {
        let html = existing.tool_controls(state);
        let colour = tool_colour("invite".to_string());
        if colour.is_empty() {
            return html;
        }
        let style = format!(" tinted\" style=\"--tool-colour:{}\" data-ev=\"tool_invite\"", colour);
        html.replace("\" data-ev=\"tool_invite\"", &style)
    }
}

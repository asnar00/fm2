struct feature_Back;
impl feature_Back {
    // a drawn ‹ at the left edge of an open tool's row: the way out, said
    // plainly. It fires tools_home — the event the old ‹ fired (#p42), the
    // one #p88 folded into the tool's own button; that button still steps
    // back a level, this leaves the tool in one tap.
    fn render_toolbar(state: String) -> String {
        let html = existing.render_toolbar(state);
        if open_tool_read().is_empty() {
            return html;
        }
        html.replacen("<div class=\"toolbar\">",
                      &format!("<div class=\"toolbar\"><div class=\"tool-button ctrl back\" data-ev=\"tools_home\" title=\"back\">{}</div>", back_svg()), 1)
    }

    fn back_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M15 5 8 12l7 7\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.6\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }
}

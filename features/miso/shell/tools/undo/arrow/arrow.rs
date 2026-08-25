struct feature_Arrow;
impl feature_Arrow {
    // the undo glyph becomes a drawn back-curving arrow. Not a character:
    // ↩ (U+21A9) carries an emoji presentation on iOS and arrives as a colour
    // bitmap that no CSS colour can touch (accounts #p36). An inline SVG in
    // currentColor is black on a tint and white on plain, like a filtered
    // emoji icon — the shape the ask wanted, in the toolbar's own ink.
    fn tool_controls(state: String) -> String {
        existing.tool_controls(state).replace("\u{21b6}", &undo_arrow_svg())
    }

    fn undo_arrow_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M9 5 4 10l5 5\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.6\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            "<path d=\"M4 10h10a5 5 0 0 1 0 10h-4\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.6\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }
}

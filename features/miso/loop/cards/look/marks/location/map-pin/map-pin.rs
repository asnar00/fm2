struct feature_MapPin;
impl feature_MapPin {
    // the map-locator glyph before the words, drawn (/glyphs): a pin in
    // currentColor, so it dims and inks with the pill
    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card);
        html.replace(">map location<", &format!(">{} map location<", pin_svg()))
    }

    fn pin_svg() -> String {
        String::from(concat!(
            "<svg class=\"pin-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M12 22s7-7.4 7-12a7 7 0 1 0-14 0c0 4.6 7 12 7 12z\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" stroke-linejoin=\"round\"/>",
            "<circle cx=\"12\" cy=\"10\" r=\"2.4\" fill=\"currentColor\"/>",
            "</svg>"))
    }
}

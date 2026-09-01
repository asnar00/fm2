struct feature_Photo;
impl feature_Photo {
    // the first kind in the set: a camera, in front of the video and the dot.
    // The order among the kinds is provenance order, which is what puts this
    // one first — nothing here chooses a position.
    fn capture_extra(state: String) -> String {
        format!("{}{}", existing.capture_extra(state),
                capture_button("capture_photo".to_string(),
                               "photo".to_string(),
                               photo_camera_svg()))
    }

    // drawn, in currentColor, per /glyphs — a body, the viewfinder hump, and
    // a lens. Never a character with an emoji presentation.
    fn photo_camera_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M3.2 8.6h3.4l1.5-2.4h7.8l1.5 2.4h3.4v10.2H3.2z\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.1\" ",
            "stroke-linejoin=\"round\"/>",
            "<circle cx=\"12\" cy=\"13.4\" r=\"3.3\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.1\"/>",
            "</svg>"))
    }
}

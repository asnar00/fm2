struct feature_VideoOnly;
impl feature_VideoOnly {
    // every post is a video, wherever the video control is in the row
    fn one_add_mode(photo: String, vid: String, rec: String) -> String {
        if !vid.is_empty() {
            return "video".to_string();
        }
        existing.one_add_mode(photo, vid, rec)
    }

    // one kind is no choice: the mode control is not drawn
    fn one_add_mode_button(glyph: String) -> String {
        let _ = glyph;
        String::new()
    }
}

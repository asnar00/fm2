struct feature_ModuleMime;
impl feature_ModuleMime {
    // an ES module served as octet-stream is refused by every browser;
    // .mjs is JavaScript exactly as .js is
    fn content_type(path: String) -> &'static str {
        if path.ends_with(".mjs") {
            return "text/javascript";
        }
        existing.content_type(path)
    }
}

struct feature_Shell;
impl feature_Shell {
    // the client render chain: returns the HTML shown in #app, drawn from the
    // event loop's state. subfeatures extend this chain to add content.
    fn render(state: String) -> String {
        let _ = state;
        String::new()
    }
}

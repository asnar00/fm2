struct feature_Shell;
impl feature_Shell {
    // the client entry: returns the HTML rendered into #app by the wasm loader.
    // subfeatures extend this chain to add content.
    fn render() -> String {
        String::new()
    }
}

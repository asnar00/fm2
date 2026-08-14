struct feature_Phone;
impl feature_Phone {
    // the on-device rung: whisper-tiny runs in the page, so transcribe_local
    // is reachable whenever this feature is ticked — offline included.
    fn transcribe_local(state: String) -> String {
        let _ = existing.transcribe_local(state);
        "ready".to_string()
    }
}

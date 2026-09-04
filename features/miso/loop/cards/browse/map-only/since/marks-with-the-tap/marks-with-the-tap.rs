struct feature_MarksWithTheTap;
impl feature_MarksWithTheTap {
    // the marks ride the pill's own tap, so the write and the filter happen in
    // one turn from one finger. Written BEFORE the chain beneath runs, so any
    // inner link that reads them on this turn already sees them — and because
    // `render` follows the whole chain, the cut this tap produces is the cut
    // this tap's frame is drawn with.
    //
    // /since's own `since_marks_write` does the writing: the var is the
    // parent's and its address belongs in one place.

    fn update(state: String, event: String) -> String {
        let marks = e_marks(event.clone());
        if !marks.is_empty() && marks != since_marks_read() {
            since_marks_write(marks);
        }
        existing.update(state, event)
    }

    // the marks ride at the TOP level of the event, beside `type`, not inside
    // `data`: `data` belongs to whichever node minted the event and its shape
    // is that node's, while nothing reads an unknown top-level key. Every
    // event carries them, so the freshest answer the phone has is written by
    // whatever the phone did last — a tap, a beat, a message arriving — and
    // the filter can be stale only until the very next event of any kind.
    fn e_marks(event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        e["marks"].as_str().unwrap_or("").to_string()
    }
}

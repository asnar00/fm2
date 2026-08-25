struct feature_FreshWords;
impl feature_FreshWords {
    // only NEW words ring. The base spoke whatever question or note the
    // changed entry carried, so a status flip on an entry still holding its
    // (answered) question re-sent the question. Now a question or note may
    // speak only if the entry did not carry those same words before; a
    // change with nothing new on it is silent.
    fn attention_news(before: &String, after: &String) -> String {
        let old: serde_json::Value = serde_json::from_str(before)
            .unwrap_or(serde_json::Value::Null);
        let new: serde_json::Value = serde_json::from_str(after)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        let olds = old.as_array().unwrap_or(&empty).clone();
        for e in new.as_array().unwrap_or(&empty) {
            let t = e["t"].clone();
            let mut was = serde_json::Value::Null;
            for o in olds.iter() {
                if o["t"] == t {
                    was = o.clone();
                }
            }
            if &was == e {
                continue;
            }
            let fresh = fresh_words_of(&was, e);
            if !fresh.is_empty() {
                return fresh;
            }
        }
        String::new()
    }

    // the words an entry gained in this change: its question text if that
    // text is new, else its note if that note is new, else nothing.
    fn fresh_words_of(was: &serde_json::Value, now: &serde_json::Value) -> String {
        let q_now = now["question"]["text"].as_str().unwrap_or("").to_string();
        let q_was = was["question"]["text"].as_str().unwrap_or("").to_string();
        if !q_now.is_empty() && q_now != q_was {
            return q_now;
        }
        let n_now = now["note"].as_str().unwrap_or("").to_string();
        let n_was = was["note"].as_str().unwrap_or("").to_string();
        if !n_now.is_empty() && n_now != n_was {
            return n_now;
        }
        String::new()
    }
}

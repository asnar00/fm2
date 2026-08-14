struct feature_Counter;
impl feature_Counter {
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({ "id": "taps", "label": "taps", "icon": "👆" }));
        }
        list.to_string()
    }
}

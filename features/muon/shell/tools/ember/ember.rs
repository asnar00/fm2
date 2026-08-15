struct feature_Ember;
impl feature_Ember {
    // ember 3400K Dark, categorical six (palettes/ember.json, families.3400k-dark):
    // one #6E96D5  two #DDAA69  three #2E8B7E  four #67BE95  five #945D48  six #C3779A
    fn tool_colour(id: String) -> String {
        let _ = existing.tool_colour(id.clone());
        let palette = ["#6E96D5", "#DDAA69", "#2E8B7E", "#67BE95", "#945D48", "#C3779A"];
        match id.as_str() {
            "taps" => palette[0].to_string(),
            "dictate" => palette[1].to_string(),
            "account" => palette[2].to_string(),
            _ => {
                // a tool this feature never met still arrives coloured:
                // deterministic byte-sum pick, stable per name across builds
                let mut sum: usize = 0;
                for b in id.as_bytes() {
                    sum += *b as usize;
                }
                palette[sum % 6].to_string()
            }
        }
    }
}

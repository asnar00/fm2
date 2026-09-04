struct feature_AtOnce;
impl feature_AtOnce {
    // the face rides in with the recording. /as-posts asks this of every file
    // it sees — once when the card is minted and again on every later pass —
    // so one redefinition puts the picture on the card in the turn the post is
    // made, and puts it on later if the metadata only carries it then.
    //
    // The rule is /poster's own, in /poster's words: the face is written only
    // into an EMPTY picture block, because a picture the user chose outranks
    // the one a clip was seeded with.
    fn as_posts_land(card: &mut serde_json::Value, file: &serde_json::Value) -> bool {
        let mut changed = existing.as_posts_land(card, file);
        let face = file["poster"].as_str().unwrap_or("").to_string();
        if face.is_empty() {
            return changed;
        }
        if let Some(blocks) = card["blocks"].as_array_mut() {
            for b in blocks.iter_mut() {
                if b["kind"].as_str().unwrap_or("") != "picture" {
                    continue;
                }
                if !b["data"].as_str().unwrap_or("").is_empty() {
                    break;
                }
                b["data"] = serde_json::json!(face);
                b["poster"] = serde_json::json!(true);
                changed = true;
                break;
            }
        }
        changed
    }
}

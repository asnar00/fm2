struct feature_RoleInTheTag;
impl feature_RoleInTheTag {
    // a person's tag says what they ARE on this project, not what kind of card
    // they are. "profile" told the reader something they could already see —
    // they are looking at a person — while the useful fact, the one that
    // decides who sees which post, was nowhere on the card.
    //
    // The word is the grade itself (`admin`, `candidate`, `team`, `volunteer`,
    // `supporter`, `public`) rather than /plain-words' sentence forms: those
    // are written to finish "visible to …" and read as prose, and a tag is a
    // label. `audience_grade_in` is the one place that answers "where does this
    // person stand in this project", so it is asked rather than re-derived.

    fn card_tag_word(card: &serde_json::Value) -> String {
        if card["type"].as_str().unwrap_or("") != "profile" {
            return existing.card_tag_word(card);
        }
        let proj = current_project_card();
        if proj.is_null() {
            return existing.card_tag_word(card);
        }
        let owner = card["owner"].as_str().unwrap_or("").to_string();
        let grade = audience_grade_in(&proj, owner);
        if grade.is_empty() {
            // named on no project, or on another one: they are still a person,
            // and "profile" is the honest word for a card with no role to show
            return existing.card_tag_word(card);
        }
        grade
    }
}

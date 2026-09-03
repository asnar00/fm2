struct feature_Title;
impl feature_Title {
    // after everything else on the page: the project you are in, by name
    fn render(state: String) -> String {
        let base = existing.render(state);
        format!("{}{}", base, current_title_html())
    }

    fn current_title_html() -> String {
        let proj = current_project_card();
        if proj.is_null() {
            return String::new();
        }
        let title = card_esc(browse_title_of(&proj));
        let title = if title.is_empty() { "a project".to_string() } else { title };
        format!("<div class=\"proj-title\" data-ev=\"proj_select:{}\" title=\"{} — tap to leave\">{}</div>",
                card_esc(proj["id"].as_str().unwrap_or("").to_string()), title, title)
    }
}

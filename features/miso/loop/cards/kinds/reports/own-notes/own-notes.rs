struct feature_OwnNotes;
impl feature_OwnNotes {
    // the writer is told what the posts are. Everything after the first
    // paragraph is /reports' own instruction, kept word for word.
    fn reports_system() -> String {
        let base = existing.reports_system();
        let cut = match base.find("Answer the question") {
            Some(i) => base[i..].to_string(),
            None => base,
        };
        format!(concat!(
            "You are writing a short internal report for a local political campaign team, ",
            "from the team's own posts. The data is notes and impressions written or dictated ",
            "by team members themselves, usually after the fact, each with a time, an author ",
            "and often a location. None of it is a recording of a member of the public, and ",
            "you must not describe it as doorstep conversations, canvassing recordings or ",
            "anything of the kind.\n\n{}"), cut)
    }

    fn reports_corpus_heading() -> String {
        String::from("THE TEAM'S POSTS, NEWEST FIRST\n\n")
    }
}

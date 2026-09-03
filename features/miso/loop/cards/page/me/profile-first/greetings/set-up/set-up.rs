struct feature_SetUp;
impl feature_SetUp {
    // the second page, with the two rows before the button; the page half
    // settles the rows and lifts `setup-wait`
    fn greetings_sheet(n: i64) -> String {
        let base = existing.greetings_sheet(n);
        if n != 2 {
            return base;
        }
        let rows = concat!(
            "<div class=\"greet-rows\">",
            "<div class=\"greet-row\" data-setup=\"passkey\"><span class=\"greet-what\">Face ID login</span><span class=\"greet-do\">enable</span></div>",
            "<div class=\"greet-row\" data-setup=\"push\"><span class=\"greet-what\">notifications</span><span class=\"greet-do\">enable</span></div>",
            "</div>");
        let base = base.replacen("class=\"greet\"", "class=\"greet setup-wait\"", 1);
        match base.find("<div class=\"greet-go\"") {
            Some(at) => format!("{}{}{}", &base[..at], rows, &base[at..]),
            None => base,
        }
    }
}

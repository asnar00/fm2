struct feature_Recentre;
impl feature_Recentre {
    // ---- the control in the row --------------------------------------------
    // a sub-tool of whichever tool is showing the map, not a button floating
    // over it (/quiet-credits' rule, and /tools': the interface is a tree of
    // tools and an action is a button in the control row). The row is composed
    // on every paint, so the crosshair follows the view turn by turn.
    //
    // TWO conditions in two links, because one link cannot answer both. Here:
    // is this device's chosen view the map — cheap, and true of the device
    // whichever tool is open. In `render`: is a map actually on the screen —
    // exact, and only knowable once the surface has been drawn. The view var
    // is sticky and device-wide, so someone who chose the map inside 👤 and
    // then opened taps is still "in map view" by the var alone; the second
    // link is what keeps the crosshair out of taps' row.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state);
        if open_tool_read().is_empty() || browse_view_read() != "map" {
            return row;
        }
        recentre_before_undo(row, recentre_button())
    }

    // /undo's button is the last in every control row, and a newer node's link
    // lands after undo's, so keeping the invariant is this node's job. Its own
    // copy of the inserter rather than /invite's or /posts': this node stands
    // without either of them (/posts made the same call, 2026-08-25).
    fn recentre_before_undo(row: String, add: String) -> String {
        if add.is_empty() {
            return row;
        }
        match row.find("data-ev=\"ctx_undo\"") {
            Some(at) => match row[..at].rfind("<div") {
                Some(start) => format!("{}{}{}", &row[..start], add, &row[start..]),
                None => format!("{}{}", row, add),
            },
            None => format!("{}{}", row, add),
        }
    }

    // the chip: the row's own 50px button wearing /ember's tint, black on the
    // colour like every other control (/glyphs). `tool_colour` is /ember's
    // stable pick for a name it never assigned — the /tinted idiom — so the
    // colour is identical on every device, and empty with /ember unticked,
    // which leaves the plain control.
    fn recentre_button() -> String {
        let colour = tool_colour("recentre".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        format!("<div class=\"tool-button ctrl recentre-ctrl{}\" data-ev=\"map_recentre\" title=\"centre on me\">{}</div>",
                tint, recentre_crosshair_svg())
    }

    // ---- the truth about whether a map is on the screen ----------------------
    // #mapData is /map's whole contribution to the page, and its presence is
    // what "the map view is up" MEANS — /live's `onMap` asks the same question
    // of the same element on the page half. It is emitted by the surface
    // renderers, which run outside /tools' toolbar, so no link on the controls
    // chain can see it; this link runs outside them all and reads the finished
    // page. No map drawn — a tool with no card surface, a card page opened
    // from a pin — and the control comes out.

    fn render(state: String) -> String {
        let html = existing.render(state);
        if html.contains("id=\"mapData\"") {
            return html;
        }
        recentre_strip(html)
    }

    // remove the whole button element: the opening <div before the marker and
    // the first </div> after it. The glyph inside is an SVG and the button
    // holds no nested div, so the first close is the button's own (/aside's
    // cut, for the same reason).
    fn recentre_strip(html: String) -> String {
        match html.find("data-ev=\"map_recentre\"") {
            Some(at) => match (html[..at].rfind("<div"), html[at..].find("</div>")) {
                (Some(start), Some(rel)) => format!("{}{}", &html[..start], &html[at + rel + 6..]),
                _ => html,
            },
            None => html,
        }
    }

    // ---- the glyph -----------------------------------------------------------
    // drawn, in currentColor (/glyphs): a ring with four ticks and a dot — the
    // crosshair, which everywhere means "put me in the middle". NOT a pin:
    // /map refused a pin for its own button because the pins are the things ON
    // the view, and the same argument holds for a control that moves it.

    fn recentre_crosshair_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"12\" r=\"6\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\"/>",
            "<path d=\"M12 1.9V3.7M12 20.3V22.1M1.9 12H3.7M20.3 12H22.1\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.6\" stroke-linecap=\"round\"/>",
            "<circle cx=\"12\" cy=\"12\" r=\"1.8\" fill=\"currentColor\"/>",
            "</svg>"))
    }
}

struct feature_MapOnly;
impl feature_MapOnly {
    // ---- the one view ------------------------------------------------------
    // /browse's reader for the `view` device var, redefined to a constant.
    // `existing` is deliberately NOT consulted: the stored value is exactly
    // what this node exists to stop mattering, so a device that chose the list
    // a week ago gets the map like everybody else. Nothing writes the var any
    // more either — the buttons that did are not drawn — so there is nothing
    // to migrate, and unticking this node hands the stored value straight back.

    fn browse_view_read() -> String {
        String::from("map")
    }

    // ---- the slot the picker vacated ---------------------------------------
    // every surface (/browse, /people, /posts, /projects) calls
    // browse_picker_html() and drops the result into the top strip. With no
    // views to choose between, that result is whatever the slot holds — the
    // empty string by default, so nothing draws rather than an empty pill
    // hanging at the top left. `browse_slot_html` is the seam the next
    // occupant redefines; /since takes it in the same breath (/learned 13).
    //
    // Note this is a redefinition rather than a wrapper: the picker's own row
    // is not appended to, it is replaced. /browse's `browse_views` and
    // `browse_view_button` are untouched and simply unreachable while this
    // node is on.

    fn browse_picker_html() -> String {
        browse_slot_html()
    }

    fn browse_slot_html() -> String {
        String::new()
    }
}

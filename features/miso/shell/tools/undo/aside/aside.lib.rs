// the turn's quiet mark: set by /aside's outermost update link for the length
// of a turn whose edits are not the person's to undo (the undo press itself,
// a card the machine makes), read by its undo_record link to decline the
// step. The loop is one thread; the Mutex is for the borrow checker, as
// /late's stash is.
pub static FM_UNDO_QUIET: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

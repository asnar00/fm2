struct feature_Members;
impl feature_Members {
    // a member may invite: the guest list grows itself. Rank 1 is "on the
    // list" — a valid cookie whose entry has since been removed ranks 0 and is
    // still refused, so the check keeps /authority's shape rather than
    // becoming "anyone with a cookie". What an invitee BECOMES is untouched:
    // every road onto the list writes an entry with no authority field, so a
    // member's invitee is a member, and /pretend's admin-only test users and
    // invite_admin's take-back of other people's invites stand as they were.
    fn invite_may(who: String) -> bool {
        !who.is_empty() && authority_rank(who) >= 1
    }
}

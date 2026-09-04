# not-people
*the time filter never hides a person; on 👤 the slot is empty, because there is nothing there for it to do*

> (transcripts/2026-09-04-field-walk.md#p162)
> yeah, let's always show all users on the project, but let's sort them as follows: a) self first b) sort by most recently active first [with a mod to stop things pinging around constantly while looking at the list]

*(The sorting half is `/by-activity`, a child of `/people`. This node is the
first clause: always show all users.)*

## user

Open 👤 and everyone you hold on the project is there, whatever the filter says.
The top-left slot is empty on that screen — there is nothing to filter — and it
is back, on the word you left it on, the moment you open posts or projects.

## spec

`/since` cuts a browsed set to a slice of time. That is right for things that
**happen** — a post has a moment it records — and wrong for people, who are not
events: a colleague does not stop being on the project because they last edited
their card in August. Ash found it in the field, on the map with **today**
chosen and a person he holds missing.

**The test is the card's type, not the tool.** "A person is never hidden by a
clock" is true wherever a person is drawn, and `open_tool_read() == "account"`
would only have been true where they are drawn today. `/since`'s own exemption
for your own profile card becomes a special case of this one; it is left where
it is, so unticking this node gives that behaviour back exactly.

**The slot goes quiet on 👤.** With people never cut, the four words change
nothing there, and a control that does nothing is noise — `/taste` 7 and 8. So
`browse_slot_html` answers empty on the people tool and is unchanged on posts
and projects. The alternative, leaving the word drawn, was rejected for the
reason the brief names: it would be a control that lies about having an effect.

**The period itself is untouched.** It is a user var and keeps whatever it held,
so walking from 👤 to posts finds the same word lit as before. Nothing is
written, nothing is reset, and the filter is exactly where the hand left it.

## hostile cases

- **A profile card on a surface that is not 👤** (a project's people section, a
  person's card opened from a map): never cut, because the test is the card.
- **The slot on the cards tool** (`/browse`'s retired tool): unchanged; only
  `account` is named.
- **A period of `today` and a person made last year.** Shown. That is the ask.
- **`/one-word` unticked.** The slot is `/since`'s four pills again, and they
  are hidden on 👤 by the same line.
- **This node unticked.** People are cut by the clock again and the slot draws
  everywhere — the behaviour ash reported.

## glossary

(no new terms)

## code description

`not-people.rs` redefines `since_keep(card)` to pass any card of type
`profile` before the chain sees it, and `browse_slot_html()` to answer empty
while the account tool is open.

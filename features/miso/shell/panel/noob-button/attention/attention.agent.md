# attention — how your messages reach the user

*You are sending something to a user — a question, a stamp, a note.
This instruction toggles with the `/attention` node.*

Send through the world, never around it: stamp the user's own vars
through the op door (`stamp_ask.py`, `POST /diag/context`) and the
attention ladder picks the channel — open panel updates in place,
foreground gets the gentle lozenge pulse, backgrounded gets a
notification. Never choose the channel yourself and never stack them.

Nothing rings about nothing: a stamp that changes no entry sends no
wire traffic; a change with no words on it stays silent; never pad a
notification with filler text to force it through. If a user should
notice something, put the words on the change itself (the question
text, the note) — the ladder does the rest.

# greetings
*two words of welcome on a first sign-in: your project and your profile, then how to learn the buttons*

> (transcripts/2026-09-03-invite-test.md#p74i)
> once you're signed in for the first time, we should see a welcome page:
> "welcome to the <project> project on miso!" - explain that we're first
> going to set up your profile and we need a picture and mission statement;
> then after profile page is done, another welcome page letting the user
> know that they can hold down any button for 2 sec to learn what it does.

## user

The first time you are in, a page says **welcome to the sevenoaks project
on miso!** and that the first thing is your profile: a picture and a line
about what you are here to do. Tap **let's go** and your card is there to
fill in. When it is done, one more page: **hold any button for two seconds
and it tells you what it does.** Tap **got it** and the app is yours; the
short tour follows as before. Neither page comes back, on this phone or
another.

## spec

`/profile-first` lands a newcomer on their own card with one sentence on
it; `/tour` follows once the card is filled. Between a scan and that card
nothing says where the person is or why (#p74i). Two pages, at the two
moments the gate already defines.

**The first page stands with the gate.** `render` is extended: while
`profile_first_gated` holds and this person has not been greeted, a sheet
over the card — the project's name from `current_project_card()` (a
newcomer's current project is set at their join, `/invited-into`; if it has
not arrived yet the page says *welcome to miso!* and repaints with the name
on the turn it lands), the ask in two sentences, and **let's go**. The card
underneath is the one they are about to fill; nothing is navigated.

**The second page comes when the gate lifts.** Once the world is joined,
the gate is down, and the person was greeted once, the second sheet:
the long-press rule in one line, and **got it**. `/tour` asks `may()`
before it starts; this node's page half wraps that to say no while a sheet
is on screen, so the tour's first card waits for **got it** and then goes
on as before.

**Once per person, and it travels.** `greeted` is a user-scoped var: 0, 1
after **let's go**, 2 after **got it**. The click `greet_next` steps it.
Someone who joined before this build is never greeted — the first page
requires the gate, which lifted for them long ago, and the second requires
the first.

**Copy.** Plain, short, the app's own word (`/taste` 7): *welcome to the
sevenoaks project on miso!* / *first, your profile: a picture, and a line
about what you're here to do.* / *hold any button for two seconds and it
tells you what it does.*

## hostile cases

- **The join has not landed when the first page paints.** *welcome to
  miso!*; the next turn carries the project and the page says its name.
- **No current project at all** (a typed invite, not a code). *welcome to
  miso!* throughout.
- **A relaunch mid-greeting.** `greeted` is the person's, so the same page
  is back; the sheet is render output, not furniture, and needs no restore.
- **`/tour` unticked.** Nothing waits; the second page still shows and says
  a true thing about `/long-press`.
- **`/current-project` unticked.** `current_project_card` is not composed
  and the linker refuses this node — they travel together.
- **This node unticked.** The card and the tour, as before.

## code description

`greetings.rs` — `render` appends the first or second sheet by the gate and
`greeted`; `update` steps `greeted` on `greet_next`; `greetings_sheet(n)`
builds a sheet. `greetings.vars` — `greeted`. `greetings.js` — wraps
`feature_Tour.may` to wait while a sheet is on screen. `greetings.css` —
the sheet.

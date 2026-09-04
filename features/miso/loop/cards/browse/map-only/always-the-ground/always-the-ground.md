# always-the-ground
*inside a browse tool the map is the ground under everything; the launcher keeps its dots*

> (transcripts/2026-09-04-field-walk.md#p110)
> also, don't revert to blue dots ever - we should always show the map as the background

> (the correction, minutes later — the scope. Not yet in an export, so it is
> quoted in the shorthand the tree uses for a prompt it is not citing; the
> anchor stays the ruling.)
> that should apply for all tools that have a list of objects e.g. posts,
> users, projects — but not for the main page

> (and the sharpening, which is the rule this node is actually built on — also
> not yet exported)
> when I hit the "add post" button, the map disappears and I see grid dots —
> it shouldn't. In general, a sub-tool panel shouldn't change the background
> from the parent's unless it has a reason to — we should treat the parent's
> background choice as important

## user

Once you are inside posts, 👤 or projects, the map is behind everything you do
there: the set, a post or a person or a project you open, the recording row,
the publish-level list, the region page, the invite page. It stays where you
left it the whole time. Tap ‹ back to the tool grid and the dots are there
again, because that is not a place on the map.

## spec

**The rule is inheritance, not a list.** Ash's sharpening is the whole design:
*a sub-tool panel shouldn't change the background from the parent's unless it
has a reason to — we should treat the parent's background choice as
important*. So the ground is chosen once, at the level that has something to
say about it, and everything opened from there keeps it until something with a
stated reason changes it. The launcher's dots are the root's choice; a browse
tool's map is that tool's; a level or page opened inside one inherits, and
inherits again however deep it goes.

The enumeration in the proof below — the recording row, the level list, the
region page, the invite page — is therefore *evidence*, not the specification.
No name of any of them appears in the code, and a level added tomorrow gets the
map without this node changing.

`/opens-over-map/on-every-tool` keeps the map behind an open **card** by
remembering which tool the map was last drawn for. This is the same memory,
asked the wider question. Measured on the rig before this node, four surfaces
still painted the dot ground — the recording row (ash's own case: "when I hit
the 'add post' button, the map disappears"), the level list, the region page,
the invite page — while the sets and the card pages were already right.

**The memory is `/on-every-tool`'s, widened rather than copied.** That node
notes the selected tool on every sync where `#mapData` is on the page, which is
every sync where a set is showing. This node reads that note and asks a wider
question of it. It needed no seam: the note is a field on a global object and
the question is asked from here.

**The one boundary is ‹.** The back chevron is drawn only while a tool is open,
so its presence in the row is exactly "not the launcher" — the only line ash
drew. Everything on the tool side of it gets the map; the launcher keeps the
graph paper, which is what the correction asked for.

**"Still inside" is asked of the registry, not of a list of levels.** The
selected button is the tool itself — its set or its card page — and the map is
that tool's ground. Or no button is selected at all, which is a level with no
tool of its own (the recording row's is `vid_rec`, `armed_flip`,
`armed_pick`) — so it inherits. Or a button is selected that the **registry
does not name**, which is `/one-level`'s own test for a nested tool, asked here
on the page half because `tools_catalog` is bridged — so it inherits too. Only
a selected button the registry *does* name, and which is not the one the ground
was chosen for, is a different top-level tool: a new root, making its own
choice, which for `/reports` is the dots.

**"Unless it has a reason" is a seam, not a sentence.** `ownGround()` answers
false for every screen today, and a level that grows a reason to draw its own
ground redefines it and says why in its own spec. Written this way the rule
stays true of levels that do not exist yet, which is what makes it inheritance
rather than a list with a nicer description.

**Clearing the memory is safe for the node that owns it.** `/on-every-tool`
never clears `was`, and does not need to: its test is that the selected tool
*is* the one the map was drawn for, which no stale value can satisfy. A wider
test can be fooled by one, so this node clears it at the launcher and on a
switch to another top-level tool — both screens on which `/on-every-tool` had
already answered no. Its behaviour is therefore unchanged.

**The deep link draws the map.** A relaunch straight into a remembered card
(`/restore` reopens the tool, `/browse` reopens the card) never shows a set, so
nothing is noted and there is no map to keep. The filter slot answers it: it is
drawn by exactly the surfaces that browse a set — `/map-only`'s own seam,
filled by `/since` — so its presence on the first frame says "this tool draws a
map" without naming posts, people or projects. The tool is noted from it, and
if Leaflet has never been made, `mount()` makes it.

**The card page is left alone.** `/opens-over-map` also arms a tap on the map
that puts the card away, and it does that through the `fm-map-behind` class.
This node stands back whenever that class is set, so the card keeps its
parent's behaviour exactly, and no other surface gets a tap that would close
something the finger was not aiming at — tapping the map while the recording
row is up does nothing, which is right.

## hostile cases

- **A page that draws its own ground.** The region page and the invite page are
  both `.card-page`, which hugs its content, so the map shows around them
  rather than being covered — the ask's preferred answer, and it needed no
  change to either.
- **The reports tool.** Its button is in the registry and is not the remembered
  one, so the memory drops and reports keeps the dots. Reports does not list
  things on a map.
- **‹ from a nested level back to the tool.** The set returns, `#mapData` is on
  the page, and `/map` shows the map itself; this node does nothing.
- **‹ all the way to the launcher.** No chevron, so the memory clears and the
  dots come back — and the next tool starts from nothing.
- **A tool opened from the launcher that never draws a map.** Nothing is noted
  and nothing is claimed.
- **A level nobody has written yet.** It inherits, because nothing names it.
  That is the point of the rule; the enumeration in the proof is only the
  evidence that it holds for the levels there are today.
- **A level that needs its own ground later.** It redefines `ownGround()` and
  says why. Nothing else changes.
- **`/since` unticked.** No filter slot, so the deep-link case falls back to
  the memory — which is `/on-every-tool`'s behaviour, and the map appears one
  screen later.
- **`/on-every-tool` unticked.** There is no memory to read; `drawnFor` is
  empty and this node does nothing. Its premise is that node.
- **Leaflet missing** (`assets/` half-copied). `mount()` returns false, `show()`
  reveals an empty dark host, and nothing throws — `/map`'s own answer for the
  same case.

## parked

- The tap on the map putting a *level* away, the way it puts a card away. It
  would need a rule per level about what "away" means, and no one has asked.

## glossary

(no new terms)

## code description

`always-the-ground.js` — `feature_AlwaysTheGround`. `insideATool()` is the
presence of ‹; `selected()` is the selected top-level button; `registry()` is
the bridged `tools_catalog`, so a button it does not name is a nested tool;
`browsing()` is the filter slot, which only a set-browsing surface draws.

`ownGround()` is the /extension point/ for a level with a reason to draw its
own ground; it answers false everywhere today.

`drawnFor`, `note` and `forget` read and write `/on-every-tool`'s own memory.
`ground()` is the question: not the set, not a level with its own reason, not
the launcher, and the screen is the tool the ground was chosen for or something
nested under it.

The wrapper on `feature_Map.sync` runs last of the three, stands back from a
card page (`fm-map-behind` is `/opens-over-map`'s), and otherwise mounts the
map if it has never been made and shows it.

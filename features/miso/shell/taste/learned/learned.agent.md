# learned — build it this way before being asked

*You are building a surface of miso. These are the defaults ash's own
tweaks have taught — each was asked for after a first cut shipped
without it (`tools/tweaks.py`, the digest of 169 refinements of 76 asks,
2026-08-13 to 09-03, plus the invite-test day). Build them in first; the tweak that would have
asked for them is then never filed. Precedents are node names. An ask
that contradicts a rule amends it; nothing else does. Re-distilled at
session end from the digest since the last run.*

1. **Controls live in the toolbar, never on the page.** A page with two
   buttons is "doing the job of the toolbar"; edit and save are toolbar
   buttons; a way in is a sub-tool button in its row; a new tool that
   belongs to another is that tool's sub-tool. (`editing/toolbar`,
   `doors/as-sub-tools`, `invite-tool`, `under-account`; the tree-of-tools
   ruling.)

2. **The context stays visible under whatever opens.** A band floats over
   the map with the map showing through; a card opened from the map keeps
   the map behind it, not the dot grid; a panel is tied to the top so it
   never covers the toolbar; dismissing a panel returns to the tool you
   were in, never the launcher. (`reel/floating`, `opens-over-map`,
   `less-busy/top-tied`, `me/stay`.)

3. **Bigger than the first cut, and further from the finger.** Buttons
   +25%, thumbnails +50%, ‹ as tall as its neighbours, tooltips a little
   further above the button. Start at the larger size. (`bigger-buttons`,
   `pins/bigger-faces`, `back/tall`, `long-press/further`.)

4. **The mark goes on the subject, not the control; hints are quiet.** The
   focused post is ringed on the map, the lozenge gets a plain outline and
   no arrow; a stem is near-black for contrast; an icon is house-style ink
   on colour, never white; no dividing rule at the foot of a card; a card's
   ground hugs its content. (`current/on-the-pin`, `pins/black-stem`,
   `plus-tinted`, `invite/last-row`, `ground/hug`.)

5. **Platform idioms and the standard glyph.** The share glyph, not the
   words; swipe sideways to put a card away; tap the ground to close;
   a plain increasing build number; a map locator for "map location".
   (`viewer/share-glyph`, `reel/swipe-away`, `browse/backdrop`,
   `update/buildnum`, `location/map-pin`.)

6. **A choice made once stays made.** The front camera stays flipped; one
   add button with the mode set elsewhere; updates by consent once for all
   devices, then automatic; an instance restarts where it was and rejoins
   on foreground. Never make the user repeat a setting. (`video/flip`,
   `capture/one-add`, `review/consent-once`, `update/auto`, `tools/restore`,
   `join/resume`.)

7. **Immediate and honest feedback.** A mark moves with the gesture, not
   after it; the map follows within a beat; an open panel refreshes itself
   when something arrives; the sheet says what is being built; the network
   is tried before the cache; a stale notice is dropped. (`reel/quicker`,
   `review/live-panel`, `lifecycle/being-built`, `update/fresh`,
   `update/honest`, `live/every-second`.)

8. **Newest first, strictly.** Builds, posts and the reel run newest to
   oldest; a new item lands at the head. (`chooser/build-order`,
   `posts/post-time`, `map/reel`.)

9. **Kinds match each other.** A post has a title like a person and a
   project; picture above words on every card; the people list is the
   cards view; name then role. When one kind gets a shape, give the
   others the same. (`posts/titled`, `picture-first`, `browse/people`,
   `projects/name-first`.)

10. **What is shown is exactly what is meant.** The band lists the map's
    set; the people band is the live people; one pin per person; the
    user's own pin joins the fan; a recording is placed where it was made;
    every visual is the central square. Any two views of one thing agree.
    (`reel` #p22, `people-there/live-only`, `live/one-pin`,
    `fan-out/with-live`, `as-posts/where-taken`, `square-crop/clips-too`.)

11. **Nothing is ever lost.** A save that could drop a keystroke becomes a
    save button; a write that could clobber is guarded and revertible; a
    copy is never the original. (`keep/manual`, `cards/guard`,
    `guard/revert`, `me/patient`.)

12. **The dark ground is the whole ground.** Tiles that have not loaded are
    dark grey, one tile style at every zoom, the credit behind a button;
    nothing bright arrives unasked. (`light-basemap/map-ground`,
    `fresh-tiles`, `quiet-credits/credits-button`; `/taste` 1 and 9.)

13. **Expect the second ask within the hour, and leave it a seam.** Every
    surface that shipped drew two to four refinements within a day, most
    within an hour. Ship the literal ask, then name the likely next three
    in the spec and give each a function to redefine (`/anticipation`).

14. **Every road in does the whole enrolment.** A person who came in by a
    scan gets what a texted login gets: the seed of their inviter's cards,
    Face ID, notifications, the project's members. A second way in that
    skips a step the first way took is a bug the next tester finds within
    the hour. (`scan-is-proof/seeded`, `greetings/set-up`, `exchange/co-members`;
    2026-09-03.)

15. **A first run is pages, one thing each, and no demo.** Welcome and the
    project; the card; the two switches; "that's it". Each page one action,
    each seen once per person on any device; the guided tour is not wanted
    once the pages have said the one thing it was for. (`profile-first/greetings`,
    `greetings/set-up`, `greetings/last-word`; 2026-09-03.)

16. **Steps that depend on the phone's own UI say where to look, not which
    glyph.** "in the browser menu below", "view more" with no icon: the same
    words whichever way Safari draws its bar. (`install/steps/menu-below`,
    2026-09-03.)

17. **Motion moves with the thing it is tied to, or it is baked into it.**
    A mask recomputed when the zoom ends stands still through the pinch and
    pings into place; the cure was to bake the boundary into the tiles so
    one layer scales. An open or close animates size *and* place to the
    exact rectangle it came from, so it reads as the same thing. The ends of
    a list rubber-band; they never wrap or fly the same card off and back.
    (`region/baked`, `back-to-the-lozenge/size-too`,
    `carries-the-card/rubber-band`, 2026-09-04.)

18. **Media arrives still and plays on the finger.** No autoplay, no seek
    to a frame, no element swapped under the reader: a poster stands in
    until the tap, on the open road, the repaint road and the incoming
    road alike. (`until-play`, `until-play/incoming-too`, 2026-09-04.)

19. **A picker is a column with a sentence each, in the row you are in.**
    Options stand vertically, each with one plain line saying what it means;
    the list pops over the current row and closes on any tap outside; it
    never descends a tool level. (`armed/explained`, `armed/in-place`,
    2026-09-04.)

20. **The + arms, it does not fire.** A destructive or costly act (start
    filming, publish) sits behind a row that shows its settings first —
    rec, stop, camera, level — so there is a moment to set them; the
    settings persist. (`video-only/armed`, 2026-09-04.)

21. **One view, filtered by time, not several views of the same set.** The
    map with its band replaced grid and list; the switch's slot became
    today / week / month / all. A second view of the same cards is a
    filter in disguise. (`browse/map-only`, `map-only/since`, 2026-09-04.)

22. **What must be fresh rides with every message.** The phone's local
    midnights travel on every event, not in a boot-time send that can be
    missed; a value that only arrives once is a value that is sometimes
    wrong. (`since/marks-with-the-tap`, 2026-09-04.)

23. **There is never no picture.** A post's face is the first frame the
    moment filming starts, replaced by a better one if it comes; a dark
    frame beats an empty square. Confidence dies on a blank tile.
    (`at-once/first-frame`, 2026-09-04.)

24. **A pill hugs its words.** A lozenge is as wide as its text and its
    padding, never a fixed width; a column is its widest row; a name sits
    centred on the screen, not at an offset tuned against its neighbours.
    (`one-word/hugs-its-words`, `one-word/in-the-middle`, 2026-09-04.)

25. **What must move with the map is in the map.** Region fill and
    boundary lines are baked into the tiles, so one layer scales through
    the pinch; an overlay recomputed at zoom end lags and pings.
    (`region/baked`, `baked/lines-too`, 2026-09-04.)

26. **A stamp is never written over by the asker's resend.** Fields have
    owners: the builder's status and build, the asker's text and urgency;
    each side fills, neither overwrites. (`being-built/stamp-stands`,
    2026-09-04.)

27. **A phone bug is measured on the phone before it is fixed.** Three
    rig disproofs of a flash ended in a black-box readout of the arriving
    picture, not a fourth guess. (`blackbox/arriving-picture`, 2026-09-04.)

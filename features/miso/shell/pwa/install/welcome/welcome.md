# welcome
*the install page says hello and why*

> (transcripts/2026-09-03-invite-test.md#p74)
> 2) below the logo, add a line "welcome to miso"; then below that a small
> paragraph explaining that miso needs to be installed on your phone screen,
> and this is how you do it:

## user

Under the face: **welcome to miso**, then a short line saying that miso
lives on your phone's home screen and needs installing once, and that the
steps below are how. Then the steps, as before.

## spec

The install page was the logo and three steps — right for someone who knew
what they were looking at, and a puzzle for a canvasser who has just scanned
a code and landed on it. Ash (#p74): a welcome and a why.

**A body fragment, placed by order.** Body fragments compose in provenance
order, so this one lands after `/steps`' in the markup. The page's `main`
becomes a column and the pieces are ordered by CSS: the logo first, the
welcome next, the steps after — no script moves anything, and unticking this
node leaves the steps where they were.

**The words.** "welcome to miso" in the page's own white; the why in the
steps' grey, two sentences, no "please" (`/taste` 7): *miso works as an app
on your home screen, so it needs installing once. this is how:*

## hostile cases

- **`/steps` unticked.** The welcome stands alone under the logo, its "this
  is how:" pointing at nothing — retick or reword; the page has nothing to
  install without steps anyway.
- **This node unticked.** The logo and the steps, as before.

## code description

`welcome.install.html` — the `.welcome` block: `.hello` and `.why`.
`welcome.install.css` — `main` as a column; `.logo` first, `.welcome`
second, `#ios`/`#android` after; the two text styles.

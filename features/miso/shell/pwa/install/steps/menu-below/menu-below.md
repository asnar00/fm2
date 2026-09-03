# menu-below
*the share step says where to look, and "view more" needs no icon*

> (transcripts/2026-09-03-invite-test.md#p100)
> on the install page: it's confusing because it says "tap [share]" but
> depending on your browser settings, you don't necesarily see [share]. On my
> test browser I see the URL at the bottom and "..." in the right hand corner
> - I have to press "..." before I see [share]. Then on the other iphone,
> "more" is actually "..." as well.

> (transcripts/2026-09-03-invite-test.md#p102, the ruling)
> probably a good idea to say something "tap [share] in the browser menu
> below" - and remove the icon for "more", so it works either way.

## user

The first step reads **tap [share] in the browser menu below**, so whether
Safari shows the share button outright or tucks it behind ⋯, you know where
to look. The second step is just **then view more** — the row is called that
on every phone, whatever glyph sits beside it.

## spec

`/steps` drew the share glyph as if it were always on screen; with Safari's
compact tab bar it is behind ⋯ at the bottom right, and on some phones the
"view more" row is a ⋯ too (#p100). Ash's ruling: say where the menu is, and
drop the icon that varies.

**The page half rewrites the two lines at load.** `/steps`' markup is a
body fragment this node cannot edit; this node's script finds `#ios` and
sets its first two steps: the share key stays (it is the one glyph that is
the same everywhere once you find it), followed by *in the browser menu
below*; the second step is the words alone. Untick and the original lines
are back.

## hostile cases

- **Android.** `#android` is untouched; its menu step already names ⋮.
- **`/steps` unticked.** No `#ios`; nothing happens.
- **This node unticked.** The original two lines.

## code description

`menu-below.install.js` — replaces the text of the first two `.step`
elements in `#ios`, keeping the share key.

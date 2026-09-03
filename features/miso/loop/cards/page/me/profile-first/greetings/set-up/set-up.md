# set-up
*the second welcome page sets the device up: Face ID and notifications*

> (transcripts/2026-09-03-invite-test.md#p87)
> the post-profile-filling in welcome page should also ask for "enable
> faceID login" and "enable notifications" before allowing continue

## user

Once your card is done, the page that tells you about holding a button
also has two rows: **Face ID login** and **notifications**, each with
**enable**. Tap one, say yes to the phone, and the row shows a tick. **got
it** appears when both rows are settled — enabled, or not possible on this
device, or declined after a real try — and the app is yours.

## spec

`/enrol` sets a new device up — a passkey and a push subscription — on the
login page, riding the PIN tap's user activation. A person who came in by a
scan (`/scan-is-proof`) never saw the login page and so never got either;
their second device could not log in by Face ID and nothing could reach
them. Ash (#p87): the second welcome page asks for both before letting the
person on.

**Two rows on the second page.** `greetings_sheet(2)` is redefined: before
**got it**, a row per thing — *Face ID login*, *notifications* — each with
an **enable** button. The buttons are the page half's (no `data-ev`, the
rule `/frame`'s buttons follow): a tap calls `feature_Passkey.enrol()` or
`feature_Push.subscribe()`, the same functions the nøøb sheet's retry
buttons call, inside the tap's own user activation, which is what the two
platform prompts require.

**Settled means one of three things.** Enabled (the row ticks); not
possible here (no `PublicKeyCredential`, no `PushManager`, or not
standalone — the row says so quietly and counts as settled, since asking
again would ask nothing); or declined after a real attempt (the prompt was
refused; the row says *not now* and counts as settled — the nøøb sheet keeps
the retry, and a page that cannot be left is a trap). Already enrolled on
this device counts as settled at once.

**got it waits.** The sheet carries `setup-wait` until both rows are
settled; the button is hidden by it. The page repaints wholesale on every
turn, so the rows' state lives in the page half and is re-applied on each
apply, `/profile-first`'s idiom.

## hostile cases

- **A browser tab, not the installed app.** Notifications are not possible
  there; the row says *home-screen app only* and settles.
- **Face ID refused.** `credentials.create` throws; the row says *not now*
  and settles; the nøøb sheet still offers it.
- **Both already set up** (a texted login before this build). Both rows tick
  at once; **got it** is there from the start.
- **`/passkey` or `/push` unticked.** The row for the missing feature is not
  drawn; the other stands alone.
- **This node unticked.** The second page as `/greetings` drew it.

## code description

`set-up.rs` — `greetings_sheet(2)` gains the two rows and `setup-wait`.
`set-up.js` — `feature_SetUp`: availability, the two enrol calls, the
settled test, re-applied on every apply. `set-up.css` — the rows.

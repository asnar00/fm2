# account
*the panel is a tool: a person button in the toolbar; the logo button is reserved for the agent*

> (transcripts/2026-08-14-fm-spec-3.md#p46)
> ok so: I just had a thought. The current "update" page (logged in as, etc) should have its own tool button in the tool-tray, instead of being driven off the little icon button. *but* let's keep the icon button, I want to use it later as an agent interface. Let's use a standard icon that people use for login (I guess a "person" silhouette?)

## spec

The system panel joins the toolbar as a proper tool — **account**, the standard person silhouette (👤) — instead of hanging off the corner logo button. Opening the account tool opens the `/panel` sheet; leaving it (the `‹`, or another tool) closes the sheet; dismissing the sheet by tapping the shade also leaves the tool, so toolbar state never lies about what's open. The corner logo button stays — `/watch`'s update pulse still lives there — but its tap is *reserved*: it will become the agent interface, and until that exists it does nothing while this feature is on. Untick this feature and the logo button opens the panel again, exactly as before.

## user

Tap the 👤 button in the toolbar for everything administrative: who's logged in, what's changed, update, log out. The little logo in the corner still glows when an update is waiting — but the person button is where you go.

## glossary

(uses `/tool` and `/toolbar` from `/tools`; the /system panel/ term is defined at `/muon`)

## code description

`account.rs` registers the tool: `tools_list` gains `{id: "account", label: "account", icon: "👤"}`.

`account.js` is the page half. It wraps `feature_Loop.apply` and watches the `open_tool` local var: on transition into `account` it calls `feature_Panel.open()`, on transition out, `feature_Panel.close()` (both typeof-guarded). It wraps `feature_Panel.close` so a shade-tap dismissal, when the account tool is still open, also sends `tools_home` — the wrap only fires while `open_tool` is `account`, so the close that follows the resulting state change cannot loop. It takes the corner button's seam (`feature_Panel.buttonTap = () => {}`): the tap is parked for the future agent interface.

The seam itself belongs to `/panel` (created for this node, default behaviour unchanged): the button's click goes through `feature_Panel.buttonTap`, which defaults to `open()`.

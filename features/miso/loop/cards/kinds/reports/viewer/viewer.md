# viewer
*a report opens inside the app, with ‹ to come back and a share button*

> (transcripts/2026-09-03-housekeeping.md#p14b)
> also, when viewing the report, there's no way to get back to the app.

## user

Tap **open** on a ready report and it opens over the app: the report itself, scrollable, with ‹ at the top-left to come back and a share button at the top-right that hands the PDF to the phone's share sheet.

## spec

`/reports` opened the PDF in a new tab so the phone's share sheet could reach it — and on the installed app a same-site "new tab" is the app's own window, so the PDF replaced the app and nothing led back (#p14b). One reading, so it builds. The report's own page — the HTML `/reports` prints from, self-contained down to its map tiles — is kept beside the PDF under the report's id after every generation, and served at `reports/view?id=…` to the report's owner and nobody else (the same lookup the PDF route makes). The **open** tap is taken on the page half: a full-screen sheet outside `#app`, a bar with ‹ and **share**, and the page in a frame beneath. ‹ closes it; **share** fetches the PDF and hands it to the phone's share sheet as a file (`navigator.share`), and where the browser cannot share files it opens the PDF the old way. A report made before this node has no page kept yet: the tap opens the PDF the old way, and its next run gives it one. Untick and **open** is the plain link again.

## hostile cases

- A report generated before this node: no kept page, the old link, until it is run again.
- Not the owner: the view route answers 404, as the PDF route does.
- Share unsupported (a desktop browser): the PDF opens in a new tab.
- A repaint while the sheet is up: the sheet lives outside `#app` and stays.

## glossary

(no new terms)

## code description

`viewer.rs` — wraps `reports_generate` (copy the printed page to `<id>.html` beside the PDF once the card says ready) and `route` (`reports/view`, owner-checked, `text/html`).

`viewer.js` — the sheet (made at load, outside `#app`), the capture-phase click on `.rep-doc` (head the view route; open the sheet, or fall back to the link), ‹ and share.

`viewer.css` — the sheet, its bar and frame.

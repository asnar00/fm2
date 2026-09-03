# on-release
*a tap sends when the finger lifts, not when the browser decides it was a click*

> (transcripts/2026-09-03-housekeeping.md#p3)
> screen taps still don't reliably press buttons, often requiring 2 or 3 taps to hit the button.

## user

Press a button and it presses — a quick jab or a firm, deliberate press, either one, the first time.

## spec

`/loop` sends a button's event on the browser's `click`, and on the phone that click is the browser's to give. Ash's black box (`/touches`, 2026-09-02 to 09-03) held 89 real presses on toolbar buttons: 65 clicked, 11 were rescued by `/lands`, 13 produced no click at all — and the thirteen are the ones held longest. Every press that clicked was down 114 ms or less; every press with no click was down 127 ms or more. The simulator repeats it exactly: a 50–110 ms tap on 👤 clicks, a 130 ms one and every longer one never does (scratchpad/holdtap). iOS hands a touch held past about 120 ms to a different recognizer, and that one keeps the click to itself. A firm press — the second, more deliberate try after the first seemed to miss — is the press it eats, which is why it read as "two or three taps".

So the tap is decided here, on `pointerup`, which arrives for every press. The `pointerdown` remembers the `data-ev` under the finger; a `pointerup` from the same pointer, within 12 px of where it went down (`/long-press`'s own drift), over an element that carries the same `data-ev` — found by point, since a touch pointer's `pointerup` targets the element it went down on, gone or not — dispatches a `click` on that element. One synthetic click, so every listener that reads clicks (`/loop`'s send, `/as-sub-tools`' doors, `/recentre`, the long-press swallows) runs as it always did. The browser's own click, if it comes, is the same tap twice: the first trusted click after a release is stopped at the window, before any listener; a new press clears that. A hold `/long-press` has fired still gets its click, aimed at the button it was armed on, so the swallow those nodes keep can consume it and reset — on the phone that click never came either, and the swallow was left armed for the next honest tap. A pointer that is cancelled, that drifted, that came up off the button, or that is not the primary button sends nothing; a scroll is not a tap. `/drive`'s taps are `el.click()`, no pointer, and are untouched. Untick and the browser's click is the tap again, and the firm press is lost again.

## hostile cases

- A quick tap on the phone: pointerup sends it; the native click 40 ms later is stopped at the window. One send.
- A press held 300 ms: pointerup sends it; no native click comes. One send.
- A hold of 500 ms: `/long-press` shows the card and marks `fired`; the release dispatches the click at the armed button; the swallow eats it and resets. No send.
- A finger that lands on a button and scrolls away: `pointercancel`, or a release more than 12 px off. Nothing sent.
- A repaint between down and up (the `/lands` case): the release finds the new button at the point, same `data-ev`. Sent once; the native click, landing on nothing, is stopped.
- A mouse (the gate, the desktop): pointerup sends, the native click is stopped. One send.
- A right button or a second finger: not primary, ignored.

## glossary

(no new terms)

## code description

`on-release.js` — capture-phase `pointerdown` (remember the event, the point, the pointer; disarm the stop), `pointercancel` (forget), `pointerup` (the checks above, then `dispatchEvent(new MouseEvent('click'))` at the element found by point, and arm the stop), and a capture-phase `click` on `window` that stops the next trusted click while armed.

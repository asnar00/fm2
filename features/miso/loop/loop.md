# loop
*the client runs an event loop: state → event → update → render*

> (transcripts/2026-08-13-fm-spec.md#p97)
> OK let's do the event core next

## spec

Miso's client stops being a render-once poster: DOM events flow into wasm, a Rust `update` chain transforms state, and the `render` chain redraws. The design is deliberately stateless-in-Rust: state is an opaque JSON string that round-trips through the page between calls — `boot()` yields the initial `{state, html}`, each event calls `on_event({state, event})` for the next. Features add behaviour by extending `update(state, event)` and appearance by extending `render(state)`, both via `existing`. Every transition is an explicit `(state, event) → state` pair — the exact shape /blackbox/ recording and replay need. Elements opt into interactivity with a `data-ev` attribute. `events` is a product commitment like `/serve`: the wasm place's entries (`entry=boot, event=on_event` in places.md) live here.

## user

For agents building miso features: give an element `data-ev="name"`, extend `update` to react to `{"type":"click","ev":"name"}` events, read and write your state under your own key, and extend `render` to draw from state. See `/tap` for the complete pattern in one node.

## glossary

- **event loop**: the cycle DOM event → `update` chain (new state) → `render` chain (new html) → screen.
- **state**: a JSON object, serialized, owned by no one place — each feature reads and writes its own keys; held by the page between wasm calls.

## code description

`events.rs` owns the loop's Rust side: `init()` (base state `{}`), `update(state, event)` (base: unchanged state), `boot()` (init → render → payload), `on_event(input)` (unwrap `{state, event}` → update → render → payload), and `event_payload` (the `{state, html}` JSON both exports return).

`events.js` owns the page side: `feature_Loop.boot()` fetches and instantiates the wasm, applies the boot payload, and installs one delegated click listener for `[data-ev]` elements; `send(event)` wraps state+event, calls `fm_event`, and applies the result; string passage uses the linker-generated `fm_alloc`.

The wasm exports themselves (`fm_entry` → `boot`, `fm_event` → `on_event`, `fm_alloc`) are linker-generated glue, declared by the product's places.md.

# patch
*a wasm-only release patches itself in live: new logic, same page, nothing reloads*

> (transcripts/2026-08-15-fm-spec.md#p6)
> let's do all three :-)

## user

Some updates now land like a thought rather than a restart: the build
number ticks over and the behaviour is new, without the screen so much
as blinking — even mid-recording.

## spec

When `/delta`'s diff shows the only code change is `client.wasm` — the
Rust logic, none of the page's JS or markup — a reload is ceremony: the
page, the fragments, the mic, the state are all still right. This node
hot-swaps instead: fetch the new module, instantiate it (the zero-
imports discipline deploy already smoke-tests), point the running loop
at it, stamp the build, and nudge one render so the new logic speaks.
The loop state is not stashed or rehydrated — it is simply **never
lost**. New state keys the new build would have seeded at boot start
absent and fall to their `unwrap_or` defaults, the standing fm idiom;
the next real reload converges them.

Anything less clean — the fetch fails, the module won't instantiate,
`/delta` is absent or blind — falls through to the full ritual
unchanged. The swap is also safe mid-task: recording, playback and
transcription live in JS and never notice.

This is the buildable rung of hot patching. The rest — patching live
JS features, which needs re-linkable chains (a registry instead of
closure capture) — is parked in notes.md as the mechanism the context
manager will also need; when that arrives, updates ride it for free.

## glossary

- **hot swap**: replacing the running wasm module in place, state
  carried by simply not touching it.

## code description

`patch.index.js` wraps `feature_Review.apply` (typeof-guarded on
`/delta`, whose manifests are the eyes here): it computes the delta;
if both manifests are known, `client.wasm` changed, and no OTHER code
path did, it runs `swap(build)` — fetch `client.wasm` no-store,
`WebAssembly.instantiate` with zero imports, verify `fm_entry`
exports, set `feature_Loop.instance`, then `feature_Delta.quiet(build)`
(evicts the delta's data files, stamps, becalms the handle) and a
`patch_resume` event nudges the render through the new module. Any
throw or a null instantiation falls through to the wrapped chain.

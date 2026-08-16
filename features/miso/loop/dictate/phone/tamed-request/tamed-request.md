# tamed-request
*the speech engine asks the GPU politely — request only what the adapter offers, and iPhones may say yes*

> (transcripts/2026-08-16-fm-spec.md#p8)
> ok let's do it (although I have to say I'm hoping we get to go the sovereign route anyway)

## user

Nothing to operate. If your phone's GPU was refusing the speech engine,
transcription may now run five to ten times faster — a note's words appear
in a blink instead of a few seconds. Phones that fell back to the slow
path before get one fresh audition automatically; if the GPU still
declines, the slow path continues as before and nothing is lost.

## spec

The refinement `/phone` named: ort's webgpu backend requests its device
with limits and features it never checks the adapter for, and iOS Safari
refuses the whole request — while the same phone runs raw WebGPU happily
(ftr's haze, our `/compute`). The experiment (this session, desktop
Chrome) made the request visible: `timestamp-query`, `shader-f16`,
`subgroups`, ten storage buffers per stage, 4GB buffers — asked for
unconditionally. WebKit grants several of those only partially or not at
all; any one refusal slams the door.

The tame: when the engine is about to build on webgpu, an optional taming
module wraps `GPUAdapter.requestDevice` with haze's exactly-what's-offered
recipe — requested features filter to the adapter's own set; max-limits
clamp down to the adapter's values; alignment (`min*`) limits clamp up;
unknown limits drop. Where the adapter offers everything (desktop Chrome)
the clamp changes nothing — proven byte-identical behaviour, ~6× faster
than wasm warm (223ms vs 1.4s on the fixture). Where it doesn't (iOS),
a doomed request becomes a best-effort grant; if whisper's kernels then
genuinely need what the phone lacks, they fail at run time and `/phone`'s
existing fresh-module wasm fallback catches them — no new failure modes.

Rejected on evidence: handing ort a pre-acquired device via its
`executionProviders: [{name: 'webgpu', device}]` API. The session builds,
but every inference dies in "Failed to wait for the operation:3" — the
bundle's future-wait machinery only tracks devices its own instance
created. Three descriptor variants failed identically; the door is the
descriptor, not the device.

One more duty: phones that failed before the tame carry a pinned `wasm`
device choice (`/phone` pins deliberately, and named clearing it as this
node's job). The taming module retires the pin once — one fresh webgpu
audition per install; a failed audition re-pins and is never repeated.

The field verdict belongs to a real iPhone — the simulator exposes
`navigator.gpu` with a null adapter, so desktop proves only
no-regression. Ash's phone decides whether the prize is collected. And
the sovereign path (`/compute` mel → matmul → attention) remains the
long game this shim buys time for, per the ask itself.

## glossary

- **taming**: clamping a GPU device request to exactly what the adapter
  reports — features intersected, limits bounded — so a refusal-prone
  descriptor becomes a grantable one.
- **audition**: the single automatic webgpu retry a previously-pinned-to-
  wasm device gets when the tame arrives.

## code description

`assets/stt/tame.js` (served beside the engine at `/stt/tame.js`) exports
`prepare()`: installs the `GPUAdapter.prototype.requestDevice` wrap once —
clone the descriptor, filter `requiredFeatures` through `adapter.features`,
clamp each `requiredLimits` entry to the adapter's value (`min*` keys clamp
upward, unknown keys drop), delegate; any shim-internal error falls back to
the unclamped original call. Then the pin duty: a stored
`muonSttDevice === "wasm"` is deleted once, guarded by a
`muonSttShimTried` marker so a genuinely wasm-bound device re-pins and
stays.

`/phone`'s `engine.js` gained the seam (revised for this node, behaviour
intact without it): `transcribe()` first tries
`import('./tame.js')` and calls `prepare()` — a 404 (this node unticked)
is caught and means the engine behaves exactly as before, the standing
absence-is-the-unticked-state discipline.

# phone
*the on-device transcription rung: a draft transcript with no network at all*

> (transcripts/2026-08-14-fm-spec-3.md#p47)
> cool. shall we look at transcription next? We can start with on-device transcription.

> (transcripts/2026-08-14-fm-spec-3.md#p63, draft-phase revision — the field found the bug)
> I tapped it and got a webgpu error - can you take me out of the loop and verify all this on your lovely ios simulator? ta

> (transcripts/2026-08-14-fm-spec-3.md#p64, draft-phase revision — the diagnosis corrected)
> except the iphone does run webgpu apps - take a look at ../ftr (you wrote it / we wrote it together) - that's webgpu and it runs nicely on the same iphone :-)

## user

Record a note and a rough transcript appears under its tile a few moments later — even in airplane mode. The first transcription after install downloads the speech model (~60MB, wifi recommended); after that it's local. Better transcripts replace rough ones automatically when the server rungs arrive.

## spec

The first rung of the graded transcription ladder (`/dictate` #p36–39), built first by choice — it is the offline-true floor. Ticked, it makes `transcribe_local()` reachable: recordings gain a draft transcript computed entirely on the device (whisper-tiny via transformers.js — WebGPU when the device has it, wasm otherwise; the engine and model are served from miso's own origin, never a third-party CDN). The scheduler stamps the result grade 1 — *a draft, honestly labelled* — and when a better rung (server, api) comes into reach later, the same recording is re-derived and upgraded in place. Transcription is compute, not hardware: it still must not run during `/replay`, because re-enactment sends no events.

Deliberate v0 gaps: the engine assets (~60MB) load through the network-first service worker, so an online session re-fetches them (a cache-first rule for `stt/` needs a seam in `/pwa`'s fetch handler — a coming refinement); transcripts are device-local (they don't mirror yet); a failed attempt is stamped as an empty grade-1 result so the scheduler moves on rather than retrying forever.

## glossary

- **rung**: one implementation of a derived value, ranked by quality; the scheduler runs the best rung currently reachable (defined at `/dictate`, the graded-derivation probe).

## code description

`phone.rs` is one redefinition: `transcribe_local()` returns "ready" — reachability is the feature being ticked.

`phone.js` watches `dict_transcribe` intent for rung `local` (replay-guarded): fetches the blob (`feature_Dictate.getBlob`), decodes and resamples to 16kHz mono float PCM (`AudioContext.decodeAudioData` + `OfflineAudioContext`), imports the engine on first use, and reports the result as a `Transcribed` event (`{id, text, rung: "local", grade: 1}`; failures send `failed: true`, empty text, and the error message for `/diag`). One job in flight at a time — the scheduler queues one file per pass.

`assets/stt/engine.js` (ES module) owns the engine: transformers.js with tokenizer + processor + model assembled by hand, models resolved from `/stt/models/` on our origin (`allowRemoteModels = false`), q4 weights. It runs on the main thread for now — ort's wasm backend refuses to initialize inside a module worker — so the wasm device blocks for a few seconds per note; webgpu compute is async. The heavy artifacts beside it (`transformers.min.js`, the ort wasm pair, the whisper-tiny.en model) are fetched by `tools/fetch_stt.py` (pinned npm + huggingface revisions) and gitignored — the repo carries the recipe, not the binaries.

Device choice survived contact with the field (#p63: a real iPhone handed over a webgpu adapter, and ort's webgpu backend then failed). The diagnosis, corrected by #p64: **the phone's GPU is fine** — ftr's haze runs raw WebGPU (Rust/wgpu) nicely on the same iPhone. The instructive contrast is in haze's own device request (ftr repo, `haze/src/renderer.rs:536-556`): it asks for `required_limits: adapter.limits()` — *exactly what this adapter offers, nothing assumed* — and only opportunistically-present features, which is why it runs on every WebGPU Safari. ort's jsep backend requests its own device internally with the limits it wants; iOS refuses something in that request, and the backend dies where haze thrives. This also bounds what our probe can promise: a default-limits `requestDevice` succeeding says nothing about ort's richer request — the run-time fallback covers that gap by construction. **Making ort's webgpu path work on iOS is a named refinement with a real prize (~5-10x faster transcription)**: the investigation starts at ort's webgpu env hooks (it accepts an externally-chosen adapter; the question is whether its device request can be tamed to haze's exactly-what's-offered recipe).

*(Revision, 2026-08-16 #p8: that refinement is now `/tamed-request` — `engine.js` gained a small seam, `transcribe()` first tries the optional `./tame.js` module and proceeds untouched when it's absent; the clearing-the-pin duty named below belongs to that node, as predicted. Second seam, #p13: `engine.js` also exports `lastDevice()`, reporting the device of the engine actually built, for `/engine-receipts` to put in its reports.)*

Mechanically: the probe demands a full adapter **and** device grant before trusting webgpu (the simulator showed Safari exposing `navigator.gpu` with a null adapter — claims without capability exist in both directions), and on any webgpu failure, load-time or run-time, the engine escapes v4's memoized-failure trap by re-importing transformers with a cache-busting query (a fresh module is a fresh memo), rebuilding on wasm, retrying once, and pinning the working device in `localStorage.misoSttDevice` so later sessions skip the broken path. Verified three ways: headless Chrome wasm path (~2.7s, correct text), headless Chrome with webgpu *forced* where none exists (fallback rescues, correct text, device pinned), and iOS Simulator WebKit (~2.5s, correct text). Fallback costs, accepted: the rescue re-fetches the model files that session, and a pinned `wasm` means the phone won't retry webgpu even after the jsep refinement lands — clearing the pin is part of that future node's job.

Hard-won transformers-v4 facts, recorded so nobody re-pays for them: `env.localModelPath` must be scheme-less (a full URL silently skips the local existence probe and the tokenizer vanishes); `pipeline()` soft-fails its processor load (assemble components by hand); a failed `from_pretrained` is memoized by model id (probe the device first, never try-catch-ladder); the q8 decoder trips a MatMulNBits bug in ort 1.26-dev (use q4); the ort `.wasm` binaries live in `onnxruntime-web`'s package, not transformers' dist — and for the wasm path they turn out to be embedded in the bundle anyway (the shipped pair is insurance for the webgpu path).

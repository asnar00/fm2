# sovereign.md — miso's own inference runtime, and whisper on it

*The plan for replacing onnxruntime with kernels we write, own, and can
debug. Started 2026-08-16 — the decision at
transcripts/2026-08-16-fm-spec.md#p15 ("remove the ort dependency and
stand up our own sovereign webgpu runner"), this document at #p16
("a bit of up-front design work… before we fire into it") — after the
ort shim experiment ended in a third day of other-people's-mystery.
This file is the map; the rungs become nodes as they are built. Edit it
freely as the ground truth changes — it is a plan, not doctrine.*

---

## 1. Why

Two days went into making someone else's runtime work on a phone. The
score: one undocumented device-request refusal (`/tamed-request`), one
misleading `webgpuInit is not a function` that turned out to be a MIME
type (`/module-mime`), a memoized-failure trap, a q8 kernel bug, a
tokenizer that vanishes when a path has a scheme, and a 26MB binary
nobody in this repo can read. Every one of those cost hours, and not one
of them taught us anything we can reuse.

Against that, the parts we built ourselves — `/compute`'s driver,
`/semantic-find`'s embedder and kernel — went in cleanly, ran on the
first phone we tried, and are debuggable by reading them.

The decision (#p15): **remove ort, stand up our own WebGPU runner, and
put whisper on it.** This is the T2 rung notes.md mapped on day 2, taken
deliberately rather than by drift.

What we get beyond speed: `client.wasm`'s zero-import law stays
untouched (WebGPU is a JS API, no bindgen), the ~40MB of vendored ort
and transformers binaries leave the tree, feature-modular WGSL gets its
first real tenant, and every future model — `/compute`'s T3 rung, a
FunctionGemma-class thing — lands on machinery we already own.

**Non-goals.** Not a general ONNX runtime: we run *this* graph. Not a
training system. Not a framework for others. Not multilingual whisper
(tiny.en only, until it works).

---

## 2. What already exists

- **`/compute`** — the driver: acquire the device with exactly the
  adapter's limits (the recipe that survives iOS), compile WGSL to a
  cached pipeline, run one kernel, admit absence. Proven on device.
- **`/semantic-find`** — the first tenant, and the pattern to copy: a
  model reduced to a table, a Python twin (`tools/potion_embed.py`) that
  reads the same table and walks the same algorithm, and the deploy-side
  and device-side halves agreeing *by construction, not by hope*.
- **`tools/fetch_stt.py` / `fetch_find.py`** — the vendoring pattern:
  pinned upstream revisions, binaries gitignored, the recipe in git.
- **`/engine-receipts`** — device telemetry, built this afternoon. It
  will measure our kernels exactly as it measured ort's.
- **`/dictate`'s graded ladder** — a scheduler that picks the best
  reachable rung and re-derives when a better one arrives. The sovereign
  engine is a rung; ort stays reachable until it isn't needed.

## 3. What has to be built

Four layers, bottom up. Each is a node, each toggles off cleanly.

**L0 — resident tensors and dispatch sequences.** Today `run()` uploads
every input and reads back the output on every call. A four-layer
encoder-decoder does hundreds of dispatches per note over weights that
must *stay* on the GPU. This is the crux rung: the driver grows
allocate / upload / bind / dispatch-many / read-once, while `run()`
keeps working exactly as it does for `/semantic-find`.

**L1 — the neural op library.** The WGSL kernels: matmul (tiled),
layernorm, softmax (masked, row-wise), GELU, conv1d, elementwise
add/mul, transpose, argmax. Each with a CPU twin for verification and a
shape-agnostic parameterisation (see §6). This is where feature-modular
WGSL is born — `compute.md` named it and said "when the first
multi-feature kernel arrives, not speculatively". It has arrived.

**L2 — the model.** Whisper-tiny.en's graph assembled from L1 ops:
mel frontend, encoder, decoder with KV cache, greedy loop, BPE detokenizer.

**L3 — the rung.** The `/dictate` integration: reachability, scheduling,
and eventually the retirement of ort.

---

## 4. Node placement (proposal — §10 asks ash to rule)

```
miso/loop/compute/
  resident/        L0  buffers that live on the GPU, multi-dispatch sequences
  nn/              L1  the op library (WGSL + CPU twins)
miso/loop/dictate/
  whisper/         L2+L3  weights, graph, decode loop, tokenizer, the rung
```

`/compute` goes to 3 children (semantic-find, resident, nn); `/dictate`
to 4 (mirror, phone, transcript, whisper). Both inside the cap.

`whisper/` will grow subfeatures as it is built (`mel`, `encoder`,
`decoder`, `tokens`) — which is the honest shape: each is a prompt, each
is separately toggleable, and each has its own acceptance test.

**Why the model lives under `/dictate`, not `/compute`:** `compute.md`
says tenants arrive as subfeatures carrying their own kernels, and by
that reading whisper belongs under `/compute`. But the *general* kernels
(matmul, layernorm) are not speech — they are the substrate's own
vocabulary, and a second model would want them untouched. So the split
is: reusable math under `/compute`, the speech model under `/dictate`
where its rung already lives. Flagged for ash because it reads against
compute.md's letter.

---

## 5. The model, precisely

*Constants below are from memory and are NOT to be trusted as written.
The Python reference of §7 is the authority: it reads the real
checkpoint and prints the real shapes, and every number here must be
confirmed against it before a kernel is written. Recorded anyway so the
implementer knows what to confirm.*

**whisper-tiny.en** — ~39M parameters, English-only.

| | |
|---|---|
| d_model | 384 |
| heads | 6 (head_dim 64) |
| encoder layers | 4 |
| decoder layers | 4 |
| MLP hidden | 1536 |
| vocab | 51864 |
| audio context | 1500 frames |
| text context | 448 tokens |

**Frontend.** 16kHz mono. Pad/trim to exactly 30s (480,000 samples).
STFT: n_fft 400 (25ms), hop 160 (10ms), periodic Hann → 3000 frames ×
201 bins. Magnitude squared → 80-bin mel filterbank → `log10`, clamped
below at 1e-10, then floored at `max - 8.0`, then `(x + 4.0) / 4.0`.
Output 80 × 3000.

**Encoder.** conv1d(80→384, k=3, pad=1) + GELU; conv1d(384→384, k=3,
stride=2, pad=1) + GELU → 1500 × 384. Add fixed sinusoidal positional
embedding. Then 4 × residual block:
`x += attn(ln1(x))` (self, unmasked); `x += mlp(ln2(x))`. Final
`ln_post`.

**Decoder.** Learned token embedding + learned positional embedding.
4 × residual block: `x += selfattn(ln1(x))` (causal mask);
`x += crossattn(lncross(x), encoder_out)`; `x += mlp(ln2(x))`. Final
`ln`. Logits = `x @ token_embedding^T` (tied weights).

**Attention detail that bites:** whisper's key projection has **no
bias** (query, value, output do). Scaling is `head_dim^-0.25` applied to
*both* q and k before the product, not `head_dim^-0.5` after. Get either
wrong and the output is plausible-but-wrong — the worst failure mode.

**Decoding.** Prompt for a `.en` model is `<|startoftranscript|>`
`<|notimestamps|>`. Greedy argmax, append, stop at `<|endoftext|>` or
448 tokens. Self-attention KV is cached per layer per step;
cross-attention K/V is computed **once** from the encoder output and
reused every step (this is most of the decoder's speed).

**Tokenizer.** GPT-2 byte-level BPE. We need **decode only** — the
prompt is fixed special-token ids. Decode is: id → token string →
byte-level unmapping → UTF-8. A table, not an algorithm.

---

## 6. Numerics, weights, and shapes

**Stage the risk: correctness first, size second.**

1. **f32 everywhere** for bring-up. ~155MB of weights — unshippable, but
   it removes an entire class of bugs while the graph is being proven.
   Desktop-only, behind a dev flag.
2. **int8 with per-row scales** once f32 is correct — the exact scheme
   `/semantic-find` already ships for its token table, dequantised in
   the shader. ~40MB, comparable to the ort assets we delete, and the
   token embedding (half the model at 51864 × 384) benefits most.
3. **f16** only if a device proves it helps; never assume `shader-f16`.

**Accumulate in f32** regardless of storage precision.

**Watch `maxStorageBufferBindingSize`.** The token embedding in f32 is
~80MB — inside desktop limits, uncomfortably close to some mobile ones.
Another reason quantisation is rung 2, not rung 9.

**Shapes are parameters, not source text.** The pipeline cache is keyed
on WGSL source, so baking shapes into the string mints a new pipeline
per shape — dozens of compilations on a phone. Kernels take a small
uniform (or a leading storage buffer) of dimensions instead. This is a
requirement on L0's binding model, decided before L1 is written.

**Workgroups.** 64 is the current convention; matmul wants 2D tiles
(16×16 = 256 invocations, safely inside every `maxComputeInvocations`
we've seen). L0 must therefore accept per-dispatch workgroup counts in
x/y/z rather than assuming `ceil(n/64)`.

---

## 7. Verification: the discipline that makes this possible

A transformer that is subtly wrong produces fluent nonsense. Timing
tells you nothing; the text looks like text. So every stage is checked
against a reference **before** the next stage is built.

**`tools/whisper_ref.py`** — the Python twin, in the `potion_embed.py`
tradition. Reads the same converted weights we ship, runs the same math
in numpy, and dumps golden tensors at every boundary: mel, conv1, conv2,
each encoder block, encoder output, each decoder step's logits, final
token ids. No torch dependency at run time if we can avoid it (numpy
only), so the reference is as readable as the kernels.

**A comparison harness in the browser** that runs our implementation
against a golden set and reports max absolute and relative error per
stage — the same shape as the drive/demo scripts already in `tools/`.

**Per-rung acceptance is numeric, not vibes.** A rung is done when its
stage matches the reference within tolerance on a fixed fixture, and the
toggle test passes. Suggested tolerances: 1e-4 relative through the
encoder, and — the only one that really matters — **the final text must
match the reference exactly** on the fixture set.

**Fixtures.** A handful of short WAVs (the `say`-generated one used
today, plus a real field recording), their goldens generated on demand
by the tool and gitignored like the weights.

---

## 8. The ladder

Each rung is a node with a falsifiable acceptance test. Rungs 1–3 are
infrastructure and can be built without touching whisper at all.

| # | rung | done when |
|---|---|---|
| 1 | **resident** — GPU-resident buffers, multi-dispatch, dimension uniforms | a two-kernel chain (scale then sum) runs with one upload and one readback; `/semantic-find` still passes unchanged |
| 2 | **feature-modular WGSL** — `.wgsl` as a composition target: chains, `existing.fn()`, provenance comments | a kernel assembled from two nodes produces byte-identical WGSL to the hand-written one, and unticking the second removes its function |
| 3 | **nn** — matmul, layernorm, softmax, gelu, conv1d, argmax + CPU twins | every op matches its numpy twin to 1e-5 on random inputs of awkward shapes (non-multiples of the tile size) |
| 4 | **weights** — `tools/fetch_whisper.py` + converter + manifest | the manifest's shapes match §5's table; f32 blobs load and checksum |
| 5 | **mel** — PCM → log-mel (CPU first, GPU later) | matches `whisper_ref.py`'s mel to 1e-4 on all fixtures |
| 6 | **encoder** | encoder output matches the reference to 1e-4 |
| 7 | **decoder** — blocks, cross-attention, KV cache, greedy loop | first-step logits match to 1e-3; full greedy output matches token-for-token |
| 8 | **tokens** — BPE detokenizer | reference text reproduced exactly from the reference's token ids |
| 9 | **the rung** — wire into `/dictate` as a reachable rung, behind `/compute` availability | a real recording transcribes on device; `/engine-receipts` reports `device: "sovereign"` with its timing |
| 10 | **int8** | size within ~10% of target, text still exact on fixtures |
| 11 | **retire ort** | `/phone` unticked, assets gone, the tree smaller than it was |

Rungs 1–3 are the ones that make or break the schedule. Rungs 5–8 are
grind, but they are *our* grind: every failure is a line we wrote.

---

## 9. Risks, and what we do about them

- **Silent numerical wrongness.** The whole of §7 exists for this. No
  rung advances on "it produced text".
- **Weight download size.** No regression against today (~40MB of ort
  assets go away as ~40MB of int8 weights arrive), but rung 1's f32
  stage is desktop-only and must never reach a phone.
- **Mobile buffer limits.** Measure early: rung 1 should print the
  device's real limits into `/engine-receipts` so we learn them from the
  field rather than from a spec sheet.
- **Long dispatches getting killed.** Chunk the encoder; never submit an
  unbounded pass. Watch for it on the phone specifically.
- **Shader compilation cost on device.** Fewer, parameterised pipelines
  (§6) rather than many specialised ones; measure compile time in the
  receipts.
- **The climb stalling half-built.** Mitigated structurally: ort stays a
  reachable rung until rung 11, so the app always transcribes. No rung
  is allowed to break the working path.
- **Scope creep into a general runtime.** §1's non-goals are load-bearing.
  We run this graph.

---

## 10. Open decisions for ash

1. **Placement** (§4): reusable math under `/compute`, the model under
   `/dictate` — against `compute.md`'s letter that tenants carry their
   own kernels. Ratify or redirect.
2. **Feature-modular WGSL now or later** — rung 2 as written builds the
   composition mechanism *before* the kernels that need it, so the
   kernels are born modular. The alternative is hand-written WGSL first
   and a migration after. Doctrine says mechanisms arrive when their
   first real user does; the first real user is rung 3.
3. **Where the sovereign engine sits in the graded ladder** — a new rung
   at the same grade as `local` (an implementation choice, not a quality
   tier) needs the scheduler to prefer by *availability*, not just
   grade. Small design, wants a ruling.
4. **f32 desktop-only bring-up** — acceptable, or should rung 4 go
   straight to int8 and pay the debugging cost?
5. **This file's status** — a living plan in the repo (current
   assumption), or should the rungs move into `notes.md` and this file
   stay a one-page charter?

---

*Companion reading: `notes.md`'s T1–T3 map (the original three-tier
plan), `compute.md` (the driver and its named WGSL-composition future),
`semantic-find.md` (the pattern this whole document generalises), and
`/phone`'s hard-won ort facts — kept as a record of what we are leaving
behind.*

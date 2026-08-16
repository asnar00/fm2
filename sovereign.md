# sovereign.md — miso's own inference runtime, and live speech on it

*The plan for replacing onnxruntime with kernels we write, own, and can
debug. Started 2026-08-16 — the decision at
transcripts/2026-08-16-fm-spec.md#p15 ("remove the ort dependency and
stand up our own sovereign webgpu runner"), this document at #p16, the
real-time requirement at #p17, and the research that reshaped it at #p18.
A living plan, not doctrine: edit freely as the ground truth changes.*

**Revision 2 (#p18).** The first draft targeted whisper-tiny.en from
memory. Research replaced that choice, and most of the numbers in it.
The target is now **Moonshine v2**, and the reasons are in §5.

---

## 1. Why

Two days went into making someone else's runtime work on a phone. The
score: an undocumented device-request refusal (`/tamed-request`), a
misleading `webgpuInit is not a function` that was really a MIME type
(`/module-mime`), a memoized-failure trap, a q8 kernel bug, a tokenizer
that vanishes when a path has a scheme, and a 26MB binary nobody here can
read. None of it reusable.

Research then found the deeper fact: **onnxruntime-web's WebGPU has never
officially supported iOS** ([#22776](https://github.com/microsoft/onnxruntime/issues/22776)),
and its JSEP path pins CPU at 400% and grows memory to 14GB on WebKit 26
until the process dies ([#26827](https://github.com/microsoft/onnxruntime/issues/26827)).
We were never going to win that fight. The sovereign turn was correct on
the evidence, not merely on taste.

Against that, the parts we built ourselves — `/compute`'s driver,
`/semantic-find`'s embedder and kernel — went in cleanly, ran on the
first phone we tried, and are debuggable by reading them.

**The goal (#p17): live transcription.** While a conversation happens,
transcribe continuously, semantic-search the on-phone database, and
surface talking points as bullets — *"state of the roads round here is
terrible"* → road-management facts, instantly. Transcription is not the
product; **retrieval is**, and that changes the engineering (§6).

**Non-goals.** Not a general ONNX runtime: we run *this* graph. Not
training. Not a framework for others. English-only until it works.

---

## 2. What already exists

- **`/compute`** — the driver: acquire the device with exactly the
  adapter's limits, compile WGSL to a cached pipeline, run one kernel,
  admit absence. Proven on device.
- **`/semantic-find`** — the retrieval half of the product, already
  built: embeds a query on-device and scores the catalog with our own
  WGSL kernel. **The conversation tool is this, fed by live speech.**
- **The Python-twin pattern** (`tools/potion_embed.py`) — deploy-side and
  device-side agreeing by construction, not by hope. §8 generalises it.
- **`tools/fetch_stt.py`** — the vendoring pattern: pinned revisions,
  binaries gitignored, recipe in git.
- **`/engine-receipts`** — device telemetry, built today. It will measure
  our kernels exactly as it measured ort's, and it already earned its
  keep twice.
- **`/dictate`'s graded ladder** — ort stays a reachable rung until the
  sovereign one is better. The app never stops transcribing mid-climb.

---

## 3. What has to be built

**L0 — resident tensors and dispatch sequences.** Today `run()` uploads
every input and reads back the output on every call. Inference needs
weights that *stay* on the GPU and hundreds of dispatches per submit.
This is the crux rung; `run()` keeps working unchanged for
`/semantic-find`. **Its governing rule is static allocation**: every
buffer created once at startup and reused forever, nothing allocated
per-inference — which §10 argues is what keeps us out of the crash class
that kills every general-purpose runtime on iOS.

**L1 — the op library.** Moonshine v2 needs roughly **ten kernels**:
matmul, layernorm, softmax, erf-GELU, SiLU/sigmoid, causal conv1d,
asinh/exp, CMVN reduce, partial RoPE, embedding gather, argmax. Each with
a CPU twin. This is where feature-modular WGSL is born — and it is not a
luxury: **WGSL has no preprocessor**, and both prior hand-written-WGSL
projects (Ratchet, Llamas-on-the-Web) independently built one and
manually unrolled their loops because the compiler won't unroll
variable-bound loops.

**L2 — the model.** Moonshine v2's graph: conv stem over raw audio,
sliding-window encoder, decoder, greedy loop, detokenizer.

**L3 — live.** Rolling capture, provisional/finalized text, and the
hand-off to `/semantic-find` that is the actual product.

---

## 4. Node placement (proposal — §11 asks ash to rule)

```
miso/loop/compute/
  resident/        L0  GPU-resident buffers, multi-dispatch, param arena
  nn/              L1  the op library (WGSL + CPU twins)
miso/loop/dictate/
  moonshine/       L2  weights, graph, decode loop, tokenizer, the rung
  live/            L3  rolling capture, streaming loop, provisional text
```

`/compute` goes to 3 children, `/dictate` to 5 — both inside the cap.
`moonshine/` grows subfeatures as it is built (`encoder`, `decoder`,
`tokens`), each a prompt with its own acceptance test.

**Why the model lives under `/dictate`, not `/compute`:** `compute.md`
says tenants arrive as subfeatures carrying their own kernels, and by
that reading the model belongs under `/compute`. But the *general* kernels
(matmul, layernorm) are the substrate's own vocabulary, and a second
model would want them untouched. Flagged because it reads against
compute.md's letter.

---

## 5. The model: Moonshine v2

[arXiv 2602.12241](https://arxiv.org/abs/2602.12241) (Feb 2026),
**MIT licensed**, English-only, safetensors weights.

| | params | fp16 | int8 | OpenASR avg WER | LS-clean | latency (M3) |
|---|---|---|---|---|---|---|
| **v2 tiny** | 44.1M | 88MB | 50MB | 12.01 | 4.49 | **50ms** |
| v2 small | 140.1M | 282MB | 199MB | 7.84 | 2.49 | 148ms |
| v2 medium | 265.9M | 534MB | 296MB | 6.65 | 2.08 | 258ms |
| *whisper-tiny.en* | *38M* | — | — | *12.81* | *5.66* | *289ms* |

**v2 tiny beats whisper-tiny.en on accuracy _and_ is ~6× faster**, at a
similar size. int8 costs only +0.1–0.35pp WER. Estimated real-time
compute is ~1.4 GFLOP/s (tiny) against an iPhone GPU's ~2–4 TFLOP/s —
enormous headroom, which is what makes continuous listening plausible.

**Why it suits us structurally**, which matters more than the numbers for
a project that hand-writes every kernel:

- **Sliding-window attention** — bounded time-to-first-token regardless of
  utterance length. Whisper's full attention makes TTFT grow with the
  prefix, which is the structural reason streaming whisper is awkward and
  lands at ~3.3s ([Whisper-Streaming](https://aclanthology.org/2023.ijcnlp-demo.3.pdf)).
  A 16-left/4-right window is a *tiny fixed-size* attention kernel, not a
  1500×1500 one.
- **No positional embeddings in the encoder at all** (translation-
  invariant); position enters only before the decoder, via partial RoPE.
- **No mel spectrogram and no FFT** — Moonshine takes **raw audio**
  through a convolutional stem. This deletes an entire rung, and with it
  the whole class of window/hop/filterbank mismatches that silently
  poison a frontend.
- **Fixed shapes** — no KV-length-dependent pipeline recompiles.
- **50Hz features → 80ms algorithmic lookahead**; the encoder emits
  *provisional* states immediately and *finalized* states once right
  context arrives, which is exactly the shape §6 needs.

**Absent from the op list, and each one is a rung we don't build:** FFT,
mel filterbank, transducer joiner, LSTM, beam search, WFST decoder,
relative-position attention.

*Every constant above is second-hand until `tools/moonshine_ref.py`
prints it from the real checkpoint. The reference is the authority (§8).*

**Rejected, with reasons.** Whisper (batch model, TTFT grows with prefix).
Silero STT (CC BY-NC-SA — non-commercial; Silero *VAD* is genuinely MIT
and still useful as a gate). Vosk/Kaldi (needs an HCLG WFST plus a second
acoustic model). SenseVoice/Paraformer (non-OSI weights despite MIT
source). Kroko (CC-BY-SA). Distil-Whisper (its encoder is byte-identical
to whisper-large-v3's — distillation removed decoder layers, so it does
nothing for streaming, where the encoder is the term that matters).
**Fallbacks, ranked.** If Moonshine disappoints in the field:
`speechbrain/asr-streaming-conformer-librispeech` (Apache-2.0, a full
latency/WER curve on test-clean: 3.80 at 320ms, 3.51 at 480ms, 3.09 at
1280ms) is the best *genuinely permissive* alternative, at the cost of
RNN-T decode complexity. `sherpa-onnx-streaming-zipformer-en-20M`
(Apache-2.0 on both source and export, ~20M params, 3.88/9.53) is the
smallest permissive option — but note the trap: **the streaming
Zipformer-CTC checkpoints, the ones with genuinely simple decoding, are
Chinese-only**; the English streaming Zipformers are all transducers, so
that family's simple-decode escape hatch doesn't exist for us. And if
"permissive" means commercially-usable rather than literally MIT/Apache,
`nvidia/stt_en_fastconformer_hybrid_large_streaming_multi` (114M,
CC-BY-4.0, selectable 0/80/480/1040ms latency, CTC head whose streaming
and offline predictions are identical by construction) reopens.

---

## 6. Real time, and what the platform allows

**The requirement reframed.** The product is retrieval, not transcript.
Two consequences worth exploiting deliberately:

- **Retrieval tolerates error; it does not tolerate latency.** A wrong
  word barely moves an embedding — "rhodes" still finds road facts. We
  can spend accuracy on speed in ways dictation never could, and v2 tiny
  is therefore likely sufficient where a dictation product would need
  small or medium.
- **The retrieval half already exists.** `/semantic-find` embeds and
  scores on-device today. L3 is a hand-off, not a second system.

**The loop.** AudioWorklet captures at the hardware's **48kHz** (asking
getUserMedia for 16k can silently yield *no audio* on iOS) and does no
DSP in its 128-frame quantum — it ring-buffers to a worker that resamples
3:1 to 16kHz. A VAD gate (energy first; Silero VAD, MIT, 309K params, if
that proves insufficient) skips silence to save battery. The encoder runs
on rolling audio, emitting provisional states; text stabilises behind a
**local-agreement commit** (a prefix is committed when two consecutive
updates agree), and provisional text must *render* as provisional — the
house rule about never lying about what exists applies to words we might
retract.

**Verified iOS platform facts** (measured percentages are field-share
figures from the research, not our own device — rung 1 replaces them):

- WebGPU ships **enabled by default on iOS 26+**; on 17.4–18.7 it exists
  but is flag-disabled. **Confirmed in our own field data (#p18): an
  installed standalone PWA on ash's iPhone acquired an adapter *and* a
  device.** The delivery vehicle is viable.
- **`shader-f16` is at ~100% on iOS**, and Apple explicitly recommends
  f16 storage to avoid memory-pressure termination. But **f16
  accumulation gives incoherent output on Apple GPUs** — store f16,
  accumulate f32, and watch the ~65,504 ceiling.
- **`subgroups` is at ~0.02% on iOS** (the ~100% Metal-3 figure is macOS;
  don't be misled by the vendor number). **Write every reduction —
  softmax, layernorm, matmul accumulation — with workgroup shared memory.**
- **Measured iOS limits: `maxBufferSize` 256MB, `maxStorageBufferBindingSize`
  ~307MB** — not the 128MB spec default that gets quoted. v2 tiny at fp16
  (88MB) fits one buffer trivially; medium would need splitting. The
  widely-cited "256MB on iPhone 6" table is from a 2021 Metal issue and is
  not current WebGPU behaviour.
- **The real ceiling is ~500MB of Safari tab memory**, not buffer limits.
- **Dispatch overhead ~32µs on Safari/Metal**, and at batch=1 it dominates
  everything regardless of kernel quality — one study cut dispatches
  876→564 for +53% throughput. **Dispatch count is the primary design
  variable.** Elementwise fusion buys nothing on Metal; fuse at the
  tiled-GEMM level. Consider parakeet.js's split — encoder on WebGPU,
  autoregressive decoder on CPU/WASM — precisely because batch=1 decode is
  the regime where dispatch overhead beats arithmetic.
- **~17% of fp32 peak** is the realistic WGSL matmul ceiling without
  subgroups (manual unrolling, 8×8 per-thread tiles). Keep workgroup
  memory ≤16KB of the 32KB/core budget or occupancy collapses; workgroup
  size a multiple of 32.
- **No push constants** (use one rotating uniform arena with dynamic
  offsets), **no buffer aliasing** (ping-pong, no in-place ops), no
  bf16/u8/u16.
- **Cache API is capped ~50MB on iOS** — weights live in **OPFS**,
  streamed to GPU through ~1MB staging buffers, never materialised in the
  JS heap. **Origin data is evicted after 7 days without interaction**: a
  PWA unopened for a week re-downloads its model. That is a product fact.

**The background-audio wall, which is a product decision.** iOS suspends
Web Audio the moment the screen locks or Safari backgrounds, and **there
is no PWA workaround** — no background-audio entitlement exists for web
content. Ambient listening with the phone in a pocket **requires a native
shell**. For canvassing the phone is presumably in hand with the bullets
on screen, so foreground-only may be entirely acceptable — but it changes
what miso *is*, and deciding it late would be expensive.

---

## 7. Numerics and weights

1. **fp16 storage, f32 accumulate**, from the start — Apple recommends the
   first and the second is *required* for coherent output on Apple GPUs.
   v2 tiny at fp16 is 88MB, comfortably inside every limit above. (The
   first draft proposed f32 bring-up; at 88MB fp16 there is no reason.)
2. **int8** later: 50MB for +0.1–0.35pp WER, using the per-row-scale
   scheme `/semantic-find` already ships.
3. **Shapes are parameters, not source text.** The pipeline cache is keyed
   on WGSL source, so baking shapes in mints a pipeline per shape — dozens
   of compilations on a phone. Kernels take dimensions in a uniform.
   Moonshine's fixed shapes make this easy; it is still required for the
   autotuner.
4. **Autotune at startup** — sweep workgroup size, tile dimensions and
   vector width over a few dozen candidates for ~1s, cache the winner in
   localStorage keyed on the adapter string. Published gains: 2.77×
   (nnJIT) and 41% (Llamas-on-the-Web) over hand-picked parameters.

---

## 8. Verification: the discipline that makes this possible

A transformer that is subtly wrong produces fluent nonsense. Timing tells
you nothing; the text looks like text. Every stage is checked against a
reference **before** the next is built.

**`tools/moonshine_ref.py`** — the Python twin, in the `potion_embed.py`
tradition: reads the same converted weights we ship, runs the same math in
numpy, and dumps golden tensors at every boundary — conv stem, each
encoder block, encoder output, each decode step's logits, final token ids.
It is also the authority on every constant in §5.

**A browser comparison harness** that runs our kernels against the goldens
and reports max absolute and relative error per stage.

**Per-rung acceptance is numeric.** Suggested tolerances: 1e-4 relative
through the encoder; and the only one that really matters, **the final
text must match the reference exactly** on the fixture set.

**Fixtures:** short WAVs including a real field recording, goldens
generated on demand and gitignored like the weights.

**A second opinion is available.** Community ONNX and GGUF exports of the
Moonshine streaming checkpoints exist (`Workmind/moonshine-streaming-small-ONNX`,
`handy-computer/moonshine-streaming-*-gguf`). We convert safetensors
ourselves, but those exports are worth keeping as an independent
cross-check when a kernel disagrees with the twin and it isn't obvious
which of the two is wrong.

---

## 9. The ladder

| # | rung | done when |
|---|---|---|
| 1 | **probe** — dump the real device: `adapter.limits`, features, a matmul microbenchmark, and a **10,000-dispatch soak test** | we have real numbers from ash's iPhone in `/engine-receipts`, and the soak survives (see §10) |
| 2 | **resident** — GPU-resident buffers, multi-dispatch, param arena, dimension uniforms | a two-kernel chain runs with one upload and one readback; `/semantic-find` unchanged |
| 3 | **feature-modular WGSL** — `.wgsl` as a composition target, plus the preprocessor/unroller everyone else needed | a kernel assembled from two nodes is byte-identical to the hand-written one; unticking the second removes its function |
| 4 | **nn** — the ten kernels + CPU twins | each matches its numpy twin to 1e-5 on awkward shapes (non-multiples of the tile) |
| 5 | **weights** — `tools/fetch_moonshine.py` + converter + manifest, fp16 | shapes match what `moonshine_ref.py` prints; blobs load from OPFS and checksum |
| 6 | **encoder** — conv stem, sliding-window blocks | encoder output matches the reference to 1e-4 |
| 7 | **decoder** — RoPE, cross-attention, greedy loop | first-step logits to 1e-3; full greedy output matches token-for-token |
| 8 | **tokens** — detokenizer | reference text reproduced exactly from reference token ids |
| 9 | **the rung** — wired into `/dictate`, batch transcription of a recording | a real recording transcribes on device; receipts report `device: "sovereign"` with timing |
| 10 | **live** — capture, VAD, rolling encode, provisional/finalized commit | live text on screen with measured end-to-end latency and a stable committed prefix |
| 11 | **the conversation tool** — live text → `/semantic-find` → bullets | *"the roads round here are terrible"* surfaces road facts on the phone, in conversation |
| 12 | **int8** | ~50MB, text still exact on fixtures |
| 13 | **retire ort** | `/phone` unticked, ~40MB of assets gone, the tree smaller than it was |

Rungs 1–4 make or break the schedule. Rung 11 is the actual product;
everything before it is scaffolding for that sentence.

---

## 10. Risks

- **The 500-inference crash.** [onnxruntime#27584](https://github.com/microsoft/onnxruntime/issues/27584)
  reports a crash after ~500 inferences on iOS Safari 26.3 WebGPU, not
  reproducible on Chrome, root cause unpublished. A streaming loop at ~120
  encoder passes/minute would hit that in **four minutes**. **This is why
  rung 1 includes a soak test**, and it is the single biggest threat to
  the plan.

  **But the reframing matters, and it favours us.** Every documented iOS
  WebGPU failure in the record is a *resource-management bug in a
  general-purpose runtime* — ORT's buffer pooling, JSEP's Wasm glue
  looping in WebKit's `HashTable::rehash()`, transformers.js failing to
  release GPU memory between calls — **not a hardware or WebKit compute
  limitation**. The #26827 pathology explicitly spares the plain WASM
  backend, which localises it to the JSEP layer. A runtime with
  explicitly owned, pre-allocated, reused buffers plausibly sidesteps the
  entire class. That is not a guarantee, and the soak test still comes
  first — but it means **L0's static-allocation discipline is a
  correctness feature, not merely a performance one**: allocate every
  buffer at startup, reuse them forever, create nothing per-inference.
- **Silent numerical wrongness.** All of §8 exists for this. No rung
  advances on "it produced text".
- **Battery and thermal.** Continuous GPU work on a phone is a real cost;
  an always-listening feature that flattens the battery is not a feature.
  Measure from rung 10, and let VAD do its job.
- **Privacy.** Always-listening changes miso's character. Nothing leaves
  the device (that is the point of on-device inference), but the *feeling*
  of a listening phone is a design problem, not just an engineering one.
- **Two unresolved WebKit process-crash reports** and one unexplained
  device-lost in Safari 26 WebGPU. Wire `device.lost` handling from rung 2
  and keep a fallback path.
- **The climb stalling half-built.** Structurally mitigated: ort stays
  reachable until rung 13.
- **Scope creep into a general runtime.** §1's non-goals are load-bearing.

---

## 11. Open decisions for ash

1. **Placement** (§4): reusable math under `/compute`, the model under
   `/dictate` — against `compute.md`'s letter. Ratify or redirect.
2. **Foreground-only, or a native shell?** (§6) Ambient listening with the
   screen locked is impossible in a pure PWA. Deciding late is expensive.
3. **Does "permissive" mean MIT/Apache literally, or commercially
   usable?** Doesn't change the #1 pick (Moonshine is MIT) but decides the
   fallback: CC-BY-4.0 reopens NVIDIA FastConformer streaming.
4. **v2 tiny or small to start?** Tiny already beats whisper-tiny.en and
   is 88MB; small is 282MB fp16 for 7.84% WER. Given retrieval tolerates
   error (§6), tiny looks right — but you know the acoustic conditions
   (doorsteps, street noise) better than the benchmark does.
5. **Where the sovereign engine sits in the graded ladder** — an
   implementation choice at the same quality tier needs the scheduler to
   prefer by availability, not just grade.
6. **This file's status** — living plan (current assumption), or a
   one-page charter with the rungs in `notes.md`?

---

*Companion reading: `compute.md` (the driver and its named
WGSL-composition future), `semantic-find.md` (the pattern this whole
document generalises), `notes.md`'s T1–T3 map, and `/phone`'s hard-won
ort facts — kept as a record of what we are leaving behind. Prior art
worth reading before rung 3: [Ratchet](https://github.com/huggingface/ratchet)
(Rust, WebGPU-only, hand-written WGSL, MIT — the closest cousin, and its
[V1 RFC](https://github.com/huggingface/ratchet/discussions/187) is an
honest post-mortem) and
[parakeet.wgsl](https://github.com/narcotic-sh/parakeet.wgsl) (MIT,
hand-written WGSL, 1h of audio in 8.4s on Safari 26).*

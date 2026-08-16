# compute
*miso computes for itself: a WebGPU substrate of our own — WGSL in, numbers out, no dependencies*

> (transcripts/2026-08-15-fm-spec.md#p12)
> I very much like the idea of being able to implement anything we want (even potion-style search) into webgpu, without depending on anything else.

## user

Nothing to see yet — this is miso growing the organ it will think with.
When tools start answering instantly on your phone with the network
off, this is what's underneath.

## spec

Miso grows its own compute substrate. Not a framework adopted, not a
runtime ridden: WebGPU is a JS API, so the whole engine is a thin
page-JS driver and WGSL kernels owned by the features that need them.
No wasm-bindgen, no ort, no burn — `client.wasm`'s zero-import law is
never approached, because the engine never enters it.

The driver does exactly four things: **acquire** the device once,
lazily, requesting precisely the limits the adapter offers (the
clamp-to-what's-granted recipe that iOS Safari demands, baked in from
birth rather than patched in later — a lost device is re-acquirable);
**compile** a WGSL kernel into a cached pipeline; **run** it — float
arrays in, storage buffers bound in order, one output buffer, dispatch,
staging readback, float array out; and **admit absence** — no WebGPU
means `available()` is false and every tenant degrades to its own CPU
path, the standing absence-is-the-unticked-state discipline applied to
hardware.

Tenants arrive as subfeatures, each carrying its own kernels: the
potion-style catalog search first (ceremonially small — the tap counter
of kernels), mel and matmul tiles for speech later, attention blocks
beyond that. The substrate itself ships with one proof kernel
(elementwise multiply) exercised by its tests, so "the GPU answered
with the right numbers" is a checkable fact on any device, not a hope.

Named future mechanism (#p12a — "can we apply our feature-modular
approach to webgpu code I wonder"): WGSL is a C-shaped language with
named functions, the exact surface the linker's chain trick already
composes for Rust — so `.wgsl` files can become a composition target:
redefinition + `existing.fn()` inside kernels, fragment slots between
pipeline stages, provenance comments in generated shader source, and
the toggle test diffing both the composed WGSL and its outputs. To be
built when the first multi-feature kernel arrives (the speech
pipeline), not speculatively.

## glossary

- **compute substrate**: miso's own WebGPU driver + the WGSL kernels of
  its feature nodes — the machinery for running math on the device's
  GPU without depending on anyone else's runtime.
- **tenant**: a feature whose kernels run on the substrate, always with
  a CPU degrade for hardware that offers no WebGPU.

## code description

`compute.js` owns `feature_Compute`.

`init()` acquires the device once and memoizes: no `navigator.gpu`, no
adapter, or a rejected request memoize `null` (absence); the request
asks for the adapter's own values for the buffer and workgroup limits
we may someday reach — never a fixed maximum, which is the iOS Safari
door-slam. A `device.lost` handler resets the memo so the next call
re-acquires. `available()` reports the memo without triggering
acquisition.

`run(wgsl, inputs, outWords)` executes one kernel: the WGSL is compiled
and cached by its source text (`pipelines`); each input Float32Array
becomes a storage buffer at `@binding(i)`, the output buffer binds
last; the entry point is `main`; dispatch is `ceil(outWords / 64)`
workgroups (kernels use `@workgroup_size(64)` by convention, guarding
their own bounds); readback maps a staging copy and returns a
Float32Array. Any failure returns `null` — callers fall to CPU.

The proof kernel (elementwise multiply) lives in the tests, not the
composition.

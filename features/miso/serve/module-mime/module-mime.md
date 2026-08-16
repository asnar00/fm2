# module-mime
*`.mjs` is JavaScript: say so, or browsers refuse to run it*

> (transcripts/2026-08-16-fm-spec.md#p14)
> so now the transcript for the first message disappeared, replaced with a webgpu: no backend available.

## user

Nothing to operate — a delivery fault, not a feature. Parts of the speech
engine ship as `.mjs` files, and the server was labelling them as anonymous
binary data, which browsers are required to refuse. They are labelled
honestly now, so the engine can load every piece it needs.

## spec

`/serve`'s `content_type` knew `.js` but not `.mjs`, so every `.mjs` went
out as `application/octet-stream`. A browser **must** refuse to execute an
ES module served with a non-JavaScript MIME type — this is required
behaviour, not a quirk — so `import()` of those files failed everywhere,
silently, from the day they were first served.

The engine's WebGPU path is the casualty: onnxruntime ships its
WebGPU-capable glue as `.mjs` beside the `.wasm` (which was always
labelled correctly). With the glue unloadable, the runtime falls back to
a module that has no WebGPU entry point at all, and reports the
misleading `webgpuInit is not a function` — the error the field saw once
`/tamed-request` retired a device's wasm pin and let it try WebGPU again.
The pin had been hiding the fault: pinned devices never took the path
that needed the file.

The fix is one line of the chain: `.mjs` answers `text/javascript`, the
same as `.js`. This does not by itself prove WebGPU works on any given
device — it removes a delivery fault standing in front of the question,
and `/engine-receipts` will now report what the device really says.

## glossary

(no new terms)

## code description

`module-mime.rs` extends `/serve`'s `content_type` chain: `.mjs` returns
`text/javascript`, everything else delegates to `existing`. The chain
takes a path, not loop state, so it carries no runtime tick gate — a
server answering its own routes is not a per-user choice.

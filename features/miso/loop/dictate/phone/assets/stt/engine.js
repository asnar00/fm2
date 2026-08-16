// the on-device transcription engine (an ES module, dynamically imported by
// phone.js on first use). Everything loads from muon's own origin:
// transformers.js + ort wasm beside this file, the whisper model under
// /stt/models/ (fetched by tools/fetch_stt.py, pinned versions — see
// features/muon/loop/dictate/phone).
//
// Main-thread by design for now: ort's wasm backend silently refuses to
// initialize inside a module worker, but runs fine on the main thread —
// webgpu compute is async anyway; the wasm fallback blocks for a few
// seconds per note, a noted gap.
//
// Components are assembled by hand (tokenizer + processor + model):
// v4's pipeline() soft-fails its processor load and explodes at call time.
//
// The fallback problem this file actually solves (field-found, #p62): iOS
// can hand out a webgpu adapter whose ort backend then fails — and v4
// MEMOIZES the failure, so retrying another device in the same module is
// futile. The escape: re-import transformers with a cache-busting query (a
// fresh module is a fresh memo), rebuild on wasm, and pin the working
// device in localStorage so later sessions skip the broken path entirely.

const MODEL = 'whisper-tiny.en';
let engine = null;

function configure(env) {
  env.allowRemoteModels = false;
  env.allowLocalModels = true;
  // deliberately scheme-less: transformers v4 only runs its local-file
  // existence probe on non-http(s) paths (a full URL silently loses the
  // tokenizer). Root-relative is right: muon serves site/ at the origin root.
  env.localModelPath = '/stt/models/';
  // numThreads 1: threaded ort wants SharedArrayBuffer, which wants
  // cross-origin-isolation headers muon doesn't send.
  env.backends.onnx.wasm = {
    wasmPaths: new URL('./', import.meta.url).href,
    numThreads: 1,
    proxy: false,
  };
}

// trust nothing but a full adapter + device grant; a pinned choice from a
// previous session's fallback wins outright
async function pickDevice() {
  try {
    if (localStorage.muonSttDevice) return localStorage.muonSttDevice;
  } catch (e) { /* storage may be walled off; probe instead */ }
  try {
    if (navigator.gpu) {
      const adapter = await navigator.gpu.requestAdapter();
      if (adapter && await adapter.requestDevice()) return 'webgpu';
    }
  } catch (e) { /* no working webgpu here */ }
  return 'wasm';
}

async function build(device, bust) {
  const url = './transformers.min.js' + (bust ? '?fresh=' + bust : '');
  const T = await import(url);
  configure(T.env);
  const model = await T.WhisperForConditionalGeneration.from_pretrained(
    MODEL, { dtype: 'q4', device });
  const tokenizer = await T.AutoTokenizer.from_pretrained(MODEL);
  const processor = await T.AutoProcessor.from_pretrained(MODEL);
  return { model, tokenizer, processor, device };
}

async function run(e, audio) {
  const inputs = await e.processor(audio);
  const out = await e.model.generate({ ...inputs, max_new_tokens: 448 });
  const text = e.tokenizer
    .batch_decode(out, { skip_special_tokens: true })[0] || '';
  return text.trim();
}

// seam: an optional taming module may prepare the ground before the device
// probe — clamping the GPU request, retiring a stale pin (see /tamed-request).
// A missing module (that node unticked) is the standing absence-is-off state.
async function tame() {
  try {
    (await import('./tame.js')).prepare();
  } catch (e) { /* absent or declined: the engine behaves as it always did */ }
}

// 16kHz mono float PCM in, text out; throws with a device-tagged message.
// `force` (tests only) overrides the device probe for the first build.
export async function transcribe(audio, force) {
  await tame();
  const device = force || await pickDevice();
  try {
    if (!engine) engine = await build(device, '');
    return await run(engine, audio);
  } catch (first) {
    if (device !== 'webgpu') {
      throw new Error(device + ': ' + String(first && first.message || first).slice(0, 180));
    }
    // webgpu lied (load- or run-time): fresh module, wasm, pin, one retry
    engine = null;
    try {
      engine = await build('wasm', 'wasm-fallback');
      const text = await run(engine, audio);
      try { localStorage.muonSttDevice = 'wasm'; } catch (e) { /* unpinned is fine */ }
      return text;
    } catch (second) {
      engine = null;
      throw new Error(
        'webgpu: ' + String(first && first.message || first).slice(0, 120)
        + ' | wasm: ' + String(second && second.message || second).slice(0, 120));
    }
  }
}

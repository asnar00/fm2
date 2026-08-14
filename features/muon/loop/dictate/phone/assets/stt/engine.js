// the on-device transcription engine (an ES module, dynamically imported by
// phone.js on first use). Everything loads from muon's own origin:
// transformers.js + ort wasm beside this file, the whisper model under
// /stt/models/ (fetched by tools/fetch_stt.py, pinned versions — see
// features/muon/loop/dictate/phone).
//
// Main-thread by design for now: ort's wasm backend silently refuses to
// initialize inside a module worker (empty error, no artifact fetches), but
// runs fine on the main thread — webgpu compute is async anyway; the wasm
// fallback blocks for a few seconds per note, a noted v0 gap.
//
// Components are assembled by hand (tokenizer + processor + model) rather
// than via pipeline(): v4's pipeline factory soft-fails its processor load
// and explodes only at call time.
import { AutoTokenizer, AutoProcessor, WhisperForConditionalGeneration, env }
  from './transformers.min.js';

env.allowRemoteModels = false;
env.allowLocalModels = true;
// deliberately scheme-less: transformers v4 only runs its local-file
// existence probe on non-http(s) paths (a full URL here silently loses the
// tokenizer). Root-relative is right: muon serves site/ at the origin root.
env.localModelPath = '/stt/models/';
// numThreads 1: threaded ort wants SharedArrayBuffer, which wants
// cross-origin-isolation headers muon doesn't send.
env.backends.onnx.wasm = {
  wasmPaths: new URL('./', import.meta.url).href,
  numThreads: 1,
  proxy: false,
};

const MODEL = 'whisper-tiny.en';
let engine = null;

// v4 memoizes model loads by id — a failed first attempt is CACHED, so a
// try-webgpu-catch-wasm ladder can never work. Probe the adapter first and
// make exactly one attempt with the device this runtime actually has.
async function pickDevice() {
  try {
    if (navigator.gpu && await navigator.gpu.requestAdapter()) return 'webgpu';
  } catch (e) { /* no webgpu here */ }
  return 'wasm';
}

async function load() {
  const device = await pickDevice();
  try {
    const model = await WhisperForConditionalGeneration.from_pretrained(
      MODEL, { dtype: 'q4', device });
    const tokenizer = await AutoTokenizer.from_pretrained(MODEL);
    const processor = await AutoProcessor.from_pretrained(MODEL);
    return { model, tokenizer, processor };
  } catch (e) {
    throw new Error(device + ': ' + String(e && e.message || e).slice(0, 180));
  }
}

// 16kHz mono float PCM in, text out; throws with a diagnosable message
export async function transcribe(audio) {
  if (!engine) engine = await load();
  const inputs = await engine.processor(audio);
  const out = await engine.model.generate({ ...inputs, max_new_tokens: 448 });
  const text = engine.tokenizer
    .batch_decode(out, { skip_special_tokens: true })[0] || '';
  return text.trim();
}

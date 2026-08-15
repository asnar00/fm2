const feature_SemanticFind = {
  ready: false, loading: null,
  meta: null, vocab: null, table: null, scales: null,
  paths: null, cat: null, // catalog: node paths + flat Float32Array [M x dims]

  load() {
    if (this.loading) return this.loading;
    this.loading = (async () => {
      const [meta, vocab, tableBuf, scalesBuf, vectors] = await Promise.all([
        fetch('find/meta.json').then((r) => r.json()),
        fetch('find/vocab.json').then((r) => r.json()),
        fetch('find/table.bin').then((r) => r.arrayBuffer()),
        fetch('find/scales.bin').then((r) => r.arrayBuffer()),
        fetch('features/vectors.json', { cache: 'no-store' }).then((r) => r.json()),
      ]);
      this.meta = meta;
      this.vocab = new Map(vocab.map((t, i) => [t, i]));
      this.table = new Int8Array(tableBuf);
      this.scales = new Float32Array(scalesBuf);
      this.paths = Object.keys(vectors.vecs);
      this.cat = new Float32Array(this.paths.length * meta.dims);
      this.paths.forEach((p, m) => this.cat.set(vectors.vecs[p], m * meta.dims));
      this.ready = true;
    })().catch(() => { this.loading = null; }); // failed load: retry next ask
    return this.loading;
  },

  // BERT basic tokenisation — the exact mirror of tools/potion_embed.py
  basic(text) {
    const out = [];
    let word = '';
    const push = () => { if (word) { out.push(word); word = ''; } };
    for (const ch of text.toLowerCase().normalize('NFD')) {
      if (/\p{Mn}|\p{Cc}|\p{Cf}/u.test(ch)) continue;
      if (/\s/.test(ch)) push();
      else if (/\p{P}/u.test(ch) || (/\p{S}/u.test(ch) && !/[a-z0-9]/.test(ch))) {
        push(); out.push(ch);
      } else word += ch;
    }
    push();
    return out;
  },

  tokenize(text) {
    const ids = [];
    for (const word of this.basic(text)) {
      const pieces = [];
      let start = 0, dead = false;
      while (start < word.length) {
        let end = word.length, id;
        for (; start < end; end--) {
          const sub = (start > 0 ? this.meta.prefix : '') + word.slice(start, end);
          if (this.vocab.has(sub)) { id = this.vocab.get(sub); break; }
        }
        if (id === undefined) { dead = true; break; }
        pieces.push(id);
        start = end;
      }
      if (dead) {
        const u = this.vocab.get(this.meta.unk);
        if (u !== undefined) ids.push(u);
      } else ids.push(...pieces);
    }
    return ids;
  },

  embed(text) {
    const dims = this.meta.dims;
    const ids = this.tokenize(text);
    const v = new Float32Array(dims);
    if (!ids.length) return v;
    for (const i of ids) {
      const s = this.scales[i], base = i * dims;
      for (let d = 0; d < dims; d++) v[d] += this.table[base + d] * s;
    }
    let norm = 0;
    for (let d = 0; d < dims; d++) { v[d] /= ids.length; norm += v[d] * v[d]; }
    norm = Math.sqrt(norm) || 1;
    for (let d = 0; d < dims; d++) v[d] /= norm;
    return v;
  },

  kernel:
    '@group(0) @binding(0) var<storage, read> cat: array<f32>;\n'
    + '@group(0) @binding(1) var<storage, read> q: array<f32>;\n'
    + '@group(0) @binding(2) var<storage, read_write> res: array<f32>;\n'
    + '@compute @workgroup_size(64)\n'
    + 'fn main(@builtin(global_invocation_id) gid: vec3<u32>) {\n'
    + '  let m = gid.x;\n'
    + '  if (m >= arrayLength(&res)) { return; }\n'
    + '  var s = 0.0;\n'
    + '  let dims = arrayLength(&q);\n'
    + '  for (var d = 0u; d < dims; d = d + 1u) { s = s + cat[m * dims + d] * q[d]; }\n'
    + '  res[m] = s;\n'
    + '}\n',

  // cosine of the query against every catalog entry (all vectors are unit):
  // the substrate when it answers, the same loop on CPU when it doesn't
  async score(q) {
    const M = this.paths.length;
    if (typeof feature_Compute !== 'undefined') {
      const gpu = await feature_Compute.run(this.kernel, [this.cat, q], M);
      if (gpu) return gpu;
    }
    const dims = this.meta.dims;
    const res = new Float32Array(M);
    for (let m = 0; m < M; m++) {
      let s = 0;
      for (let d = 0; d < dims; d++) s += this.cat[m * dims + d] * q[d];
      res[m] = s;
    }
    return res;
  },
};
{
  if (typeof feature_Ask !== 'undefined' && typeof feature_Chooser !== 'undefined') {
    const fm_semanticFeatures = feature_Ask.features.bind(feature_Ask);
    feature_Ask.features = async function (words) {
      const sf = feature_SemanticFind;
      if (!sf.ready) { sf.load(); return fm_semanticFeatures(words); }
      try {
        await feature_Chooser.load();
        const scores = await sf.score(sf.embed(words.join(' ')));
        const hits = [];
        for (let m = 0; m < sf.paths.length; m++) {
          if (scores[m] >= 0.3) hits.push({ m, s: scores[m] });
        }
        hits.sort((a, b) => b.s - a.s);
        const nodes = hits.slice(0, 3)
          .map((h) => feature_Chooser.byPath[sf.paths[h.m]])
          .filter(Boolean);
        return nodes.length ? nodes : fm_semanticFeatures(words);
      } catch (e) {
        return fm_semanticFeatures(words);
      }
    };
  }
}

# semantic-find
*the substrate's first tenant: asks find tools by meaning, instantly, on the device*

> (transcripts/2026-08-15-fm-spec.md#p13)
> sure-footed as a mountain goat - this is awesome to watch :-)
> *(the climb continued: #p9 "if the tool exists, we'll find it quickly on the phone without touching the network"; #p12 "implement anything we want (even potion-style search) into webgpu, without depending on anything else")*

## spec

`/ask`'s find step matches words; this node matches **meaning**, with
machinery muon owns end to end. The model is potion-base-8M distilled
to its essence: a 29528×256 token→vector table (int8, ~7.5MB, pinned
and converted by `tools/fetch_find.py` — the stt fetching pattern), a
WordPiece walk, a mean, a normalise. No inference framework exists at
runtime because none is needed.

The work splits by where the data lives. **Deploy embeds the catalog**:
`tools/embed_catalog.py` runs after the tree export, embedding each
node's name+purpose+intro with the Python twin of the device's embedder
(`tools/potion_embed.py`) — both read the same table and walk the same
WordPiece, so the two sides agree by construction, not by hope — and
ships `features/vectors.json`. **The device embeds only the query**,
then scores it against the catalog by cosine — on `/compute`'s
substrate with a WGSL dot-product kernel, falling to the same loop on
CPU when the GPU is absent. At catalog scale the GPU is ceremonial;
that is the point — the substrate's first tenant is small enough to
verify to the last decimal.

The assets load lazily on the first ask (once, ~8MB, cached by the
service worker and preserved by `/delta` ever after); until they
arrive, and whenever anything fails, the word-overlap scorer keeps
answering — absence degrades, never breaks.

## user

Ask in your own words and the right tools surface even when you use
none of their names. It answers in a blink, works in airplane mode,
and nothing you type ever leaves your phone.

## glossary

- **catalog vectors**: the feature tree embedded at deploy time —
  meaning, precomputed where the tree lives.
- **query vector**: your ask embedded on the device — meaning, computed
  where the words live.

## code description

`semantic-find.index.js` owns `feature_SemanticFind`.

`load()` fetches the four artifacts once (`find/vocab.json`,
`find/table.bin`, `find/scales.bin`, `features/vectors.json`),
memoising in flight; `ready` flips when all arrive.

The embedder mirrors `tools/potion_embed.py` exactly: BERT basic
tokenisation (lowercase, accents stripped, punctuation split), greedy
longest-match WordPiece with `##` continuations, unknown words to
`[UNK]`; `embed()` means the matching int8 rows (dequantised by the
per-row scale), then L2-normalises.

`score(q)` computes the catalog dot-products — `/compute`'s `run()`
with a `@workgroup_size(64)` WGSL kernel, one thread per catalog entry
(typeof-guarded), and an identical CPU loop when the substrate is
absent or declines.

The wrap on `feature_Ask.features` rebuilds the query from the words
`/ask` passes, embeds, scores, and returns the top three entries above
0.3 cosine; assets not yet ready (the load is kicked, not awaited) or
zero hits above threshold fall through to the original word-overlap
scorer.

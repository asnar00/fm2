# fmdoc
*fm.md heads the tree — visitors orient on the founding document*

> (transcripts/2026-08-13-fm-spec.md#p80)
> supercool. One final thing: can we add one item to the top of the tree: the original fm.md document itself. So if I give someone that URL, they come to the fm.md document first, by way of orientation.

## user

Share https://miso.nøøb.org/features/ and people land on fm.md; the tree is one tap away throughout.

## spec

`/features/` itself renders fm.md — the user-authored source of truth — with the tree alongside; every page carries the `fm.md` entry at the top of the tree. A stranger meets the intent first, in the author's own voice, and the tree beneath is that intent realised.

## glossary

(no new terms)

## code description

`doc_link` in explorer.py, prepended to the tree by both page renderers (local: links `/view/fm.md`; static export rewrites it to `/features/`); export_features.py renders the root `index.html` via `render_file_page("fm.md")`.

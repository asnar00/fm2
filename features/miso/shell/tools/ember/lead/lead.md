# lead
*projects, posts, people lead the toolbar*

> (asks#1787704573976)
> main toolbar should be projects, then posts, then users
> *(filed from the field on 2026-08-26 by ash)*

## user

The toolbar starts with projects, then posts, then people (👤); the rest follow.

## spec

Tools registered in the order their nodes were written, so the launcher read taps, dictate, 👤, posts, projects — the demos first. Ash asked for projects, posts, users (`asks#1787704573976`). One reading, so it builds: this node extends `tools_list` and orders the registered tools with those three ids first, in that order, then everything else as it came. A person's own drag order (`/reorder`, in build) is a newer link and so sits outside this one: the default yields to it. Untick and registration order returns.

## glossary

(no new terms)

## code description

`lead.rs` — `tools_list` calls `existing` and re-sorts: the lead ids, then the rest.

# counter
*the tap counter becomes the "taps" tool on the launcher*

> (transcripts/2026-08-14-fm-spec-2.md#p41)
> we'd have a tool called "transcribe" which is a button on the main muon screen (a bit like the iphone's app launcher)

## spec

With `/tools` owning the main screen, the tap counter becomes miso's first
registered tool. This node exists because of provenance ordering: `/tap`
linearises at its birth time, before the `tools_list` chain exists, so an
older feature registers on a newer chain through a new subfeature — causality
made visible in the tree. (`/tap` itself gained a launcher-aware gate: it
renders only as the open tool when the launcher is present, and renders as
always when it isn't.)

## user

The launcher shows a **taps** button; the shared counter lives inside it.

## glossary

(no new terms)

## code description

`counter.rs` redefines `tools_list`, appending `{id: "taps", label: "taps"}`
to whatever earlier features registered.

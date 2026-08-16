# context-manager
*your ticks come true: unticking a feature actually switches it off, per user, live*

> (transcripts/2026-08-16-fm-spec.md#p4)
> OK, shall we review the notes / ideas (bullet point form) and decide what feels like fun to try and achieve today?
> *(the context manager was the chosen rung; the lockout question — should the
> steering surface be exempt from enforcement? — was ruled "nothing exempt")*

> (transcripts/2026-08-16-fm-spec.md#p4a, the ruling's rider)
> (I think we'll come back to that later when we enact per-user privileges)

## spec

The enforcement the chooser has been waiting for (`/chooser`: ticks are
"stored, not yet enforced, awaiting the context manager"). The mechanism is
the linker's own composition made runtime-aware: every chain link is already
a static call to the previous definition, so **unticking is falling
through**. When this node is in the composition, the linker emits a gate at
the head of every chain-extending function that takes loop state: if the
owning node's path — or any ancestor of it — is explicitly `false` in the
user's `feature_ticks` map, the link returns the previous definition's
answer instead of its own. The same toggle at two speeds: compose-time
unticking removes the code; a runtime untick skips it, per user, on the
next event.

What falls out by construction: **ancestor patterns survive** (the gate
walks path prefixes, so unticking `/dictate` silences `/mirror` and
`/phone` with it); the Elm-style page halves idle out (the state edges
they watch never fire); tool registrations vanish from the toolbar
(`tools_list` is a gated chain); a rung's reachability slot falls back to
"unreachable". Chain-*starting* definitions stay ungated — they are the
seams themselves — and so do functions that don't carry loop state
(server routes and message handling gate nothing; enforcement is what the
user's own instances do). Absent stays on: a user with no explicit ticks
pays one substring scan per gated link and nothing more.

The ruling of record (#p4): **nothing is exempt** — the chooser and panel
gate like everything else, so a user who unticks the chooser has removed
their own re-tick surface (repair is a server-side var edit). The rider
(#p4a): this is deliberate — the exemption question returns as a privilege
question when per-user privileges land (notes.md #p12), where "who may
untick what" gets its real answer.

One subtree stands outside the ruling, by necessity rather than policy:
the **trusted base** (`trusted.md`, read by the linker — currently
`/scope`). The context manager reads the ticks var; `/scope` is what
delivers it. The first build gated scope like everything else, and the
field proof arrived within minutes: unticking `miso/loop` froze a stale
map in place — the explicit `false` gated the very `VarUpdate` that would
have cleared it, and the tick became irreversible, breaking this spec's
own promise. Gating the transport of the ticks is gating the context
manager's senses; a node the enforcer needs in order to enforce is part
of the enforcer. Unticking the enforcer itself already has a defined
meaning — no enforcement — and the trusted base is that same fail-open
principle applied to its nervous system.

Unticking THIS node removes the hook from the composition, and with it
every gate the linker would have emitted: ticks return to
stored-not-enforced, exactly the standing behaviour.

## user

The tickboxes in your feature list now mean it: untick a feature and it
switches off for you — its buttons leave your toolbar, its behaviour
stops, on all your devices, within moments and without an update. Tick it
back and it returns just as fast. Nobody else's app changes: your ticks
are yours. Unticking something that contains other things switches off
the whole family. Careful with the feature list itself — untick that and
you've hidden the very switch that brings things back.

## glossary

- **context manager**: the runtime half of composition — the mechanism
  that consults your per-user choices while the app runs, where the
  linker consults the product's choices while it builds.
- **gate**: the linker-emitted check at the head of a chain link: unticked
  means answer with the previous definition instead.

## code description

`context-manager.lib.rs` (verbatim library) owns the hook the gates call:
`fm_unticked(state, path)` — a fast substring scan finds the
`feature_ticks` var's raw string in the state JSON without parsing it;
a thread-local cache keyed on that raw slice re-derives the explicit-false
path list only when the map actually changes; the verdict is a prefix walk
of that list. No ticks in state (or an unparseable map) means nothing is
off.

The gate emission lives in the linker (`tools/fmlink.py`, scaffolding per
the standing arrangement): `compose_features` detects the hook's presence
in any composed verbatim lib, and for each chain-extending fn whose first
parameter is `state: String` injects the gate line after the signature —
`if fm_unticked(&state, "<node path>") { return feature_<Prev>::<fn>(<args>); }`
— the same fall-through target `existing` resolves to. Node paths are
shared-tree paths (a product's materialised override dirs normalise to
them). Subtrees listed in the hook node's `trusted.md` are skipped. No
hook in the composition, no gates: the emitted source is byte-identical
to today's.

PAID-FOR LESSONS, recorded so nobody re-pays: (1) the raw-scan must match
the ticks KEY, not any occurrence — the same text rides inside queued
`VarSet` messages as a value (`"key":"feature_ticks"`), where the scanner
would read `"user"` as the map and fail open; the discriminator is the
`':'` that follows a real key. (2) The trusted base exists because its
absence was observed to freeze the map (see spec). (3) An update-chain
gate reads the *incoming* state, so the event that flips a tick is itself
processed under the old context; the render that follows sees the new one
— the correct Elm-style boundary.

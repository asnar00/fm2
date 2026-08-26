# own-slot
*each world's broadcast slot lives with the world*

> (transcripts/2026-08-25-accounts.md#p112)
> it would be really nice to deploy whatever's complete now.
> *(the reason the deploys were not completing: the smoke gate's server and a rig server shared one broadcast file and talked over each other — fixed under the residuals rule, #p50)*

## user

Nothing visible. Two miso servers on one machine — the deploy gate's, a worker's rig, yours — no longer hear each other's messages.

## spec

`/messaging` kept its versioned broadcast slot at a fixed `/tmp/miso-broadcast.json`, so every server on a machine published into one file and every client of any of them heard all of it. The handover's two processes need to share a slot; strangers must not — and on 2026-08-26 the deploy gate failed three times because a rig on another port was talking into its stream. This node redefines `broadcast_file()` to `<context_dir>/broadcast.json`: each state directory has its own, a handover's two processes still share theirs. On first use in the real state directory the old `/tmp` slot is copied over once, so clients holding a version number from it keep their place; a rig (`MISO_CONTEXT_DIR` set) starts empty — seeding it from the machine's slot handed the deploy gate a stale build announcement and reloaded its page mid-test. Untick and the fixed path returns.

## glossary

(no new terms)

## code description

`own-slot.rs` — `broadcast_file` redefined; seeds the new file from `existing.broadcast_file()` when absent.

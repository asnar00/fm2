# same-door
*the tooling POST mints the same op a client does*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.

## user

For agents and operators. `POST /diag/context` takes the body it always took:

```
curl -X POST localhost:8095/diag/context \
  -d '{"path":"miso/shell/panel/noob-button/chooser","name":"enabled","value":true}'
```

Three things are new. The edit now reaches the person's other devices, so a
repair made through the tunnel takes effect without a reload. It is recorded in
the log the way a client's edit is, with an id, so a replay after a restart
produces the same world. And a var whose merge is `counter` refuses a bare
value and tells you what to send instead — `{"op":"add","value":[epoch,delta]}`
to count, `{"op":"set","value":[epoch+1,n]}` to reset. `{"op":"clear"}` gives a
var back to whatever it inherits, and `"at":"global"` addresses the shared layer
exactly as `/msg` does, refused by the same privilege check.

## spec

Rung 3 wrote this route before ops existed. It assigned straight into the
world through `set_from_json`, which meant four disagreements with every other
edit in the system: no merge discipline (an absolute value landed on a counter,
losing the epoch that makes concurrent adds coherent), no op id, a log record
minted by `remember`'s separate seam on this route rather than by the op path,
and nothing said to the caller's other instances. A repair typed at the server
was a second door into the world, and second doors are where invariants go to
die.

This node makes the route a **translation**: it builds the `CtxOp` message a
client would have sent and hands it to `handle_msg`. Everything downstream then
happens by itself — `apply_op` enforces the declared merge, `overlay` checks the
seen-set and the layer privilege, `remember` logs exactly what was applied, and
`converge` relays the resolved value to the sender's other instances. The route
keeps its URL, its body, and its `{"ok":true}` answer (now with the resolved
`value` beside it), so every rig, doc and repair path that uses the plain
three-key body keeps working — the overwhelming case, and proven both with the
node on and with it off.

**The counter refusal is written here rather than inherited.** A bare number
sent to a counter fails deep inside the merge with a serde message about arrays
of length two, which tells the caller nothing. The refusal is raised before the
op is minted, in the caller's vocabulary, and it names both verbs and the
current value. The rule it enforces is 7b's: an absolute assignment is a RESET,
and a reset carries the epoch that makes every add minted before it stale —
which is precisely what the log needs to replay it.

**Identity comes from the request, not the payload.** The op is stamped with
`sender_of`'s cookie-proven identity, the same one `/messaging` stamps, so the
relay reaches that person and nobody else. Localhost tooling editing a `local:`
world has no audience and gets none — the edit still applies and still logs.

**Ids are minted per call** — the clock plus a process counter, prefixed
`tool-`, so a human reading the op log can see which records came through this
door.

**The old door is what unticking restores.** This link deliberately does not
call `existing.context_set`: the chain beneath is rung 3's assignment plus
`remember`'s log seam on it, and calling it would log the same edit twice.
Untick this node and that pair answers again, with no id and no relay.

## glossary

- **the tooling door**: `POST /diag/context`, the agent's and operator's write
  path into a world — localhost open, tunnel cookie-gated.
- **reset**: a counter's absolute assignment, carrying the epoch that
  invalidates adds minted before it.

## code description

`same-door.rs` redefines one function and adds five.

`context_set` (line 13) is the route's new body: it screens as before, reads
`path`, `name`, an optional `op` (default `set`) and `value` (`clear` needs
none), raises the counter refusal, and hands off.

`context_op_post` (line 50) mints the message — `type: CtxOp`, a `tool-` id,
`_from` from the cookie, `at` passed through when given — calls `handle_msg`,
and translates the answer back into this route's shape: a `CtxUpdate` becomes
`{"ok":true,"value":…}`, an absorbed repeat stays `{"ok":true}`, anything else
becomes a 400 carrying the handler's own words.

`counter_refusal` (line 84) answers `Some(message)` for a bare set on a
`counter` var and `None` for everything else, so the check costs one snapshot
read on the tooling path and nothing anywhere else.

`context_merge_of`, `context_value_of` and `context_var_field` (lines 104–141)
read a var's declared attributes out of the generated snapshot walker — the
same trick `enforced` uses for presence, and the reason this node knows nothing
about which vars exist.

`context_tool_op_id` (line 145) is the id: `tool-<ms>-<n>`.

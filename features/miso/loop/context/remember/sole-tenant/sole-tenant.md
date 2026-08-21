# sole-tenant
*one server per state directory, claimed at boot*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.

## user

For operators and agents. A second server started on a state directory another
live server holds refuses to start, and says which pid holds it and what to do:
stop the other one, give this one `MISO_CONTEXT_DIR`, or set
`MISO_ALLOW_SHARED_STATE=1` to mean it anyway. A crash leaves nothing to clean
up — the next boot takes the directory back and logs that the last run did not
shut down cleanly.

Rigs and dev servers are the reason this exists: a second server on another
port, pointed at the live state directory, used to be silent.

## spec

Rung 6a's ruling, arriving where it can be enforced. Two processes on one state
directory each keep worlds in memory and each append to the same per-user op
logs; compaction then rewrites a log from one process's view of a world the
other has also been editing, and the loser's records are gone. Neither server is
wrong on its own, which is what makes it hard to see.

**Boot, not deploy, is where this is honest.** A deploy can only look at one
machine at one moment; the second process appears whenever somebody starts one,
and the state directory is not even necessarily on the machine that deployed.
The server knows its own `MISO_CONTEXT_DIR` at boot and can refuse before it has
read anything out of it. `tools/deploy.sh` asserts the outcome instead — it
warns when more than one `miso_server` is live on the mini — which is a
different and complementary statement, and never fatal to a release.

**The claim is a pidfile with a liveness check.** `<state dir>/server.pid`
carries the pid and the boot time. At boot the holder is read and tested with
`ps -p`, which answers two questions at once: no output means the process is
gone, and a name that is not a miso server means the pid was recycled by
something unrelated. Only a live miso is a reason to refuse — 6a's own argument
about lock files is that a stale one must never wedge the server, and the two
ways a pidfile goes stale are exactly those two.

**What each case does.** A live holder: refuse, exit 1, one message naming the
pid, the directory and the three ways out. A dead holder: take the directory and
log that the last shutdown was not clean — the only trace a crash leaves. No
file: claim it quietly. Unwritable directory: warn and carry on serving, because
a server that cannot write a pidfile is still a working server, and refusing
would turn a permissions problem into an outage.

**The override does not take the claim.** A guest admitted by
`MISO_ALLOW_SHARED_STATE` leaves `server.pid` pointing at the server that
actually holds the directory, so a later boot still finds a live holder rather
than the guest's corpse. Deliberate sharing stays deliberate; it does not
silently disarm the check for the next process.

**The port is not this check.** Two servers on one port already fail at the
bind. What that never caught is the interesting case and the one the rigs live
in: another port, same directory.

## glossary

- **the claim**: `<state dir>/server.pid`, this process's assertion that the
  directory is its own.
- **stale claim**: a pidfile whose pid is dead, or has been recycled by a
  process that is not a miso server.

## code description

`sole-tenant.rs` extends `serve` and adds four functions.

`serve` (line 6) claims the directory before the chain beneath binds the port,
so the refusal never depends on which port this server was going to use.

`claim_state_dir` (line 17) is the decision: read the holder, refuse if it is a
live miso (unless the override is set, in which case it warns and leaves the
claim alone), announce a takeover if the holder is gone, then write the pidfile.
A failed write is a warning, not a refusal.

`state_pid_file` (line 58) and `pid_held` (line 64) are the file: `<dir>/
server.pid`, holding `<pid> <boot ms>`, of which only the first field is parsed.

`pid_is_miso` (line 73) runs `ps -p <pid> -o comm=` — one subprocess, once, at
boot — and treats an empty answer or a foreign name as "nobody is holding this".

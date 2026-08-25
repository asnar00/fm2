struct feature_Handover;
impl feature_Handover {
    // how long a leaving server waits for its in-flight requests before it
    // exits anyway. Long-polls answer within a tick once draining (see
    // msg_wait_ticks), so this is the ceiling on ordinary requests, not on a
    // poll's 25s horizon — a drain is normally over in well under a second.
    fn drain_grace_ms() -> u64 {
        5000
    }

    // how long a successor waits for the incumbent to go. Generous: it is
    // only ever spent when something is wrong, and the incumbent is serving
    // perfectly well the whole time.
    fn handover_grace_ms() -> u64 {
        30000
    }

    // MISO_HANDOVER=1 means "this process is replacing whoever holds the
    // state directory". Unset — dev, rigs, the ordinary case — nothing below
    // changes, and /sole-tenant's refusal still protects everybody.
    fn handover_wanted() -> bool {
        fm_handover_wanted()
    }

    fn claim_state_dir() {
        if handover_wanted() {
            // deferred on purpose. /sole-tenant claims before the port is
            // bound, which is right for every other case and wrong for this
            // one: refusing here would refuse the very thing this process
            // came to do, and evicting here would take the incumbent down
            // before this process has proved it can bind at all. The claim is
            // written in bind_listener, once the listener is up and the
            // incumbent has gone.
            return;
        }
        existing.claim_state_dir()
    }

    // the sequence, in the only order that never leaves the port unheld:
    // bind beside the incumbent, then ask it to leave, then take the claim.
    // A binary that cannot bind dies here without touching a working server.
    fn bind_listener() -> std::net::TcpListener {
        let l = existing.bind_listener();
        fm_handover_hold(&l);
        fm_handover_install(drain_grace_ms());
        if handover_wanted() {
            let held = pid_held(&state_pid_file());
            if held != 0 && held != std::process::id() && pid_is_miso(held) {
                println!("miso: handover — pid {} holds {}, asking it to leave",
                         held, context_dir());
                if !fm_handover_evict(held, handover_grace_ms()) {
                    eprintln!("miso: REFUSING TO START: pid {} did not leave \
                               {} within {}ms. It is still serving, so nothing \
                               is down; stop it by hand and start again.",
                              held, context_dir(), handover_grace_ms());
                    std::process::exit(1);
                }
            }
            let _ = std::fs::create_dir_all(context_dir());
            let _ = std::fs::write(state_pid_file(),
                                   format!("{} {}\n", std::process::id(), now_ms()));
            println!("miso: handover complete — pid {} holds {}",
                     std::process::id(), context_dir());
        }
        l
    }

    // a wait must not outlive the server answering it. Two ticks rather than
    // none so a page cannot spin through the drain: a parked poll returns its
    // ordinary empty answer within 200ms, a fresh one within 400ms, and in
    // both cases the page re-asks at once and lands on the successor.
    fn msg_wait_ticks() -> u32 {
        if fm_draining() {
            2
        } else {
            existing.msg_wait_ticks()
        }
    }

    // two same-host endpoints and the in-flight count the drain waits on.
    // /admin is localhost-only by the same rule /gate trusts: `!r.tunnel`
    // means the request cannot have come from outside this machine, because
    // /loopback binds the port there.
    fn route(r: request) -> response {
        if r.path == "admin/whoami" && !r.tunnel {
            return json_response(200, format!(
                "{{\"pid\":{},\"build\":\"{}\",\"draining\":{}}}",
                std::process::id(), build_stamp(), fm_draining()));
        }
        if r.path == "admin/drain" && r.method == "POST" && !r.tunnel {
            let body = format!("{{\"ok\":true,\"pid\":{}}}", std::process::id());
            fm_drain_begin(drain_grace_ms());
            return json_response(200, body);
        }
        fm_inflight_enter();
        let resp = existing.route(r);
        fm_inflight_leave();
        resp
    }

    // the build this process is serving, read from site/ the same way the
    // shell reads it. Two processes mid-handover share one site/, so this is
    // not what tells them apart — the pid beside it is.
    fn build_stamp() -> String {
        std::fs::read_to_string("site/version")
            .unwrap_or_default()
            .trim()
            .to_string()
    }
}

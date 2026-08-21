struct feature_SoleTenant;
impl feature_SoleTenant {
    // boot is where a second process actually appears, so this is where the
    // state directory is claimed — before anything is read out of it and
    // before the port is bound.
    fn serve() {
        claim_state_dir();
        existing.serve()
    }

    // one server per state directory (rung 6a's ruling). Two processes holding
    // one directory both keep worlds in memory and both append to the same op
    // logs: each would compact away the other's records and neither would be
    // wrong on its own. The port already stops a second server on the same
    // port; what it does not stop is the interesting case — a dev or rig server
    // on ANOTHER port, or a MISO_CONTEXT_DIR pointing at the live one.
    fn claim_state_dir() {
        let dir = context_dir();
        let file = state_pid_file();
        let mine = std::process::id();
        let held = pid_held(&file);
        if held != 0 && held != mine && pid_is_miso(held) {
            if std::env::var("MISO_ALLOW_SHARED_STATE").is_ok() {
                // the guest does NOT take the claim: the server that holds the
                // directory keeps it, so the next boot still finds a live
                // holder rather than this one's corpse.
                eprintln!("miso: WARNING: pid {} already holds {} — continuing \
                           because MISO_ALLOW_SHARED_STATE is set, and leaving \
                           the claim with them. Two servers on one state \
                           directory will lose each other's context ops.",
                          held, dir);
                return;
            } else {
                eprintln!("miso: REFUSING TO START: pid {} is already serving \
                           from the state directory {}. Two servers on one \
                           directory overwrite each other's context logs. Stop \
                           the other one, give this one its own directory with \
                           MISO_CONTEXT_DIR, or set MISO_ALLOW_SHARED_STATE=1 \
                           if you really mean it.", held, dir);
                std::process::exit(1);
            }
        }
        if held != 0 && held != mine {
            // a crash leaves the file behind. It must never wedge the server:
            // the previous run is gone, this one takes the directory and says
            // so, which is the only trace that the last shutdown was not clean.
            eprintln!("miso: the state directory {} was left claimed by pid {}, \
                       which is no longer running — taking it over (the last \
                       run did not shut down cleanly).", dir, held);
        }
        let _ = std::fs::create_dir_all(&dir);
        if let Err(e) = std::fs::write(&file, format!("{} {}\n", mine, now_ms())) {
            eprintln!("miso: WARNING: cannot claim {} ({}) — a second server \
                       would not be noticed.", file, e);
        }
    }

    fn state_pid_file() -> String {
        format!("{}/server.pid", context_dir())
    }

    // the pid, or 0 for "nobody has claimed this". The file carries the boot
    // time beside it for a human reading it; only the first field is parsed.
    fn pid_held(file: &String) -> u32 {
        let raw = std::fs::read_to_string(file).unwrap_or_default();
        raw.trim().split(' ').next().unwrap_or("").parse().unwrap_or(0)
    }

    // is that pid a live miso server? `ps -p` answers both questions at once:
    // no output means the process is gone (a crash), and a name that is not
    // ours means the pid was recycled by something unrelated — neither is a
    // reason to refuse. One subprocess, once, at boot.
    fn pid_is_miso(pid: u32) -> bool {
        let out = std::process::Command::new("ps")
            .args(["-p", &pid.to_string(), "-o", "comm="])
            .output();
        match out {
            Ok(o) => String::from_utf8_lossy(&o.stdout).contains("miso"),
            Err(_) => false,
        }
    }
}

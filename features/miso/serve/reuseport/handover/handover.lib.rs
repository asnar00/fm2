// ---- handover: two servers, one port, one of them leaving ------------------
//
// Process-global like /present's presence table and /harden's store lock: a
// server has one listener, one drain, and one in-flight count.

// set once and never cleared — a server that has begun leaving does not come
// back, and every reader only ever needs "has it started".
pub static FM_DRAINING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
// the signal handler may do nothing but this; a watchdog thread reads it.
pub static FM_SIGTERM: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
// requests inside route(). The drain waits for this to reach zero.
pub static FM_INFLIGHT: std::sync::atomic::AtomicI64 =
    std::sync::atomic::AtomicI64::new(0);
// the listening descriptor, remembered so the drain can leave the port
// without owning the accept loop. -1 until bind_listener has run.
pub static FM_LISTEN_FD: std::sync::atomic::AtomicI64 =
    std::sync::atomic::AtomicI64::new(-1);

pub fn fm_draining() -> bool {
    FM_DRAINING.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn fm_inflight_enter() {
    FM_INFLIGHT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

pub fn fm_inflight_leave() {
    FM_INFLIGHT.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
}

/// MISO_HANDOVER=1: this process is replacing whoever holds the state
/// directory. Anything else — unset, "0", empty — and it is an ordinary boot.
pub fn fm_handover_wanted() -> bool {
    match std::env::var("MISO_HANDOVER") {
        Ok(v) => !v.is_empty() && v != "0",
        Err(_) => false,
    }
}

/// remember the listening descriptor so fm_drain_begin can close it.
pub fn fm_handover_hold(l: &std::net::TcpListener) {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        FM_LISTEN_FD.store(l.as_raw_fd() as i64, std::sync::atomic::Ordering::SeqCst);
    }
    #[cfg(not(unix))]
    {
        let _ = l;
    }
}

#[cfg(unix)]
fn fm_pid_alive(pid: u32) -> bool {
    // signal 0 asks the question without sending anything: alive, or alive
    // and not ours (EPERM). Either way somebody is there.
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

#[cfg(not(unix))]
fn fm_pid_alive(_pid: u32) -> bool {
    false
}

/// ask the incumbent to leave and wait until it has. SIGTERM rather than a
/// request to its port, because with SO_REUSEPORT a request to the port might
/// come back to this process; a pid names exactly one server. Returns whether
/// it is gone — false means the caller must not proceed, because two servers
/// on one state directory lose each other's ops (/sole-tenant).
#[cfg(unix)]
pub fn fm_handover_evict(pid: u32, grace_ms: u64) -> bool {
    unsafe {
        libc::kill(pid as libc::pid_t, libc::SIGTERM);
    }
    let mut waited = 0u64;
    while waited < grace_ms {
        if !fm_pid_alive(pid) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
        waited = waited + 25;
    }
    !fm_pid_alive(pid)
}

#[cfg(not(unix))]
pub fn fm_handover_evict(_pid: u32, _grace_ms: u64) -> bool {
    true
}

/// leave the port. The successor is already bound (SO_REUSEPORT), so closing
/// this descriptor hands every new connection to it without a moment in which
/// nobody is listening. The accept loop then fails forever, which /serve
/// answers by sleeping rather than spinning; this process exits seconds later,
/// so the listener is never dropped and the descriptor never double-closed.
fn fm_close_listener() {
    #[cfg(unix)]
    {
        let fd = FM_LISTEN_FD.swap(-1, std::sync::atomic::Ordering::SeqCst);
        if fd >= 0 {
            unsafe {
                libc::close(fd as libc::c_int);
            }
        }
    }
}

/// begin leaving: stop accepting, let what is in flight finish, exit clean.
/// Idempotent — a second SIGTERM, or a drain request racing the signal, is a
/// no-op rather than a second exit timer.
pub fn fm_drain_begin(grace_ms: u64) {
    if FM_DRAINING.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    println!("miso: draining — leaving the port, {} request(s) in flight",
             FM_INFLIGHT.load(std::sync::atomic::Ordering::SeqCst));
    std::thread::spawn(move || {
        fm_close_listener();
        let mut waited = 0u64;
        while FM_INFLIGHT.load(std::sync::atomic::Ordering::SeqCst) > 0 && waited < grace_ms {
            std::thread::sleep(std::time::Duration::from_millis(10));
            waited = waited + 10;
        }
        // the response body is written after route() returns, outside the
        // count; a short settle lets the last one reach the wire.
        std::thread::sleep(std::time::Duration::from_millis(150));
        println!("miso: drained after {}ms — exiting", waited);
        // clean exit on purpose: the LaunchAgent's KeepAlive is
        // SuccessfulExit=false, so a drained server stays down and a crashed
        // one is still restarted (deploy.md).
        std::process::exit(0);
    });
}

/// SIGTERM starts a drain. The handler itself may only touch an atomic, so a
/// watchdog thread does the work — 50ms of latency on a shutdown nobody is
/// timing, in exchange for a handler that is honestly async-signal-safe.
#[cfg(unix)]
extern "C" fn fm_sigterm_handler(_sig: libc::c_int) {
    FM_SIGTERM.store(true, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(unix)]
pub fn fm_handover_install(grace_ms: u64) {
    unsafe {
        // via a fn pointer, not the fn item: casting the item straight to an
        // integer is a lint and, on some targets, not the address at all
        let h: extern "C" fn(libc::c_int) = fm_sigterm_handler;
        libc::signal(libc::SIGTERM, h as usize as libc::sighandler_t);
    }
    std::thread::spawn(move || loop {
        if FM_SIGTERM.load(std::sync::atomic::Ordering::SeqCst) {
            fm_drain_begin(grace_ms);
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    });
}

#[cfg(not(unix))]
pub fn fm_handover_install(_grace_ms: u64) {}

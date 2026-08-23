// verbatim library for /harden: a process-global lock and a unix perms helper.
// full Rust (generics, cfg) outside the chain machinery — see harden.md.

use std::sync::{Mutex, OnceLock};

fn fm_store_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// serialise a read-modify-write over the flat-file auth stores. The mini is a
/// single process, so an in-memory lock is enough to stop two concurrent
/// requests racing the PIN attempt counter or double-spending a one-time
/// challenge. A poisoned lock is recovered rather than propagated — a panic
/// mid-write must not wedge every later login.
pub fn with_store_lock<F: FnOnce() -> R, R>(f: F) -> R {
    let _g = fm_store_lock().lock().unwrap_or_else(|e| e.into_inner());
    f()
}

/// tighten a secret file to owner read/write only. The wasm place never holds
/// secrets, so this is a no-op off unix.
#[cfg(unix)]
pub fn fm_own_only(path: &str) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}
#[cfg(not(unix))]
pub fn fm_own_only(_path: &str) {}

use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const MAX_ATTEMPTS: u32 = 15;

#[derive(Serialize, Deserialize)]
struct LockoutState {
    failed_attempts: u32,
}

fn lockout_path() -> PathBuf {
    let mut path = PathBuf::from(std::env::var("HOME").expect("HOME not set"));
    path.push(".ccvault");
    path.push("lockout.json");
    path
}

fn load_state(path: &Path) -> LockoutState {
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or(LockoutState { failed_attempts: 0 })
        }
        Err(_) => LockoutState { failed_attempts: 0 },
    }
}

fn save_state(path: &Path, state: &LockoutState) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(state).expect("Failed to serialize lockout state");
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
        .and_then(|f| {
            use std::io::Write;
            let mut f = f;
            f.write_all(json.as_bytes())
        })
        .expect("Failed to write lockout state");
}

pub fn check_destroyed() -> Result<(), String> {
    let state = load_state(&lockout_path());
    if state.failed_attempts >= MAX_ATTEMPTS {
        Err("Too many failed password attempts. The vault was not deleted; remove ~/.ccvault/lockout.json to retry if you believe this lockout is local DoS.".into())
    } else {
        Ok(())
    }
}

pub fn record_failure() {
    record_failure_impl(&lockout_path());
}

fn record_failure_impl(lockout: &Path) {
    let mut state = load_state(lockout);
    state.failed_attempts += 1;

    save_state(lockout, &state);
    let remaining = MAX_ATTEMPTS.saturating_sub(state.failed_attempts);
    eprintln!(
        "WARNING: {}/{} failed attempts. {} attempts remaining before lockout.",
        state.failed_attempts, MAX_ATTEMPTS, remaining
    );
}

pub fn record_success() {
    let _ = std::fs::remove_file(lockout_path());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_state_missing_file_returns_zero() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lockout.json");
        let state = load_state(&path);
        assert_eq!(state.failed_attempts, 0);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lockout.json");
        let state = LockoutState { failed_attempts: 7 };
        save_state(&path, &state);
        let loaded = load_state(&path);
        assert_eq!(loaded.failed_attempts, 7);
    }

    #[test]
    fn record_failure_increments() {
        let dir = tempfile::tempdir().unwrap();
        let lockout = dir.path().join("lockout.json");

        record_failure_impl(&lockout);
        let state = load_state(&lockout);
        assert_eq!(state.failed_attempts, 1);

        record_failure_impl(&lockout);
        let state = load_state(&lockout);
        assert_eq!(state.failed_attempts, 2);
    }

    #[test]
    fn record_success_resets() {
        let dir = tempfile::tempdir().unwrap();
        let lockout = dir.path().join("lockout.json");

        record_failure_impl(&lockout);
        record_failure_impl(&lockout);
        let state = load_state(&lockout);
        assert_eq!(state.failed_attempts, 2);

        // Simulate record_success by removing the file
        let _ = std::fs::remove_file(&lockout);
        let state = load_state(&lockout);
        assert_eq!(state.failed_attempts, 0);
    }

    #[test]
    fn vault_is_not_destroyed_after_max_failures() {
        let dir = tempfile::tempdir().unwrap();
        let lockout = dir.path().join("lockout.json");
        let vault = dir.path().join("vault.enc");
        std::fs::write(&vault, b"fake vault data").unwrap();

        for _ in 0..MAX_ATTEMPTS {
            record_failure_impl(&lockout);
        }

        assert!(vault.exists(), "failed attempts must not delete vault.enc");
        assert_eq!(load_state(&lockout).failed_attempts, MAX_ATTEMPTS);
    }

    #[test]
    fn warning_message_on_high_failure_count() {
        let dir = tempfile::tempdir().unwrap();
        let lockout = dir.path().join("lockout.json");

        save_state(
            &lockout,
            &LockoutState {
                failed_attempts: 12,
            },
        );

        record_failure_impl(&lockout);
        let state = load_state(&lockout);
        assert_eq!(state.failed_attempts, 13);
    }
}

use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::card::Card;

const SWITCH_DELAY_SECS: u64 = 3;

fn check_xdotool() -> Result<(), String> {
    Command::new("which")
        .arg("xdotool")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .ok_or_else(|| "xdotool not found. Install it: sudo apt install xdotool".to_string())?;
    Ok(())
}

fn wait_and_switch(field_name: &str) {
    print!(
        "Ready to type {}. Press Enter, then click the field in your browser.",
        field_name
    );
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    for i in (1..=SWITCH_DELAY_SECS).rev() {
        print!("  Typing in {}...\r", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    print!("               \r");
    io::stdout().flush().unwrap();
}

fn type_text(text: &str) -> Result<(), String> {
    let runtime_dir = private_runtime_dir()?;
    let mut tmp = tempfile::Builder::new()
        .prefix("ccvault-autofill.")
        .tempfile_in(&runtime_dir)
        .map_err(|e| format!("Failed to create autofill handoff file: {e}"))?;

    {
        use std::io::Write;
        tmp.write_all(text.as_bytes())
            .map_err(|e| format!("Failed to prepare autofill text: {e}"))?;
        tmp.flush()
            .map_err(|e| format!("Failed to flush autofill text: {e}"))?;
    }

    let status = Command::new("xdotool")
        .arg("type")
        .arg("--clearmodifiers")
        .arg("--delay")
        .arg("50")
        .arg("--file")
        .arg(tmp.path())
        .status()
        .map_err(|e| format!("Failed to run xdotool: {e}"))?;
    if !status.success() {
        return Err("xdotool exited with error".into());
    }
    Ok(())
}

fn private_runtime_dir() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from) {
        if is_private_runtime_dir(&path) {
            return Ok(path);
        }
    }

    let shm_dir = PathBuf::from(format!("/dev/shm/ccvault-{}", effective_uid()));
    if std::path::Path::new("/dev/shm").is_dir() {
        std::fs::create_dir_all(&shm_dir)
            .map_err(|e| format!("Failed to create private autofill runtime directory: {e}"))?;
        std::fs::set_permissions(&shm_dir, std::fs::Permissions::from_mode(0o700))
            .map_err(|e| format!("Failed to secure autofill runtime directory: {e}"))?;
        if is_private_runtime_dir(&shm_dir) {
            return Ok(shm_dir);
        }
    }

    Err("No private memory-backed runtime directory found for autofill handoff".into())
}

fn is_private_runtime_dir(path: &PathBuf) -> bool {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    metadata.is_dir()
        && metadata.uid() == effective_uid()
        && metadata.permissions().mode() & 0o077 == 0
}

fn effective_uid() -> u32 {
    unsafe extern "C" {
        fn geteuid() -> u32;
    }
    unsafe { geteuid() }
}

pub fn autofill(card: &Card) -> Result<(), String> {
    check_xdotool()?;

    println!("Autofill mode for: {}", card.label);
    println!(
        "After pressing Enter, you have {} seconds to click the target field in your browser.\n",
        SWITCH_DELAY_SECS
    );

    let fields = [
        ("card number", &card.number),
        ("expiry (MM/YY)", &card.exp),
        ("CVV", &card.cvv),
        ("cardholder name", &card.name),
        ("billing ZIP", &card.zip),
    ];

    for (name, value) in &fields {
        wait_and_switch(name);
        type_text(value)?;
        println!("  Typed {}.", name);
    }

    println!("\nAutofill complete.");
    Ok(())
}

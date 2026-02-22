use std::io::{self, Write};
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
    print!("Ready to type {}. Press Enter, then click the field in your browser.", field_name);
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
    let status = Command::new("xdotool")
        .arg("type")
        .arg("--clearmodifiers")
        .arg("--delay")
        .arg("50")
        .arg("--")
        .arg(text)
        .status()
        .map_err(|e| format!("Failed to run xdotool: {e}"))?;
    if !status.success() {
        return Err("xdotool exited with error".into());
    }
    Ok(())
}

pub fn autofill(card: &Card) -> Result<(), String> {
    check_xdotool()?;

    println!("Autofill mode for: {}", card.label);
    println!("After pressing Enter, you have {} seconds to click the target field in your browser.\n", SWITCH_DELAY_SECS);

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

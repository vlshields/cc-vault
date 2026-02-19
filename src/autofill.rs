use std::io::{self, Write};
use std::process::Command;

use crate::card::Card;

fn check_xdotool() -> Result<(), String> {
    Command::new("which")
        .arg("xdotool")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .ok_or_else(|| "xdotool not found. Install it: sudo apt install xdotool".to_string())?;
    Ok(())
}

fn wait_for_enter(field_name: &str) {
    print!("Focus the {} field, then press Enter...", field_name);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
}

fn type_text(text: &str) -> Result<(), String> {
    let status = Command::new("xdotool")
        .arg("type")
        .arg("--delay")
        .arg("50")
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
    println!("Switch to your browser and focus each field when prompted.\n");

    let fields = [
        ("card number", &card.number),
        ("expiry (MM/YY)", &card.exp),
        ("CVV", &card.cvv),
        ("cardholder name", &card.name),
        ("billing ZIP", &card.zip),
    ];

    for (name, value) in &fields {
        wait_for_enter(name);
        type_text(value)?;
        println!("  Typed {}.", name);
    }

    println!("\nAutofill complete.");
    Ok(())
}

use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::card::Card;

fn check_xclip() -> Result<(), String> {
    Command::new("which")
        .arg("xclip")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .ok_or_else(|| "xclip not found. Install it: sudo apt install xclip".to_string())?;
    Ok(())
}

fn set_clipboard(text: &str) -> Result<(), String> {
    let mut child = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run xclip: {e}"))?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(text.as_bytes())
        .map_err(|e| format!("Failed to write to xclip: {e}"))?;

    let status = child.wait().map_err(|e| format!("xclip error: {e}"))?;
    if !status.success() {
        return Err("xclip exited with error".into());
    }
    Ok(())
}

fn clear_clipboard() -> Result<(), String> {
    set_clipboard("")
}

pub fn get_field<'a>(card: &'a Card, field: &str) -> Result<&'a str, String> {
    match field {
        "number" => Ok(&card.number),
        "exp" => Ok(&card.exp),
        "cvv" => Ok(&card.cvv),
        "name" => Ok(&card.name),
        "zip" => Ok(&card.zip),
        _ => Err(format!(
            "Unknown field: {field}\nValid fields: number, exp, cvv, name, zip"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_card() -> Card {
        Card {
            label: "Test Visa".into(),
            number: "4111111111111111".into(),
            exp: "12/25".into(),
            cvv: "123".into(),
            name: "John Doe".into(),
            zip: "90210".into(),
        }
    }

    #[test]
    fn get_field_number() {
        let card = sample_card();
        assert_eq!(get_field(&card, "number").unwrap(), "4111111111111111");
    }

    #[test]
    fn get_field_exp() {
        let card = sample_card();
        assert_eq!(get_field(&card, "exp").unwrap(), "12/25");
    }

    #[test]
    fn get_field_cvv() {
        let card = sample_card();
        assert_eq!(get_field(&card, "cvv").unwrap(), "123");
    }

    #[test]
    fn get_field_name() {
        let card = sample_card();
        assert_eq!(get_field(&card, "name").unwrap(), "John Doe");
    }

    #[test]
    fn get_field_zip() {
        let card = sample_card();
        assert_eq!(get_field(&card, "zip").unwrap(), "90210");
    }

    #[test]
    fn get_field_invalid() {
        let card = sample_card();
        let err = get_field(&card, "ssn").unwrap_err();
        assert!(err.contains("Unknown field"));
        assert!(err.contains("ssn"));
    }

    #[test]
    fn get_field_empty_string() {
        let card = sample_card();
        let err = get_field(&card, "").unwrap_err();
        assert!(err.contains("Unknown field"));
    }
}

pub fn copy_to_clipboard(card: &Card, field: &str) -> Result<(), String> {
    check_xclip()?;

    let value = get_field(card, field)?;
    set_clipboard(value)?;

    println!("Copied {field} to clipboard.");
    print!("Clearing in 10s...");
    std::io::stdout().flush().unwrap();

    for i in (1..=10).rev() {
        thread::sleep(Duration::from_secs(1));
        print!("\rClearing in {i:2}s...");
        std::io::stdout().flush().unwrap();
    }

    clear_clipboard()?;
    println!("\rClipboard cleared.   ");
    Ok(())
}

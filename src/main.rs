mod autofill;
mod card;
mod clipboard;
mod vault;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccvault", about = "Local encrypted credit card vault")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactively add a card
    Add,
    /// List card labels
    List,
    /// Show full card details
    Show { label: String },
    /// Remove a card
    Remove { label: String },
    /// Autofill card fields via xdotool
    Fill {
        /// The card name whos' info you wish to autofill (as enderd by ccvault add)
        label: String,
    },
    /// Copy a card field to clipboard (cleared after 10 seconds)
    Copy {
        /// The card label
        label: String,
        /// Field to copy: number, exp, cvv, name, zip
        field: String,
    },
    /// Re-encrypt vault with a new master password
    ChangePassword,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => cmd_add(),
        Commands::List => cmd_list(),
        Commands::Show { label } => cmd_show(&label),
        Commands::Remove { label } => cmd_remove(&label),
        Commands::Fill { label } => cmd_fill(&label),
        Commands::Copy { label, field } => cmd_copy(&label, &field),
        Commands::ChangePassword => cmd_change_password(),
    }
}

fn cmd_add() {
    let password = vault::ask_password("Master password: ");
    // Verify password if vault already exists
    let mut cards = match vault::load_cards(&password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let card = card::Card::prompt_interactive();
    println!("Adding card: {}", card.label);
    cards.push(card);

    if let Err(e) = vault::save_cards(&cards, &password) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    println!("Card saved.");
}

fn cmd_list() {
    let password = vault::ask_password("Master password: ");
    let cards = match vault::load_cards(&password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    if cards.is_empty() {
        println!("No cards stored.");
        return;
    }

    println!("Stored cards:");
    for card in &cards {
        println!("  - {}", card.label);
    }
}

fn cmd_show(label: &str) {
    let password = vault::ask_password("Master password: ");
    let cards = match vault::load_cards(&password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    match find_card(&cards, label) {
        Some(card) => card.display(),
        None => {
            eprintln!("No card found with label: {label}");
            std::process::exit(1);
        }
    }
}

fn cmd_remove(label: &str) {
    let password = vault::ask_password("Master password: ");
    let mut cards = match vault::load_cards(&password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let before = cards.len();
    cards.retain(|c| c.label != label);

    if cards.len() == before {
        eprintln!("No card found with label: {label}");
        std::process::exit(1);
    }

    if let Err(e) = vault::save_cards(&cards, &password) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    println!("Removed card: {label}");
}

fn cmd_fill(label: &str) {
    let password = vault::ask_password("Master password: ");
    let cards = match vault::load_cards(&password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    match find_card(&cards, label) {
        Some(card) => {
            if let Err(e) = autofill::autofill(card) {
                eprintln!("Autofill error: {e}");
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("No card found with label: {label}");
            std::process::exit(1);
        }
    }
}

fn cmd_copy(label: &str, field: &str) {
    let password = vault::ask_password("Master password: ");
    let cards = match vault::load_cards(&password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    match find_card(&cards, label) {
        Some(card) => {
            if let Err(e) = clipboard::copy_to_clipboard(card, field) {
                eprintln!("Clipboard error: {e}");
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("No card found with label: {label}");
            std::process::exit(1);
        }
    }
}

fn cmd_change_password() {
    let old_password = vault::ask_password("Current master password: ");
    let cards = match vault::load_cards(&old_password) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let new_password = vault::ask_password("New master password: ");
    let confirm = vault::ask_password("Confirm new password: ");

    if new_password != confirm {
        eprintln!("Passwords do not match.");
        std::process::exit(1);
    }

    if let Err(e) = vault::save_cards(&cards, &new_password) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    println!("Password changed. Vault re-encrypted.");
}

fn find_card<'a>(cards: &'a [card::Card], label: &str) -> Option<&'a card::Card> {
    cards.iter().find(|c| c.label == label)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cards() -> Vec<card::Card> {
        vec![
            card::Card {
                label: "Chase Visa".into(),
                number: "4111111111111111".into(),
                exp: "12/25".into(),
                cvv: "123".into(),
                name: "John Doe".into(),
                zip: "90210".into(),
            },
            card::Card {
                label: "Amex Gold".into(),
                number: "378282246310005".into(),
                exp: "06/26".into(),
                cvv: "1234".into(),
                name: "Jane Doe".into(),
                zip: "10001".into(),
            },
        ]
    }

    #[test]
    fn find_card_existing() {
        let cards = sample_cards();
        let result = find_card(&cards, "Chase Visa");
        assert!(result.is_some());
        assert_eq!(result.unwrap().number, "4111111111111111");
    }

    #[test]
    fn find_card_second_entry() {
        let cards = sample_cards();
        let result = find_card(&cards, "Amex Gold");
        assert!(result.is_some());
        assert_eq!(result.unwrap().cvv, "1234");
    }

    #[test]
    fn find_card_not_found() {
        let cards = sample_cards();
        assert!(find_card(&cards, "Nonexistent").is_none());
    }

    #[test]
    fn find_card_empty_list() {
        let cards: Vec<card::Card> = vec![];
        assert!(find_card(&cards, "anything").is_none());
    }

    #[test]
    fn find_card_case_sensitive() {
        let cards = sample_cards();
        assert!(find_card(&cards, "chase visa").is_none());
    }
}

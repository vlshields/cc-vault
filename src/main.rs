mod autofill;
mod card;
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
    Fill { label: String },
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

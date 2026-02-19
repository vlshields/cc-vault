use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Serialize, Deserialize, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Card {
    pub label: String,
    pub number: String,
    pub exp: String,
    pub cvv: String,
    pub name: String,
    pub zip: String,
}

impl Card {
    pub fn prompt_interactive() -> Card {
        println!("Enter card details:");

        let label = prompt("  Label (e.g. Chase Visa): ");
        let number = prompt("  Card number: ");
        let exp = prompt("  Expiry (MM/YY): ");
        let cvv = prompt("  CVV: ");
        let name = prompt("  Cardholder name: ");
        let zip = prompt("  Billing ZIP: ");

        Card {
            label,
            number,
            exp,
            cvv,
            name,
            zip,
        }
    }

    pub fn display(&self) {
        println!("  Label:  {}", self.label);
        println!("  Number: {}", self.number);
        println!("  Exp:    {}", self.exp);
        println!("  CVV:    {}", self.cvv);
        println!("  Name:   {}", self.name);
        println!("  ZIP:    {}", self.zip);
    }
}

fn prompt(msg: &str) -> String {
    use std::io::{self, Write};
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

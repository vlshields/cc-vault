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
    fn serialize_deserialize_roundtrip() {
        let card = sample_card();
        let json = serde_json::to_string(&card).unwrap();
        let restored: Card = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.label, card.label);
        assert_eq!(restored.number, card.number);
        assert_eq!(restored.exp, card.exp);
        assert_eq!(restored.cvv, card.cvv);
        assert_eq!(restored.name, card.name);
        assert_eq!(restored.zip, card.zip);
    }

    #[test]
    fn serialize_vec_roundtrip() {
        let cards = vec![sample_card(), sample_card()];
        let json = serde_json::to_vec(&cards).unwrap();
        let restored: Vec<Card> = serde_json::from_slice(&json).unwrap();
        assert_eq!(restored.len(), 2);
        assert_eq!(restored[0].label, "Test Visa");
    }

    #[test]
    fn deserialize_empty_vec() {
        let json = b"[]";
        let cards: Vec<Card> = serde_json::from_slice(json).unwrap();
        assert!(cards.is_empty());
    }

    #[test]
    fn deserialize_rejects_missing_field() {
        let json = r#"{"label":"x","number":"1","exp":"1","cvv":"1","name":"n"}"#;
        let result = serde_json::from_str::<Card>(json);
        assert!(result.is_err());
    }
}

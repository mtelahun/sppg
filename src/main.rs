use std::collections::HashMap;

use rand::distributions::{Distribution, Uniform};

pub mod cli;
pub mod original_wordlist;

use crate::cli::process_command_line;
use original_wordlist::ORIGINAL_WORDLIST;

fn main() {
    let cli_args = process_command_line();
    let num_choices = cli_args.num_of_pass;
    let diceware_map: HashMap<&str, &str> = ORIGINAL_WORDLIST.into_iter().collect();
    let list = iterate(num_choices, &diceware_map);
    for l in list {
        println!("{l}");
    }
}

fn iterate(iterations: u8, diceware_map: &HashMap<&str, &str>) -> Vec<String> {
    let mut list = Vec::<String>::new();
    for _ in 0..iterations {
        let mut passphrase = String::new();
        for _ in 0..4 {
            let lookup = roll_dice();
            let word = lookup_word(&lookup, diceware_map);
            passphrase.push_str(&format!("{word} "));
        }
        list.push(passphrase.trim().to_string())
    }

    list
}

fn roll_dice() -> String {
    let mut lookup_number = String::new();
    let dice = Uniform::from(1..7);
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let number = dice.sample(&mut rng);
        lookup_number.push(char::from_digit(number, 10).unwrap());
    }

    lookup_number
}

fn lookup_word<'a>(lookup: &str, diceware_map: &'a HashMap<&'a str, &'a str>) -> &'a str {
    diceware_map.get(lookup).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_digits() {
        let lookup_num = roll_dice();

        assert_eq!(lookup_num.len(), 5, "The lookup number is 5 digits long");

        for i in 0..lookup_num.len() {
            let ch = lookup_num.as_bytes()[i];
            assert!(
                ch >= b'1' && ch <= b'6',
                "Each character is between 1 - 6, inclusive"
            )
        }
    }

    #[test]
    fn lookup_number_retrieves_word() {
        let cases = [("11111", "a"), ("36355", "levi"), ("66666", "\"@")];

        let diceware_map: HashMap<&str, &str> = ORIGINAL_WORDLIST.into_iter().collect();
        for (index, expected) in cases {
            let word = lookup_word(&index, &diceware_map);
            assert_eq!(
                word, expected,
                "lookup index {}, expected {}",
                index, expected
            )
        }
    }

    #[test]
    fn default_iterations() {
        let cli_args = process_command_line();
        let num_choices = cli_args.num_of_pass;
        let diceware_map: HashMap<&str, &str> = ORIGINAL_WORDLIST.into_iter().collect();
        let list = iterate(num_choices, &diceware_map);

        assert_eq!(list.len(), 6, "number of passphrases is 6");
    }
}

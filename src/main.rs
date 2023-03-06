use std::collections::HashMap;

use rand::distributions::{Distribution, Uniform};

#[macro_use]
extern crate lazy_static;

mod cli;
mod eff_wordlist;
mod original_wordlist;

use crate::{
    cli::{process_command_line, Args},
    eff_wordlist::EFF_WORDLIST,
    original_wordlist::ORIGINAL_WORDLIST,
};

lazy_static! {
    static ref LIST_ORIG: HashMap<&'static str, &'static str> =
        ORIGINAL_WORDLIST.into_iter().collect();
}
lazy_static! {
    static ref LIST_EFF: HashMap<&'static str, &'static str> = EFF_WORDLIST.into_iter().collect();
}

fn main() {
    let cli_args = process_command_line();
    let list = iterate(&cli_args);
    for l in list {
        println!("{l}");
    }
}

fn choose_word_list(cli_args: &Args) -> &'static HashMap<&'static str, &'static str> {
    if cli_args.eff_word_list {
        return &LIST_EFF;
    }

    &LIST_ORIG
}

fn iterate(cli_args: &Args) -> Vec<String> {
    let word_count = cli_args.word_count;
    let iterations = cli_args.num_of_pass;
    let diceware_map = choose_word_list(cli_args);
    let mut list = Vec::<String>::new();
    for _ in 0..iterations {
        let mut passphrase = String::new();
        for _ in 0..word_count {
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

fn lookup_word(
    lookup: &str,
    diceware_map: &'static HashMap<&'static str, &'static str>,
) -> &'static str {
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

        let cli_args = process_command_line();
        let diceware_map = choose_word_list(&cli_args);
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
    fn default_iterations_and_word_count() {
        let cli_args = process_command_line();
        let num_choices = cli_args.num_of_pass;
        let word_count = cli_args.word_count;
        let list = iterate(&cli_args);

        assert_eq!(list.len(), 6, "number of passphrases is {}", num_choices);

        for l in list {
            let list: Vec<&str> = l.split_whitespace().collect();

            assert_eq!(list.len(), 5, "words in passphrase = {}", word_count);
        }
    }

    #[test]
    fn iterations_and_word_count() {
        let num_choices = 12;
        let word_count = 15;
        let mut cli_args = process_command_line();
        cli_args.num_of_pass = num_choices;
        cli_args.word_count = word_count;
        let list = iterate(&cli_args);

        assert_eq!(list.len(), 12, "number of passphrases is {}", num_choices);

        for l in list {
            let list: Vec<&str> = l.split_whitespace().collect();

            assert_eq!(list.len(), 15, "words in passphrase = {}", word_count);
        }
    }

    #[test]
    fn choose_wordlist_default() {
        let args = process_command_line();
        let map = choose_word_list(&args);
        assert_eq!(
            lookup_word(&"11111", map),
            "a",
            "default wordlist is the original one",
        );
    }

    #[test]
    fn choose_wordlist_eff() {
        let mut args = process_command_line();
        args.eff_word_list = true;
        let map = choose_word_list(&args);

        assert_eq!(
            lookup_word("11111", map),
            "abacus",
            "when -e is used wordlist is the EFF one",
        );
    }
}

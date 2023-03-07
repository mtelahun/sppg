use std::collections::HashMap;

use passphrase::PassPhrase;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

#[macro_use]
extern crate lazy_static;

mod cli;
mod passphrase;
mod wordlist;

use crate::{
    cli::{process_command_line, Args},
    wordlist::eff::EFF_WORDLIST,
    wordlist::original::ORIGINAL_WORDLIST,
    wordlist::special_char::SPECIAL_CHARS,
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

fn iterate(cli_args: &Args) -> Vec<PassPhrase> {
    let word_count = cli_args.word_count;
    let iterations = cli_args.num_of_pass;
    let diceware_map = choose_word_list(cli_args);
    let mut list = Vec::<PassPhrase>::new();
    for _ in 0..iterations {
        let mut passphrase = PassPhrase::new();
        for _ in 0..word_count {
            let lookup = roll_dice_5_times();
            let word = lookup_word(&lookup, diceware_map);
            passphrase.push(word);
        }
        if passphrase.is_insecure() {
            continue;
        }
        list.push(passphrase)
    }
    if list.is_empty() && iterations > 0 {
        eprintln!("error: unable to derive a secure enough passphrase");
        eprintln!("error: try increasing the word count or adding a special character.");
    }

    if cli_args.use_special_char {
        for pp in &mut list {
            let ch = roll_for_special_char();
            let _ = add_special_char(pp, ch).clone();
        }
    }

    list
}

fn add_special_char(pp: &mut PassPhrase, ch: char) -> &PassPhrase {
    let dice = Uniform::from(1..pp.len() as u32);
    let mut rng = rand::thread_rng();
    let idx_word = roll_dice(&dice, &mut rng);

    let len_word = pp[idx_word].len();
    let dice = Uniform::from(0..(len_word - 1) as u32);
    let mut rng = rand::thread_rng();
    let idx_char = roll_dice(&dice, &mut rng);

    let word = &pp[idx_word];
    let w1 = &word[0..idx_char];
    let w2 = &word[idx_char..];
    pp[idx_word] = format!("{w1}{ch}{w2}");

    pp
}

fn roll_for_special_char() -> char {
    let dice = Uniform::from(0..6);
    let mut rng = rand::thread_rng();
    let x = roll_dice(&dice, &mut rng);
    let y = roll_dice(&dice, &mut rng);

    SPECIAL_CHARS[x][y]
}

fn roll_dice_5_times() -> String {
    let mut lookup_number = String::new();
    let dice = Uniform::from(1..7);
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let number = roll_dice(&dice, &mut rng);
        lookup_number.push(char::from_digit(number as u32, 10).unwrap());
    }

    lookup_number
}

fn roll_dice(dice: &Uniform<u32>, rng: &mut ThreadRng) -> usize {
    dice.sample(rng) as usize
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
        let lookup_num = roll_dice_5_times();

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

        // Because there may be insecure passphrases we can't test for equality
        assert!(
            list.len() <= 6,
            "number of passphrases is <= {}",
            num_choices
        );

        for pp in list {
            assert_eq!(pp.len(), 5, "words in passphrase = {}", word_count);
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

        // Because there may be insecure passphrases we can't test for equality
        assert!(
            list.len() <= 12,
            "number of passphrases is <= {}",
            num_choices
        );

        for pp in list {
            assert_eq!(pp.len(), 15, "words in passphrase = {}", word_count);
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

    #[test]
    fn special_char_handling() {
        let num_choices = 1;
        let mut cli_args = process_command_line();
        cli_args.num_of_pass = num_choices;
        let mut list = iterate(&cli_args);
        let special_char = roll_for_special_char();
        let new_pp = add_special_char(&mut list[0], special_char);

        let output = format!("{}", new_pp);
        assert!(
            output.contains(special_char),
            "the passphrase contains a special char"
        );
    }

    #[test]
    fn no_insecure_passphrases() {
        let num_choices = 1;
        let word_count = 1;
        let mut cli_args = process_command_line();
        cli_args.num_of_pass = num_choices;
        cli_args.word_count = word_count;
        let list = iterate(&cli_args);

        assert_eq!(list.len(), 0, "list of passphrases is empty");
    }
}

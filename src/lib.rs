use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use std::collections::HashMap;

pub mod cli;
pub mod passphrase;
mod wordlist;
use cli::Args;
use passphrase::PassPhrase;
use wordlist::{eff::EFF_WORDLIST, original::ORIGINAL_WORDLIST, special_char::SPECIAL_CHARS};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LIST_ORIG: HashMap<&'static str, &'static str> =
        ORIGINAL_WORDLIST.into_iter().collect();
}
lazy_static! {
    static ref LIST_EFF: HashMap<&'static str, &'static str> = EFF_WORDLIST.into_iter().collect();
}

pub fn choose_word_list(cli_args: &Args) -> &'static HashMap<&'static str, &'static str> {
    if cli_args.eff {
        return &LIST_EFF;
    }

    &LIST_ORIG
}

pub fn iterate(cli_args: &Args) -> Vec<PassPhrase> {
    let separator = cli_args.separator;
    let word_count = cli_args.word_count;
    let iterations = cli_args.num_of_pass;
    let diceware_map = choose_word_list(cli_args);
    let mut list = Vec::<PassPhrase>::new();
    for _ in 0..iterations {
        let mut passphrase = PassPhrase::new(separator);
        for _ in 0..word_count {
            let lookup = roll_dice_5_times();
            let word = lookup_word(&lookup, diceware_map);
            passphrase.push(word);
        }

        // This needs to be done before adding a special character so as to
        // not run the risk of attempting to convert a special character
        // to uppercase.
        if cli_args.use_capital_char {
            add_capital_char(&mut passphrase);
        }

        if cli_args.use_special_char {
            let ch = roll_for_special_char();
            let _ = add_special_char(&mut passphrase, ch).clone();
        }

        if passphrase.is_insecure() {
            continue;
        }
        list.push(passphrase)
    }
    if list.is_empty() && iterations > 0 {
        eprintln!("error: unable to derive a secure enough passphrase");
        eprintln!("error: try increasing the word count or adding quality (--quality).");
    }

    list
}

pub fn add_special_char(pp: &mut PassPhrase, ch: char) -> &PassPhrase {
    let dice = Uniform::from(0..pp.len() as u32);
    let mut rng = rand::thread_rng();
    let idx_word = roll_dice(&dice, &mut rng);

    let len_word = pp[idx_word].len();
    let mut idx_char: usize = 0;
    if len_word > 1 {
        let dice = Uniform::from(0..len_word as u32);
        let mut rng = rand::thread_rng();
        idx_char = roll_dice(&dice, &mut rng);
    }

    if idx_char == 0 {
        let w1 = &pp[idx_word][..len_word];
        pp[idx_word] = format!("{ch}{w1}");
    } else {
        let word = &pp[idx_word];
        let w1 = &word[0..=idx_char];
        let w2 = &word[idx_char + 1..];
        pp[idx_word] = format!("{w1}{ch}{w2}");
    }

    pp
}

pub fn add_capital_char(pp: &mut PassPhrase) -> &PassPhrase {
    let dice = Uniform::from(0..pp.len() as u32);
    let mut rng = rand::thread_rng();
    let idx_word = roll_dice(&dice, &mut rng);

    let len_word = pp[idx_word].len();
    let mut idx_char: usize = 0;
    if len_word > 1 {
        let dice = Uniform::from(0..len_word as u32);
        let mut rng = rand::thread_rng();
        idx_char = roll_dice(&dice, &mut rng);
    }

    let mut ch = String::from("");
    let word = &pp[idx_word];
    for (idx, c) in word.char_indices() {
        if idx == idx_char {
            // We convert to string because for some languages to_uppercase() may
            // return more than one char.
            if c.is_alphabetic() {
                ch = c.to_uppercase().to_string();
            }
        }
    }
    let w1 = &word[0..idx_char];
    let w2 = &word[idx_char + 1..];
    pp[idx_word] = format!("{w1}{ch}{w2}");

    pp
}

pub fn roll_for_special_char() -> char {
    let dice = Uniform::from(0..6);
    let mut rng = rand::thread_rng();
    let x = roll_dice(&dice, &mut rng);
    let y = roll_dice(&dice, &mut rng);

    SPECIAL_CHARS[x][y]
}

pub fn roll_dice_5_times() -> String {
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

pub fn lookup_word(
    lookup: &str,
    diceware_map: &'static HashMap<&'static str, &'static str>,
) -> &'static str {
    diceware_map.get(lookup).unwrap()
}

pub fn print_passphrases(list: &Vec<PassPhrase>) {
    for l in list {
        println!("{l}");
    }
}

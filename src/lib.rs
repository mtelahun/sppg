use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
    seq::SliceRandom,
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

    // Every gap is a candidate: before the word, between any two chars, after it.
    let word = &pp[idx_word];
    let gaps: Vec<usize> = word
        .char_indices()
        .map(|(i, _)| i)
        .chain([word.len()])
        .collect();
    let at = *gaps.choose(&mut rng).unwrap(); // never empty: always holds word.len()
    pp[idx_word] = format!("{}{ch}{}", &word[..at], &word[at..]);

    pp
}

pub fn add_capital_char(pp: &mut PassPhrase) -> &PassPhrase {
    // Pick only among lowercase letters so digits/punctuation are never dropped.
    let candidates: Vec<(usize, usize, char)> = (0..pp.len())
        .flat_map(|w| {
            pp[w]
                .char_indices()
                .filter(|(_, c)| c.is_lowercase())
                .map(move |(i, c)| (w, i, c))
                .collect::<Vec<_>>()
        })
        .collect();

    if let Some(&(w, i, c)) = candidates.choose(&mut rand::thread_rng()) {
        let word = &pp[w];
        // to_uppercase() may yield more than one char in some languages.
        pp[w] = format!(
            "{}{}{}",
            &word[..i],
            c.to_uppercase(),
            &word[i + c.len_utf8()..]
        );
    }

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

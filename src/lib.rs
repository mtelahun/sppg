use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
    seq::SliceRandom,
};
pub mod cli;
pub mod passphrase;
mod wordlist;
use cli::Args;
use passphrase::PassPhrase;
use wordlist::{eff::EFF_WORDLIST, original::ORIGINAL_WORDLIST, special_char::SPECIAL_CHARS};

const MAX_ATTEMPTS_PER_PASSPHRASE: usize = 1000;

pub type WordList = [(&'static str, &'static str); 7776];

pub fn choose_word_list(cli_args: &Args) -> &'static WordList {
    if cli_args.eff {
        return &EFF_WORDLIST;
    }

    &ORIGINAL_WORDLIST
}

pub fn iterate(cli_args: &Args) -> Vec<PassPhrase> {
    let separator = cli_args.separator;
    let word_count = cli_args.word_count;
    let iterations = cli_args.num_of_pass;
    let word_list = choose_word_list(cli_args);
    let mut list = Vec::<PassPhrase>::new();
    // Retry insecure rolls so the user gets the count they asked for; the cap
    // stops settings that can never pass (e.g. -w 3 without -q) from looping forever.
    let max_attempts = iterations as usize * MAX_ATTEMPTS_PER_PASSPHRASE;
    let mut attempts = 0;
    while list.len() < iterations as usize && attempts < max_attempts {
        attempts += 1;
        let mut passphrase = PassPhrase::new(separator);
        for _ in 0..word_count {
            let lookup = roll_dice_5_times();
            let word = lookup_word(&lookup, word_list);
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
            add_special_char(&mut passphrase, ch);
        }

        if passphrase.is_insecure() {
            continue;
        }
        list.push(passphrase)
    }
    if list.len() < iterations as usize {
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

pub fn lookup_word(lookup: &str, word_list: &'static WordList) -> &'static str {
    // Lists are sorted by dice number, so "11111".."66666" is base 6 with digits 1-6.
    let idx = lookup
        .bytes()
        .fold(0, |acc, d| acc * 6 + d.wrapping_sub(b'1') as usize);
    let (key, word) = word_list[idx];
    assert_eq!(key, lookup, "invalid dice roll");
    word
}

pub fn print_passphrases(list: &Vec<PassPhrase>) {
    for l in list {
        println!("{l}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn every_entry_found_by_its_dice_number() {
        for list in [&ORIGINAL_WORDLIST, &EFF_WORDLIST] {
            for (key, word) in list.iter() {
                assert_eq!(lookup_word(key, list), *word);
            }
        }
    }
}

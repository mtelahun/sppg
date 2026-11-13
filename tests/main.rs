use clap::Parser;
use sppg::{
    add_capital_char, add_special_char, choose_word_list, iterate, lookup_word,
    passphrase::PassPhrase, print_passphrases, roll_dice_5_times, roll_for_special_char,
};

#[test]
fn correct_digits() {
    let lookup_num = roll_dice_5_times();

    assert_eq!(lookup_num.len(), 5, "The lookup number is 5 digits long");

    for i in 0..lookup_num.len() {
        let ch = lookup_num.as_bytes()[i];
        assert!(
            (b'1'..=b'6').contains(&ch),
            "Each character is between 1 - 6, inclusive"
        )
    }
}

#[test]
fn lookup_number_retrieves_word() {
    let cases = [("11111", "a"), ("36355", "levi"), ("66666", "\"@")];

    let cli_args = default_args();
    let diceware_map = choose_word_list(&cli_args);
    for (index, expected) in cases {
        let word = lookup_word(index, diceware_map);
        assert_eq!(
            word, expected,
            "lookup index {}, expected {}",
            index, expected
        )
    }
}

#[test]
fn default_iterations_and_word_count() {
    let cli_args = default_args();
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
    let mut cli_args = default_args();
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
    let args = default_args();
    let map = choose_word_list(&args);
    assert_eq!(
        lookup_word("11111", map),
        "a",
        "default wordlist is the original one",
    );
}

#[test]
fn choose_wordlist_eff() {
    let mut args = default_args();
    args.eff = true;
    let map = choose_word_list(&args);

    assert_eq!(
        lookup_word("11111", map),
        "abacus",
        "when -e is used wordlist is the EFF one",
    );
}

#[test]
fn special_char_handling() {
    let mut cli_args = default_args();
    cli_args.num_of_pass = 1;
    cli_args.word_count = 7;
    let mut list = iterate(&cli_args);
    assert!(!list.is_empty());
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
    let mut cli_args = default_args();
    cli_args.num_of_pass = num_choices;
    cli_args.word_count = word_count;
    let list = iterate(&cli_args);

    assert_eq!(list.len(), 0, "list of passphrases is empty");
}

#[test]
fn capital_char_handling() {
    let mut pp = PassPhrase::new(None);
    pp.push("some");
    pp.push("phrase");
    let new_pp = add_capital_char(&mut pp);

    let mut contains_capital = false;
    let output = format!("{}", new_pp);
    for (_, ch) in output.char_indices() {
        if ch.is_uppercase() {
            contains_capital = true;
        }
    }

    assert!(contains_capital, "the passphrase contains a Capital letter");
}

#[test]
fn quality_for_short_phrases() {
    let mut cli_args = default_args();
    cli_args.num_of_pass = 1;
    cli_args.use_capital_char = true;
    cli_args.use_special_char = true;
    cli_args.word_count = 2;
    let mut list = iterate(&cli_args);
    while list.is_empty() {
        list = iterate(&cli_args);
    }
    let pp = &list[0];

    // contains capital
    let mut contains_capital = false;
    let output = format!("{}", pp);
    for (_, ch) in output.char_indices() {
        if ch.is_uppercase() {
            contains_capital = true;
        }
    }
    assert!(contains_capital, "the passphrase contains a Capital letter");

    // contains special char
    let mut contains_special = false;
    let output = format!("{}", pp);
    for (_, ch) in output.char_indices() {
        if ch.is_numeric() || ch.is_ascii_punctuation() {
            contains_special = true;
        }
    }
    assert!(contains_special, "the passphrase contains a special char");
}

#[test]
fn special_char_in_first_word() {
    let mut pp = PassPhrase::new(None);
    pp.push("some");
    pp.push("phrase");
    let mut contains_special_char = false;
    // assume 500 tries is enough to get at least one special char in the first word
    for _ in 0..500 {
        // Wordlist contains numbers so skip them to avoid false positives
        let mut ch = roll_for_special_char();
        while ch.is_ascii_digit() {
            ch = roll_for_special_char();
        }
        let new_pp = add_special_char(&mut pp, ch);

        let output = new_pp[0].clone();
        for (_, ch) in output.char_indices() {
            if ch.is_ascii_punctuation() {
                contains_special_char = true;
            }
        }
    }

    assert!(
        contains_special_char,
        "the first word in the passphrase contains a special char"
    );
}

#[test]
fn special_char_in_first_character() {
    let mut pp = PassPhrase::new(None);
    pp.push("some");
    pp.push("phrase");
    let mut contains_special_char = false;
    // assume 500 tries is enough to get at least one special char in the first word
    for _ in 0..500 {
        let ch = '*';
        let new_pp = add_special_char(&mut pp, ch);

        let output = new_pp[0].clone();
        if output[0..1] == ch.to_string() {
            contains_special_char = true;
        }
    }

    assert!(
        contains_special_char,
        "the first character in a word is a special char"
    );
}

#[test]
fn capital_in_first_word() {
    let mut pp = PassPhrase::new(None);
    pp.push("some");
    pp.push("phrase");
    let mut contains_special_char = false;
    // assume 500 tries is enough to get at least one special char in the first word
    for _ in 0..500 {
        let new_pp = add_capital_char(&mut pp);

        let output = new_pp[0].clone();
        for (_, ch) in output.char_indices() {
            if ch.is_uppercase() {
                contains_special_char = true;
            }
        }
    }

    assert!(
        contains_special_char,
        "the first word in the passphrase contains a capital letter"
    );
}

#[test]
fn printed_correctly() {
    let mut pp = PassPhrase::new(None);
    pp.push("some");
    pp.push("phrase");
    let list = vec![pp];
    print_passphrases(&list);

    assert_eq!(1, 1);
}

#[test]
fn default_separator_char_is_space() {
    let mut pp = PassPhrase::new(None);
    pp.push("some");
    pp.push("phrase");
    let display = format!("{}", pp);

    assert_eq!(
        display, "some phrase",
        "the default separator must be a space"
    )
}

#[test]
fn separator_char_in_output() {
    let mut pp = PassPhrase::new(Some('-'));
    pp.push("some");
    pp.push("phrase");
    let display = format!("{}", pp);

    assert_eq!(display, "some-phrase", "the separator '-' is displayed")
}

#[test]
fn capital_char_never_drops_non_letters() {
    // Words with digits/punctuation (as in the original diceware list) must keep
    // every character; only one lowercase letter gets uppercased.
    for _ in 0..500 {
        let mut pp = PassPhrase::new(None);
        pp.push("10").push("!").push("a&p").push("t-shirt");
        add_capital_char(&mut pp);

        let out = pp.to_string();
        assert_eq!(
            out.to_lowercase(),
            "10 ! a&p t-shirt",
            "no character was lost"
        );
        assert_eq!(out.chars().filter(|c| c.is_uppercase()).count(), 1);
    }

    // No letters at all: passphrase is left unchanged.
    let mut pp = PassPhrase::new(None);
    pp.push("10").push("!");
    add_capital_char(&mut pp);
    assert_eq!(pp.to_string(), "10 !");
}

#[test]
fn special_char_can_land_in_every_gap() {
    // "abc" has 4 gaps; all must be reachable, including between 'a' and 'b'.
    let mut seen = std::collections::HashSet::new();
    for _ in 0..1000 {
        let mut pp = PassPhrase::new(None);
        pp.push("abc");
        add_special_char(&mut pp, '!');
        seen.insert(pp.to_string());
    }
    let expected: std::collections::HashSet<String> = ["!abc", "a!bc", "ab!c", "abc!"]
        .into_iter()
        .map(String::from)
        .collect();
    assert_eq!(seen, expected);
}

#[test]
fn iterate_returns_requested_count() {
    // 4 words often fall under the 19-char minimum; those must be re-rolled, not dropped.
    let cli_args = sppg::cli::Args::try_parse_from(["sppg", "-w", "4", "-n", "50"]).unwrap();
    assert_eq!(iterate(&cli_args).len(), 50);
}

#[test]
fn impossible_settings_exit_with_error() {
    // 3 words without --quality can never pass the security check.
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_sppg"))
        .args(["-w", "3", "-n", "1"])
        .output()
        .unwrap()
        .status;
    assert!(!status.success());
}

// Build args explicitly: the test binary's own argv (e.g. a name filter) is not sppg's CLI.
fn default_args() -> sppg::cli::Args {
    sppg::cli::Args::try_parse_from(["sppg"]).unwrap()
}

#[test]
fn tests_run_with_name_filter() {
    // Re-run this test binary with a filter; it used to abort with
    // "unexpected argument" because tests parsed the real command line.
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["default_iterations_and_word_count", "--exact"])
        .output()
        .unwrap()
        .status;
    assert!(status.success());
}

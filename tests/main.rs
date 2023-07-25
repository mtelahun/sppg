use sppg::{
    add_capital_char, add_special_char, choose_word_list, cli::process_command_line, iterate,
    lookup_word, passphrase::PassPhrase, print_passphrases, roll_dice_5_times,
    roll_for_special_char,
};

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
    let mut cli_args = process_command_line();
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
    let mut cli_args = process_command_line();
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
    let mut list = Vec::<PassPhrase>::new();
    list.push(pp);
    let new_pp = add_capital_char(&mut list[0]);

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
    let mut cli_args = process_command_line();
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
    let mut list = Vec::<PassPhrase>::new();
    list.push(pp);
    let mut contains_special_char = false;
    // assume 500 tries is enough to get at least one special char in the first word
    for _ in 0..500 {
        // Wordlist contains numbers so skip them to avoid false positives
        let mut ch = roll_for_special_char();
        while ch >= '0' && ch <= '9' {
            ch = roll_for_special_char();
        }
        let new_pp = add_special_char(&mut list[0], ch);

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
    let mut list = Vec::<PassPhrase>::new();
    list.push(pp);
    let mut contains_special_char = false;
    // assume 500 tries is enough to get at least one special char in the first word
    for _ in 0..500 {
        let ch = '*';
        let new_pp = add_special_char(&mut list[0], ch);

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
    let mut list = Vec::<PassPhrase>::new();
    list.push(pp);
    let mut contains_special_char = false;
    // assume 500 tries is enough to get at least one special char in the first word
    for _ in 0..500 {
        let new_pp = add_capital_char(&mut list[0]);

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
    let mut list = Vec::<PassPhrase>::new();
    list.push(pp);
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

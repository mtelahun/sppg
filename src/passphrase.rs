use std::ops::{Index, IndexMut};

const CHAR_COUNT_MIN: usize = 19;
const WORD_COUNT_MIN: usize = 4;
const QUALITY_CHAR_COUNT_MIN: usize = 8;
const QUALITY_WORD_COUNT_MIN: usize = 2;

#[derive(Clone, Debug)]
pub struct PassPhrase {
    inner: Vec<String>,
}

impl PassPhrase {
    pub fn new() -> Self {
        Self {
            inner: Vec::<String>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, word: &str) -> &mut Self {
        self.inner.push(word.into());

        self
    }

    pub fn is_insecure(&self) -> bool {
        // All lowercase with less than 4 words is insecure
        let word_count = self.len();

        // we should count the spaces between the words
        let spaces = self.len() - 1;
        let mut char_length = 0;
        for word in &self.inner {
            char_length += word.chars().count();
        }

        // does the phrase have quality?
        let mut quality = 0;
        let mut uppercase = 0;
        let mut special = 0;
        let mut numeric = 0;
        for word in &self.inner {
            for ch in word.chars() {
                // keep track of numbers separately because they
                // appear in the regular and the special char lists
                if ch.is_numeric() {
                    numeric += 1;
                }
                if ch.is_ascii_punctuation() {
                    special += 1;
                }
                if ch.is_uppercase() {
                    uppercase += 1;
                }
            }
        }
        if uppercase > 0 {
            quality += 1;
        }
        // A quality of 3 is necessary because numbers can appear in
        // both the word list and the special char list. If there is
        // not a number but there is at least one upper case and one
        // special char then increase quality.
        if numeric > 0 || (special > 0 && uppercase > 0) {
            quality += 1;
        }
        // If there is no punctuation but there is at least one
        // number and one capital letter then increase quality.
        if special > 0 || (numeric > 0 && uppercase > 0) {
            quality += 1;
        }

        if (QUALITY_WORD_COUNT_MIN..WORD_COUNT_MIN).contains(&word_count) {
            if quality < 3 || (char_length + spaces) < QUALITY_CHAR_COUNT_MIN {
                return true;
            } else if quality >= 3 {
                return false;
            }
        }

        word_count < WORD_COUNT_MIN || (char_length + spaces) < CHAR_COUNT_MIN
    }
}

impl Default for PassPhrase {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PassPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pp = String::new();
        for word in &self.inner {
            pp.push_str(&format!("{word} "));
        }
        let pp = pp.trim();

        write!(f, "{}", pp)
    }
}

impl Index<usize> for PassPhrase {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for PassPhrase {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_insecure19() {
        let mut passphrase = PassPhrase::new();
        passphrase
            .push("this")
            .push("is")
            .push("19")
            .push("chars")
            .push("lo");

        assert!(!passphrase.is_insecure(), "passphrase IS 19 chars long");
    }

    #[test]
    fn test_is_insecure18() {
        let mut passphrase = PassPhrase::new();
        passphrase
            .push("this")
            .push("is")
            .push("19")
            .push("chars")
            .push("l");

        assert!(
            passphrase.is_insecure(),
            "insecure: passphrase is LESS THAN 19 chars"
        );
    }

    #[test]
    fn test_is_insecure_i18n() {
        let mut passphrase = PassPhrase::new();
        passphrase
            .push("1")
            .push("2")
            .push("3")
            .push("ラウトは難しいです！");

        assert!(
            passphrase.is_insecure(),
            "insecure: I18N passphrase is LESS THAN 19 chars"
        );
    }

    #[test]
    fn test_is_insecure_words3() {
        let mut passphrase = PassPhrase::new();
        passphrase
            .push("this_is_longer_than")
            .push("19_chars_but_it_is")
            .push("only_three_words");

        assert!(passphrase.is_insecure(), "passphrase is LESS THAN 4 words");
    }

    #[test]
    fn short_with_quality() {
        let mut passphrase = PassPhrase::new();
        passphrase.push("!a").push("shortA");

        assert!(
            !passphrase.is_insecure(),
            "passphrase is short but contains a capital and special char"
        );
    }

    #[test]
    fn default_impl() {
        #[derive(Default)]
        struct TestStruct {
            pp: PassPhrase,
        }
        let s = TestStruct {
            ..Default::default()
        };

        assert!(s.pp.is_empty());
    }
}

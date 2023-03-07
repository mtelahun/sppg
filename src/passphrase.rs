use std::ops::{Index, IndexMut};

const CHAR_COUNT_MIN: usize = 19;
const WORD_COUNT_MIN: usize = 4;

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

    pub fn push(&mut self, word: &str) -> &mut Self {
        self.inner.push(word.into());

        self
    }

    pub fn is_insecure(&self) -> bool {
        // Any thing less than 4 words is too insecure
        let word_count = self.len();

        // we should count the spaces between the words
        let spaces = self.len() - 1;
        let mut char_length = 0;
        for word in &self.inner {
            char_length += word.chars().count();
        }

        word_count < WORD_COUNT_MIN || (char_length + spaces) < CHAR_COUNT_MIN
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
}

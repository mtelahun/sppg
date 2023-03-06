use std::ops::{Index, IndexMut};

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

    pub fn push(&mut self, word: &str) {
        self.inner.push(word.into())
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

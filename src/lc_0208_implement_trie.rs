use std::{
    collections::{hash_map::Entry, HashMap},
    str::Chars,
};

#[derive(Default, Debug)]
pub struct Trie {
    trielist: Vec<Self>,
    children: HashMap<char, usize>,
    word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    fn char_insert(&mut self, mut iter: Chars) {
        if let Some(ch) = iter.next() {
            match self.children.entry(ch) {
                Entry::Occupied(o) => self.trielist[*o.get()].char_insert(iter),
                Entry::Vacant(v) => {
                    let num = self.trielist.len();
                    self.trielist.push(Trie::new());
                    v.insert(num);
                    self.trielist[num].char_insert(iter)
                }
            }
        } else {
            self.word = true;
        }
    }

    fn char_search(&self, mut iter: Chars, full: bool) -> bool {
        if let Some(ch) = iter.next() {
            if let Some(trie) = self.children.get(&ch) {
                self.trielist[*trie].char_search(iter, full)
            } else {
                false
            }
        } else if full && self.word {
            true
        } else {
            !full
        }
    }

    pub fn insert(&mut self, word: String) {
        self.char_insert(word.chars());
    }

    pub fn search(&self, word: String) -> bool {
        self.char_search(word.chars(), true)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.char_search(prefix.chars(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let mut trie = Trie::new();
        trie.insert("apple".into());
        assert!(trie.search("apple".into()));
        assert!(!trie.search("app".into()));
        assert!(trie.starts_with("app".into()));
        trie.insert("app".into());
        assert!(trie.search("app".into()));
    }
}

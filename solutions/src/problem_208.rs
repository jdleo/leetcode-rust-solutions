use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    is_word: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }

    fn insert(&mut self, word: String) {
        // get current pointer
        let mut current = self;

        // go through each character in word
        for chr in word.chars() {
            // traverse through all the trie nodes, creating if not existing
            current = current.children.entry(chr).or_insert(Trie::new());
        }

        // mark as end of word
        current.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        // get current pointer
        let mut current = self;

        // go through each character in word
        for chr in word.chars() {
            // check if doesnt exist
            if let Some(trie) = current.children.get(&chr) {
                // set next
                current = trie;
            } else {
                // nope
                return false;
            }
        }

        // check if end
        current.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        // get current pointer
        let mut current = self;

        // go through each character in prefix
        for chr in prefix.chars() {
            // check if doesnt exist
            if let Some(trie) = current.children.get(&chr) {
                // set next
                current = trie;
            } else {
                // nope
                return false;
            }
        }

        // just prefix, no need to check for full word
        true
    }
}

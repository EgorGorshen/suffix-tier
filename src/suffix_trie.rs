use crate::node::Node;
use std::collections::{HashSet};
use std::default::Default;
use std::fmt;
/// A SuffixTrie struct represents a suffix trie with a dictionary of words added to it,
/// and a main node acting as the root of the trie.
#[derive(Default, Debug)]
pub struct SuffixTrie {
    pub dictionary: Option<Vec<String>>,
    main_node: Option<Node>,
}

impl SuffixTrie {
    /// Creates a new `SuffixTrie` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds multiple words to the suffix trie. Each word is added to the dictionary
    /// and inserted into the trie by breaking it down into suffixes.
    pub fn add_words<T: AsRef<str>>(&mut self, words: Vec<T>) {
        for word in words {
            self.add_word(word.as_ref().to_string());
        }
    }

    /// Adds a single word to the suffix trie. The word is added to the dictionary
    /// and all its suffixes are inserted into the trie.
    pub fn add_word(&mut self, word: String) {
        self.dictionary
            .get_or_insert_with(Vec::new)
            .push(word.clone());
        self.main_node
            .get_or_insert_with(Node::new)
            .add_suffix(word);
    }

    /// Finds all suffixes in the trie that start with the given prefix.
    pub fn find_prefixes(&self, prefix: &str) -> Option<HashSet<String>> {
        self.main_node
            .as_ref()?
            .find_by_prefix(prefix)
            .map(|suffixes| suffixes.into_iter().collect())
    }
}

impl fmt::Display for SuffixTrie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(dict) = &self.dictionary {
            write!(f, "{words}", words = dict.join("\n"))
        } else {
            write!(f, "dictionary is empty")
        }
    }
}

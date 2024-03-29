use std::collections::HashMap;
use std::default::Default;

/// Represents a node in the suffix trie. Each node may contain children nodes
/// represented by a `HashMap` keyed by characters. A node is terminal if it marks the end of a word.
#[derive(Default, Debug)]
pub struct Node {
    pub children: HashMap<char, Node>,
    pub terminal: bool,
}

impl Node {
    /// Creates a new, empty `Node`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a suffix to the node. If the suffix is not empty, it recursively inserts
    /// each character into the trie, marking the last character's node as terminal.
    pub fn add_suffix(&mut self, suffix: String) {
        if let Some(first_char) = suffix.chars().next() {
            let remaining_suffix = suffix[1..].to_string();
            let child_node = self.children.entry(first_char).or_insert_with(Node::new);
            child_node.add_suffix(remaining_suffix);
        } else {
            self.terminal = true;
        }
    }

    /// Recursively searches for a node that matches the end of the given prefix.
    /// If such a node is found, it collects all suffixes from that node onwards.
    pub fn find_by_prefix(&self, prefix: &str) -> Option<Vec<String>> {
        if prefix.is_empty() {
            return Some(self.collect_suffixes(""));
        }

        if let Some(first_char) = prefix.chars().next() {
            if let Some(child) = self.children.get(&first_char) {
                return child.find_by_prefix(&prefix[1..]);
            }
        }

        None
    }

    /// Collects all suffixes branching from the current node.
    /// The `prefix` argument is the accumulated prefix leading up to this node.
    fn collect_suffixes(&self, prefix: &str) -> Vec<String> {
        let mut suffixes = Vec::new();
        if self.terminal {
            suffixes.push(prefix.to_string());
        }
        for (&ch, node) in &self.children {
            let new_prefix = format!("{}{}", prefix, ch);
            let mut child_suffixes = node.collect_suffixes(&new_prefix);
            suffixes.append(&mut child_suffixes);
        }
        suffixes
    }
}

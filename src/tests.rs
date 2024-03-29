use node::Node;
use suffix_trie::SuffixTrie;

use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
	use crate::suffix_trie::SuffixTrie;

    #[test]
    fn find_prefixes() {
        let dict = vec![
            "Huuu", "Helo", "Hello", "Helium",
            "Mem", "Memory", "Meme", "Memento",
            "Tempo", "Templar", "Temporary",
            "Demo", "Democracy", "Demonstration",
        ];
        let mut suf_trie = SuffixTrie::new();

        for word in &dict {
            suf_trie.add_word(word.to_string());
        }

        let expected_he = HashSet::from(["lo".to_string(), "lium".to_string(), "llo".to_string()]);
        let expected_me = HashSet::from(["m".to_string(), "me".to_string(), "mento".to_string(), "mory".to_string()]);
        let expected_temp = HashSet::from(["o".to_string(), "lar".to_string(), "orary".to_string()]);
        let expected_dem = HashSet::from(["o".to_string(), "onstration".to_string(), "ocracy".to_string()]);

        assert_eq!(suf_trie.find_prefixes("He"), Some(expected_he));
        assert_eq!(suf_trie.find_prefixes("Me"), Some(expected_me));
        assert_eq!(suf_trie.find_prefixes("Temp"), Some(expected_temp));
        assert_eq!(suf_trie.find_prefixes("Dem"), Some(expected_dem));

        assert_eq!(suf_trie.find_prefixes("Z"), None);

        let expected_h = HashSet::from([
            "uuu".to_string(), "elo".to_string(), "ello".to_string(), "elium".to_string()
        ]);
        assert_eq!(suf_trie.find_prefixes("H"), Some(expected_h));
    }
}


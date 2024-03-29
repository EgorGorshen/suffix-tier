# Suffix Trie Library

This Rust library provides a flexible and efficient implementation of a Suffix Trie data structure, ideal for applications involving string searches, autocomplete systems, and other text processing tasks. It supports adding words, searching by prefix, and collecting all suffixes or substrings that match a given prefix.

## Features

- Efficient string insertion and search
- Support for collecting all suffixes matching a given prefix
- Ideal for autocomplete systems, text processing, and pattern matching

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
suffix_trie = "0.1.0"
```

## Usage

Here's a quick example to get you started:

```rust
use suffix_trie::SuffixTrie;

let mut trie = SuffixTrie::new();
trie.add_suffix("hello".to_string());
trie.add_suffix("helium".to_string());

if let Some(suffixes) = trie.find_prefixes("he") {
    for suffix in suffixes {
        println!("{}", suffix);
    }
}
```

## Running Tests

To run tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, create issues, or suggest improvements.

## source links

- [Suffix tree wiki](https://en.wikipedia.org/wiki/Suffix_tree)

## TODO

- [ ] None

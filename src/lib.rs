use std::collections::HashMap;

/// An implementation of a Trie data structure.
/// A Trie is a tree-like data structure that stores strings
/// in a way that allows for efficient retrieval of strings.
#[derive(Debug)]
pub struct Trie {
    root: Node,
}

#[derive(Debug)]
struct Node {
    is_terminal: bool,
    children: HashMap<char, Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node {
                is_terminal: false,
                children: HashMap::new(),
            },
        }
    }

    /// Inserts a key into the Trie.
    pub fn insert(&mut self, key: &str) {
        let mut current = &mut self.root;
        for c in key.chars() {
            current = current.children.entry(c).or_insert_with(|| Node {
                is_terminal: false,
                children: HashMap::new(),
            });
        }
        current.is_terminal = true;
    }

    /// Removes a key from the Trie.
    pub fn remove(&mut self, key: &str) {
        let mut current = &mut self.root;
        for c in key.chars() {
            current = match current.children.get_mut(&c) {
                Some(node) => node,
                None => return,
            }
        }
        current.is_terminal = false;
    }

    /// Checks if the Trie contains a key.
    pub fn contains(&self, key: &str) -> bool {
        let mut current = &self.root;
        for c in key.chars() {
            current = match current.children.get(&c) {
                Some(node) => node,
                None => return false,
            };
        }
        current.is_terminal
    }

    /// Checks if the Trie contains a prefix.
    pub fn contains_prefix(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            current = match current.children.get(&c) {
                Some(node) => node,
                None => return false,
            };
        }
        true
    }
}
pub struct TrieIterator {
    trie: Trie,
    stack: Vec<String>,
}

impl Iterator for TrieIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl IntoIterator for Trie {
    type Item = String;
    type IntoIter = TrieIterator;

    fn into_iter(self) -> Self::IntoIter {
        TrieIterator {
            trie: self,
            stack: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");

        assert!(trie.contains("hello"));
        assert!(trie.contains("world"));
    }

    #[test]
    fn test_remove() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        trie.remove("hello");

        assert!(!trie.contains("hello"));
        assert!(trie.contains("world"));
    }

    #[test]
    fn test_contains_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");

        assert!(trie.contains_prefix("he"));
        assert!(trie.contains_prefix("wor"));
        assert!(trie.contains_prefix("h"));
        assert!(trie.contains_prefix("w"));
    }
}

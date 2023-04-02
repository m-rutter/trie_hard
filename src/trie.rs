use crate::trie_map::TrieMap;

/// An implementation of a Trie data structure.
/// A Trie is a tree-like data structure that stores strings
#[derive(Debug)]
pub struct Trie {
    /// The underlying TrieMap that stores the strings.
    /// The value of the TrieMap is a unit type, since we only care about the keys.
    map: TrieMap<()>,
}

impl Trie {
    /// Creates a new Trie.
    pub fn new() -> Self {
        Self {
            map: TrieMap::new(),
        }
    }

    /// Inserts a key into the Trie.
    pub fn insert(&mut self, key: &str) {
        self.map.insert(key, ());
    }

    /// Removes a key from the Trie.
    pub fn remove(&mut self, key: &str) {
        self.map.remove(key);
    }

    /// Checks if the Trie contains a key.
    pub fn contains(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    /// Checks if the Trie contains a prefix.
    pub fn contains_prefix(&self, prefix: &str) -> bool {
        self.map.contains_prefix(prefix)
    }

    /// Checks if the Trie is empty.
    pub fn is_empty(&self) -> bool {
        self.map.root.children.is_empty()
    }

    /// Clears the Trie.
    pub fn clear(&mut self) {
        self.map.root.children.clear();
    }

    /// Returns a vector of all keys in the Trie that start with the given prefix.
    pub fn suggest(&self, prefix: &str) -> Vec<String> {
        self.map.suggest(prefix)
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

    #[test]
    fn test_suggest() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        trie.insert("worldwide");
        trie.insert("worldwide web");
        trie.insert("worldwide web browser");
        trie.insert("worldwide web browser extension");

        assert_eq!(
            trie.suggest("world"),
            vec![
                "world",
                "worldwide",
                "worldwide web",
                "worldwide web browser",
                "worldwide web browser extension"
            ]
        );
        assert_eq!(
            trie.suggest("worldwide"),
            vec![
                "worldwide",
                "worldwide web",
                "worldwide web browser",
                "worldwide web browser extension"
            ]
        );
        assert_eq!(
            trie.suggest("worldwide web"),
            vec![
                "worldwide web",
                "worldwide web browser",
                "worldwide web browser extension"
            ]
        );
        assert_eq!(
            trie.suggest("worldwide web browser"),
            vec!["worldwide web browser", "worldwide web browser extension"]
        );
        assert_eq!(
            trie.suggest("worldwide web browser extension"),
            vec!["worldwide web browser extension"]
        );
    }
}

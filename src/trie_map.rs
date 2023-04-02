use std::collections::HashMap;

/// An implementation of a TrieMap data structure.
/// A TrieMap is a tree-like data structure that stores key-value pairs.
#[derive(Debug)]
pub struct TrieMap<T> {
    pub(crate) root: TrieMapNode<T>,
}

#[derive(Debug)]
pub struct TrieMapNode<T> {
    pub(crate) value: Option<T>,
    pub(crate) children: HashMap<char, TrieMapNode<T>>,
}

impl<T> TrieMapNode<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            children: HashMap::new(),
        }
    }
}

impl<T> TrieMap<T> {
    pub fn new() -> Self {
        Self {
            root: TrieMapNode::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let mut current = &mut self.root;
        for c in key.chars() {
            current = current
                .children
                .entry(c)
                .or_insert_with(|| TrieMapNode::new());
        }
        current.value = Some(value);
    }

    pub fn remove(&mut self, key: &str) {
        let mut current = &mut self.root;
        for c in key.chars() {
            current = match current.children.get_mut(&c) {
                Some(node) => node,
                None => return,
            };
        }
        current.value = None;
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let mut current = &self.root;
        for c in key.chars() {
            current = match current.children.get(&c) {
                Some(node) => node,
                None => return None,
            };
        }
        current.value.as_ref()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        let mut current = &self.root;
        for c in key.chars() {
            current = match current.children.get(&c) {
                Some(node) => node,
                None => return false,
            };
        }
        current.value.is_some()
    }

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

    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }

    fn suggest_helper(&self, prefix: &str, node: &TrieMapNode<T>, suggestions: &mut Vec<String>) {
        if let Some(_) = &node.value {
            suggestions.push(prefix.to_string());
        }
        for (c, child) in &node.children {
            let mut new_prefix = prefix.to_string();
            new_prefix.push(*c);
            self.suggest_helper(&new_prefix, child, suggestions);
        }
    }

    pub fn suggest(&self, prefix: &str) -> Vec<String> {
        let mut current = &self.root;
        for c in prefix.chars() {
            current = match current.children.get(&c) {
                Some(node) => node,
                None => return vec![],
            };
        }
        let mut suggestions = vec![];
        self.suggest_helper(prefix, current, &mut suggestions);
        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let trie: TrieMap<()> = TrieMap::new();
        assert_eq!(trie.root.value, None);
        assert_eq!(trie.root.children.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut trie = TrieMap::new();
        trie.insert("hello", 1);
        trie.insert("world", 2);
        trie.insert("world", 3);
        assert_eq!(trie.get("hello"), Some(&1));
        assert_eq!(trie.get("world"), Some(&3));
    }

    #[test]
    fn test_remove() {
        let mut trie = TrieMap::new();
        trie.insert("hello", 1);
        trie.insert("world", 2);
        trie.remove("hello");
        assert_eq!(trie.get("hello"), None);
        assert_eq!(trie.get("world"), Some(&2));
    }

    #[test]
    fn test_contains_key() {
        let mut trie = TrieMap::new();
        trie.insert("hello", 1);
        trie.insert("world", 2);
        assert!(trie.contains_key("hello"));
        assert!(trie.contains_key("world"));
        assert!(!trie.contains_key("foo"));
    }

    #[test]
    fn test_contains_prefix() {
        let mut trie = TrieMap::new();
        trie.insert("hello", 1);
        trie.insert("world", 2);
        assert!(trie.contains_prefix("he"));
        assert!(trie.contains_prefix("wo"));
        assert!(trie.contains_prefix("hello"));
        assert!(trie.contains_prefix("world"));
        assert!(!trie.contains_prefix("foo"));
    }

    #[test]
    fn test_empty() {
        let trie = TrieMap::<i32>::new();
        assert!(!trie.contains_key("hello"));
        assert!(!trie.contains_prefix("he"));
    }
}

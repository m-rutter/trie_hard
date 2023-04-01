use std::collections::HashMap;

/// An implementation of a TrieMap data structure.
/// A TrieMap is a tree-like data structure that stores key-value pairs.
pub struct TrieMap<T> {
    root: TrieMapNode<T>,
}

pub struct TrieMapNode<T> {
    value: Option<T>,
    children: HashMap<char, TrieMapNode<T>>,
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
}

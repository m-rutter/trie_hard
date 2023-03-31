#[derive(Debug)]
struct Trie<V> {
    root: Node<V>,
}

#[derive(Debug)]
struct Node<V> {
    key: String,
    value: Option<V>,
    children: Option<Vec<Node<V>>>,
}

impl<V> Trie<V> {
    fn new() -> Self {
        Self {
            root: Node {
                key: String::new(),
                value: None,
                children: None,
            },
        }
    }

    fn insert(&mut self, key: &str, value: V) {}

    fn get(&self, key: &str) -> Option<&V> {
        None
    }

    fn remove(&mut self, key: &str) -> Option<V> {
        None
    }

    fn contains(&self, key: &str) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn size(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_trie() {
        let trie = Trie::<i32>::new();
        assert_eq!(trie.size(), 0);
        assert!(trie.is_empty());
    }

    #[test]
    fn insert() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        assert_eq!(trie.size(), 1);
        assert!(!trie.is_empty());
    }

    #[test]
    fn get() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        assert_eq!(trie.get("hello"), Some(&1));
    }

    #[test]
    fn remove() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        assert_eq!(trie.remove("hello"), Some(1));
        assert_eq!(trie.size(), 0);
        assert!(trie.is_empty());
    }

    #[test]
    fn contains() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        assert!(trie.contains("hello"));
    }

    #[test]
    fn is_empty() {
        let mut trie = Trie::<i32>::new();
        assert_eq!(trie.is_empty(), true);
        trie.insert("hello", 1);
        assert_eq!(trie.is_empty(), false);
    }

    #[test]
    fn size() {
        let mut trie = Trie::<i32>::new();
        assert_eq!(trie.size(), 0);
        trie.insert("hello", 1);
        assert_eq!(trie.size(), 1);
    }

    #[test]
    fn insert_multiple() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        trie.insert("hello_there", 2);
        trie.insert("hello_there_you", 3);
        assert_eq!(trie.size(), 3);
        assert_eq!(trie.get("hello"), Some(&1));
        assert_eq!(trie.get("hello_there"), Some(&2));
        assert_eq!(trie.get("hello_there_you"), Some(&3));
    }

    #[test]
    fn remove_multiple() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        trie.insert("hello_there", 2);
        trie.insert("hello_there_you", 3);
        assert_eq!(trie.size(), 3);
        assert_eq!(trie.remove("hello"), Some(1));
        assert_eq!(trie.remove("hello_there"), Some(2));
        assert_eq!(trie.remove("hello_there_you"), Some(3));
        assert_eq!(trie.size(), 0);
        assert!(trie.is_empty());
    }

    #[test]
    fn contains_multiple() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        trie.insert("hello_there", 2);
        trie.insert("hello_there_you", 3);
        assert!(trie.contains("hello"));
        assert!(trie.contains("hello_there"));
        assert!(trie.contains("hello_there_you"));
    }

    #[test]
    fn get_multiple() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        trie.insert("hello_there", 2);
        trie.insert("hello_there_you", 3);
        assert_eq!(trie.get("hello"), Some(&1));
        assert_eq!(trie.get("hello_there"), Some(&2));
        assert_eq!(trie.get("hello_there_you"), Some(&3));
    }

    #[test]
    fn insert_remove() {
        let mut trie = Trie::<i32>::new();
        trie.insert("hello", 1);
        trie.insert("hello_there", 2);
        trie.insert("hello_there_you", 3);
        assert_eq!(trie.size(), 3);
        assert_eq!(trie.remove("hello"), Some(1));
        assert_eq!(trie.remove("hello_there"), Some(2));
        assert_eq!(trie.remove("hello_there_you"), Some(3));
        assert_eq!(trie.size(), 0);
        assert!(trie.is_empty());
    }
}

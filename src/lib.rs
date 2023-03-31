struct Trie<K, V>
where
    K: ToString,
{
    root: Node<K, V>,
}

struct Node<K, V>
where
    K: ToString,
{
    key: K,
    value: Option<V>,
    children: Option<Vec<Node<K, V>>>,
}

impl<K, V> Trie<K, V>
where
    K: ToString,
{
    fn new() -> Self {
        Self {
            root: Node { value: None },
        }
    }

    fn insert(&mut self, key: K, value: V) {}

    fn get(&self, key: K) -> Option<&V> {
        None
    }

    fn remove(&mut self, key: K) -> Option<V> {
        None
    }

    fn contains(&self, key: K) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        false
    }
}

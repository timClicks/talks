struct ArrayMap<K, V> {
    inner: Vec<(K, V)>,
}

impl ArrayMap<K, V> {
    fn new() -> Self {
        ArrayMap {
            inner: vec![],
        }
    }
}

impl<K: Copy + PartialEq, V: Copy> ArrayMap {
    fn insert(&mut self, key: K, value: V) {
        self.inner.push((key, value));
    }

    fn get(&self, key: &K) -> Option<&V> {
        for (key_for_val, val) in &self.inner {
            if key == key_for_val {
                return Some(val);
            }
        }

        None
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (key_for_val, val) in &mut self.inner {
            if key == key_for_val {
                return Some(val);
            }
        }

        None
    }
}

// TODO: add Ord constraint to K and then use binary search on lookup

fn main() {
    let counts = ArrayMap::new();
    counts.insert('%', 3);
    println!("Hello, world!");
}

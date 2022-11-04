use std::ops::Index;

/// A workaround for creating HashMaps with order
#[derive(Clone)]
pub struct SortedHashMap<K: PartialEq + Clone, V: Clone> {
    collection: Vec<(K, V)>,
}

impl<K: PartialEq + Clone, V: Clone> SortedHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            collection: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.collection.push((key, value))
    }

    pub fn contains_key(&self, target_key: K) -> bool {
        for (key, _) in &self.collection {
            if *key == target_key {
                return true;
            }
        }
        false
    }

    pub fn get(&self, target_key: &K) -> Option<&V> {
        for (key, value) in &self.collection {
            if key == target_key {
                return Some(value)
            }
        }
        None
    }
}

// Into iterator
impl<'a, K: PartialEq + Clone, V: Clone> IntoIterator for &'a SortedHashMap<K, V> {
    type Item = &'a (K, V);
    type IntoIter = SortedHashMapIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        SortedHashMapIterator {
            count: 0,
            shm: &self,
        }
    }
}

impl<K: PartialEq + Clone, V: Clone> Index<K> for SortedHashMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        for (key, value) in self {
            if *key == index {
                return value
            }
        }
        panic!("Cannot index with index in the sorted hashmap.")
    }
}

pub struct SortedHashMapIterator<'a, K: PartialEq + Clone, V: Clone> {
    count: usize,
    shm: &'a SortedHashMap<K, V>,
}

impl<'a, K: PartialEq + Clone, V: Clone> Iterator for SortedHashMapIterator<'a, K, V> {
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let something = self.shm.collection.get(self.count);
        self.count += 1;

        something
    }
}


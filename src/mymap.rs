use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher}
};

static MAP_CAPACITY: usize = 16;

#[derive(Debug)]
struct Entry<K, V> {
    hash: u64,
    key: K,
    value: V
}

#[derive(Debug)]
pub struct MyMap<K, V> {
    entries: Vec<Vec<Entry<K, V>>>
}

impl<K: Eq + Hash, V> MyMap<K, V> {
    pub fn new() -> Self {
        let mut entries: Vec<Vec<Entry<K, V>>> = Vec::with_capacity(MAP_CAPACITY);

        for _ in 0..MAP_CAPACITY {
            entries.push(Vec::new());
        }

        Self { entries }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let (hash, idx) = hash_and_index(&key);

        let target = &mut self.entries[idx];
        
        let new_entry = Entry { hash, key, value };

        match target.into_iter().find(|entry| entry.hash == hash) {
            Some(entry) => *entry = new_entry,
            None => target.push(new_entry),
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let (hash, idx) = hash_and_index(&key);
        
        let target = &mut self.entries[idx];

        let search_result = target.iter().enumerate()
            .find(|(_idx, entry)| entry.hash == hash);
        
        if let Some((idx, _entry)) = search_result {
            Some(target.remove(idx).value)
        } else {
            None
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let (hash, idx) = hash_and_index(&key);

        let target = &self.entries[idx];

        target.into_iter().find_map(|e| {
            if e.hash == hash {
                Some(&e.value)
            } else {
                None
            }
        })
    }
}

fn hash_and_index<V: Hash>(hashee: &V) -> (u64, usize) {
    let mut hasher = DefaultHasher::new();
    hashee.hash(&mut hasher);
    let hash = hasher.finish();
    let idx = (hash % MAP_CAPACITY as u64) as usize;
    (hash, idx)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut my_map: MyMap<i32, &str> = MyMap::new();
        assert!(my_map.get(&42).is_none());

        my_map.insert(42, "good number");
        assert!(my_map.get(&42) == Some(&"good number"));

        my_map.insert(42, "another good number");
        assert!(my_map.get(&42) == Some(&"another good number"));

        my_map.remove(&42);
        assert!(my_map.get(&42).is_none());
    }
}
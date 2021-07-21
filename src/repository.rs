use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::MyMap;

#[derive(Debug, Clone)]
struct Snapshot<K, V> {
    hash: u64,
    map: MyMap<K, V>
}

impl<K: Eq + Hash, V: Hash> Snapshot<K, V> {
    fn new(map: MyMap<K, V>) -> Self {
        let hash = hash(&map);
        Self { hash, map }
    }
}

#[derive(Debug)]
pub struct Repository<K, V> {
    repository: Vec<Snapshot<K, V>>,
    head: u64
}

impl<K: Hash + Eq, V: Hash> Repository<K, V> {
    pub fn init_empty() -> Self {
        Repository {
            repository: Vec::new(),
            head: 0
        }
    }

    pub fn head(&self) -> &MyMap<K, V> {
        &self.repository.iter()
            .find(|snap| snap.hash == self.head)
            .unwrap()
            .map
    }

    pub fn repository_size(&self) -> usize {
        self.repository.len()
    }

    pub fn checkpoint(&mut self, map: MyMap<K, V>) -> u64 {
        let hash = hash(&map);
        self.repository.push(Snapshot { hash, map });
        self.head = hash;
        self.head
    }

    pub fn rollback(&mut self, head: u64) -> bool {
        match self.repository.iter().any(|snap| snap.hash == head) {
            true => {
                self.head = head;
                true
            },
            false => false,
        }
    }

    pub fn prune(&mut self) {
        let head = self.head;
        self.repository.retain(|snap| snap.hash == head);
    }
}

fn hash<V: Hash>(hashee: &V) -> u64 {
    let mut hasher = DefaultHasher::new();
    hashee.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_repository() {
        let mut repo = Repository::init_empty();

        assert!(repo.repository_size() == 0);

        let snap1 = {
            let mut map = MyMap::new();
            map.insert(42, "foo");
            map
        };

        let snap_hash1 = repo.checkpoint(snap1.clone());

        assert!(repo.repository_size() == 1);
        assert!(repo.head() == &snap1);

        let snap2 = {
            let mut map = MyMap::new();
            map.insert(111, "bar");
            map
        };

        repo.checkpoint(snap2.clone());

        assert!(repo.repository_size() == 2);
        assert!(repo.head() == &snap2);

        repo.rollback(snap_hash1);

        assert!(repo.repository_size() == 2);
        assert!(repo.head() == &snap1);

        repo.prune();

        assert!(repo.repository_size() == 1);
        assert!(repo.head() == &snap1);
    }
}
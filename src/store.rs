pub mod store {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::{Mutex, OnceLock};

    static DB_INSTANCE: OnceLock<Mutex<KeyValueStore<String, String>>> = OnceLock::new();
    
    pub fn get_store() -> &'static Mutex<KeyValueStore<String, String>> {
        DB_INSTANCE.get_or_init(|| Mutex::new(KeyValueStore::new()))
    }

    pub trait Store<K, V>
    where
        K: Eq + Hash,
        V: Clone,
    {
        fn set(&mut self, key: K, value: V);
        fn get(&self, key: &K) -> Option<V>;
        fn remove(&mut self, key: K);
    }

    pub struct KeyValueStore<K, V>
    where
        K: Eq + Hash,
        V: Clone,
    {
        store: HashMap<K, V>,
    }

    impl<K: Eq + Hash, V: Clone> KeyValueStore<K, V> {
        pub fn new() -> Self {
            Self { store: HashMap::new() }
        }
    }

    impl<K: Eq + Hash, V: Clone> Store<K, V> for KeyValueStore<K, V> {
        fn set(&mut self, key: K, value: V) -> () {
            self.store.insert(key, value);
        }

        fn get(&self, key: &K) -> Option<V> {
            self.store.get(&key).cloned()
        }

        fn remove(&mut self, key: K) -> () {
            self.store.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::store::store::{get_store, Store};

    #[test]
    fn test_set() {
        let store = get_store();
        let mut store = store.lock().unwrap();
        
        store.set("name".to_string(), "John".to_string());
        assert_eq!(store.get(&"name".to_string()), Some("John".to_string()));
        
        store.set("name".to_string(), "Sam".to_string());
        assert_eq!(store.get(&"name".to_string()), Some("Sam".to_string()));
    }

    #[test]
    fn test_get() {
        let store = get_store();
        let mut store = store.lock().unwrap();
        
        store.set("name".to_string(), "John".to_string());
        store.set("age".to_string(), "20".to_string());

        assert_eq!(store.get(&"name".to_string()), Some("John".to_string()));
        assert_eq!(store.get(&"age".to_string()), Some("20".to_string()));
        
        assert_eq!(store.get(&"nonExisting".to_string()), None);
    }

    #[test]
    fn test_remove() {
        let store = get_store();
        let mut store = store.lock().unwrap();
        
        store.set("name".to_string(), "John".to_string());
        store.set("age".to_string(), "20".to_string());

        store.remove("name".to_string());
        assert_eq!(store.get(&"name".to_string()), None);

        store.remove("nonExisting".to_string());
        assert_eq!(store.get(&"nonExisting".to_string()), None);
    }
}
pub mod store {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::{Mutex, OnceLock};

    static DB_INSTANCE: OnceLock<Mutex<KeyValueStore<String, String>>> = OnceLock::new();
    
    pub fn getStore() -> &'static Mutex<KeyValueStore<String, String>> {
        // TODO: Handle unsafe unwrapping with unwrap()
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
    use crate::store::store::{getStore, Store};

    #[test]
    fn test_set() {
        let store = getStore();
        let mut store = store.lock().unwrap();
        
        store.set(String::from("name"), String::from("John"));
        assert_eq!(store.get(&String::from("name")), Some(String::from("John")));
        
        store.set(String::from("name"), String::from("Sam"));
        assert_eq!(store.get(&String::from("name")), Some(String::from("Sam")));
    }

    #[test]
    fn test_get() {
        let store = getStore();
        let mut store = store.lock().unwrap();
        
        store.set(String::from("name"), String::from("John"));
        store.set(String::from("age"), String::from("20"));

        assert_eq!(store.get(&String::from("name")), Some(String::from("John")));
        assert_eq!(store.get(&String::from("age")), Some(String::from("20")));
        
        assert_eq!(store.get(&String::from("nonExisting")), None);
    }

    #[test]
    fn test_remove() {
        let store = getStore();
        let mut store = store.lock().unwrap();
        
        store.set(String::from("name"), String::from("John"));
        store.set(String::from("age"), String::from("20"));

        store.remove(String::from("name"));
        assert_eq!(store.get(&String::from("name")), None);

        store.remove(String::from("nonExisting"));
        assert_eq!(store.get(&String::from("nonExisting")), None);
    }
}
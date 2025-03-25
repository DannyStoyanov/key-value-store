pub mod store {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{Error, Read};

    pub fn getStore() -> KeyValueStore {
        KeyValueStore::new()
    }

    pub trait Store {
        fn set(&mut self, key: String, value: String);
        fn get(&self, key: String) -> Option<String>;
        fn remove(&mut self, key: String);
        fn saveToFile(&self, filename: &str) -> Result<(), Error>;
        fn loadFromFile(filename: &str) -> Result<KeyValueStore, Error>;
    }

    pub struct KeyValueStore {
        store: HashMap<String, String>,
    }

    impl KeyValueStore {
        fn new() -> Self {
            Self {
                store: HashMap::new(),
            }
        }
    }

    impl Store for KeyValueStore {
        fn set(&mut self, key: String, value: String) -> () {
            self.store.insert(key, value);
        }

        fn get(&self, key: String) -> Option<String> {
            self.store.get(&key).cloned()
        }

        fn remove(&mut self, key: String) -> () {
            self.store.remove(&key);
        }

        fn saveToFile(&self, filename: &str) -> Result<(), Error> {
            let json = serde_json::to_string(&self.store)?;
            fs::write(filename, json)
        }

        fn loadFromFile(filename: &str) -> Result<KeyValueStore, Error> {
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let store: HashMap<String, String> = serde_json::from_str(&content)?;
            Ok(KeyValueStore { store })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::store::store::{getStore, Store};

    #[test]
    fn test_set() {
        let mut store = getStore();

        store.set(String::from("name"), String::from("John"));
        assert_eq!(store.get(String::from("name")), Some(String::from("John")));

        store.set(String::from("name"), String::from("Sam"));
        assert_eq!(store.get(String::from("name")), Some(String::from("Sam")));
    }

    #[test]
    fn test_get() {
        let mut store = getStore();

        store.set(String::from("name"), String::from("John"));
        store.set(String::from("age"), String::from("20"));

        assert_eq!(store.get(String::from("name")), Some(String::from("John")));
        assert_eq!(store.get(String::from("age")), Some(String::from("20")));

        assert_eq!(store.get(String::from("nonExisting")), None);
    }

    #[test]
    fn test_remove() {
        let mut store = getStore();

        store.set(String::from("name"), String::from("John"));
        store.set(String::from("age"), String::from("20"));

        store.remove(String::from("name"));
        assert_eq!(store.get(String::from("name")), None);

        store.remove(String::from("nonExisting"));
        assert_eq!(store.get(String::from("nonExisting")), None);
    }
}

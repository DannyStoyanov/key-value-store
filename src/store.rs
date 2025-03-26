pub mod store {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{Error, Read};

    use serde_json::Value;

    pub fn get_store() -> KeyValueStore {
        KeyValueStore::new()
    }

    pub trait Store {
        fn set(&mut self, key: String, value: Value);
        fn get(&self, key: String) -> Option<Value>;
        fn remove(&mut self, key: String);
        fn save_to_file(&self, filename: &str) -> Result<(), Error>; // TODO: add file format option
        fn load_from_file(filename: &str) -> Result<KeyValueStore, Error>;
    }

    pub struct KeyValueStore {
        store: HashMap<String, Value>,
    }

    impl KeyValueStore {
        fn new() -> Self {
            Self {
                store: HashMap::<String, Value>::new(),
            }
        }
    }

    impl Store for KeyValueStore {
        fn set(&mut self, key: String, value: Value) -> () {
            self.store.insert(key, value);
        }

        fn get(&self, key: String) -> Option<Value> {
            self.store.get(&key).cloned()
        }

        fn remove(&mut self, key: String) -> () {
            self.store.remove(&key);
        }

        fn save_to_file(&self, filename: &str) -> Result<(), Error> {
            let json = serde_json::to_string(&self.store)?;
            fs::write(filename, json)
        }

        fn load_from_file(filename: &str) -> Result<KeyValueStore, Error> {
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let store: HashMap<String, Value> = serde_json::from_str(&content)?;
            Ok(KeyValueStore { store })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::store::store::{get_store, KeyValueStore, Store};

    const FILENAME: &str = "test-file.json";

    #[test]
    fn test_set() {
        let mut store = get_store();

        store.set("name".into(), "John".into());
        assert_eq!(store.get("name".into()), Some("John".into()));

        store.set("name".into(), "Sam".into());
        assert_eq!(store.get("name".into()), Some("Sam".into()));
    }

    #[test]
    fn test_get() {
        let mut store = get_store();

        store.set("name".into(), "John".into());
        store.set("age".into(), 25.into());

        assert_eq!(store.get("name".into()), Some("John".into()));
        assert_eq!(store.get("age".into()), Some(25.into()));

        assert_eq!(store.get("nonExisting".into()), None);
    }

    #[test]
    fn test_remove() {
        let mut store = get_store();

        store.set("name".into(), "John".into());
        store.set("age".into(), 25.into());

        store.remove("name".into());
        assert_eq!(store.get("name".into()), None);

        store.remove("nonExisting".into());
        assert_eq!(store.get("nonExisting".into()), None);
    }

    #[test]
    fn test_save_and_load() {
        let mut store = get_store();

        store.set("key".into(), "value".into());

        assert!(store.save_to_file(FILENAME).is_ok());

        let store = KeyValueStore::load_from_file(FILENAME);

        assert!(store.is_ok());

        fs::remove_file(FILENAME).expect("Failed to delete test file");
    }

    #[test]
    fn test_load_from_nonexisting_file() {
        let store = KeyValueStore::load_from_file("nonexisting.json");
        assert!(store.is_err(), "Expected error for nonexistent file, but got success");
    }
}

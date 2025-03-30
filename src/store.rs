pub mod store {

    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{Error, Read};

    use csv::{Reader, Writer};
    use serde_json::Value;

    use crate::fileformat::fileformat::FileFormat;

    pub fn get_store() -> KeyValueStore {
        KeyValueStore::new()
    }

    pub trait Store {
        fn set(&mut self, key: String, value: Value);
        fn get(&self, key: String) -> Option<Value>;
        fn remove(&mut self, key: &String);
        fn save_to_file(&self, filename: &str, format: &FileFormat) -> Result<(), Error>;
        fn load_from_file(filename: &str, format: &FileFormat) -> Result<KeyValueStore, Error>;
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

        fn remove(&mut self, key: &String) -> () {
            self.store.remove(key);
        }

        fn save_to_file(&self, filename: &str, format: &FileFormat) -> Result<(), Error> {
            match format {
                FileFormat::JSON => {
                    let content = serde_json::to_string(&self.store)?;
                    fs::write(format!("{}.{}", filename, format), content)
                }
                FileFormat::CSV => {
                    let file = File::create(format!("{}.{}", filename, format))?;
                    let mut wtr = Writer::from_writer(file);
                    wtr.write_record(&["Key", "Value"])?;

                    for (key, value) in &self.store {
                        let value_str = match value {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            Value::Array(a) => {
                                serde_json::to_string(a).unwrap_or_else(|_| "[]".to_string())
                            }
                            Value::Object(o) => {
                                serde_json::to_string(o).unwrap_or_else(|_| "{}".to_string())
                            }
                            Value::Null => "null".to_string(),
                        };
                        wtr.write_record(&[key, &value_str])?;
                    }

                    Ok(())
                }
            }
        }

        fn load_from_file(filename: &str, format: &FileFormat) -> Result<KeyValueStore, Error> {
            let mut store = HashMap::<String, Value>::new();

            match format {
                FileFormat::JSON => {
                    let mut file = File::open(format!("{}.{}", filename, format))?;
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;
                    store = serde_json::from_str(&content)?;
                }
                FileFormat::CSV => {
                    let file = File::open(format!("{}.{}", filename, format))?;
                    let mut rdr = Reader::from_reader(file);

                    let mut store = HashMap::<String, Value>::new();
                    for result in rdr.records() {
                        let record = result?;
                        if record.len() >= 2 {
                            let key = record[0].to_string();
                            let value = record[1].to_string();
                            store.insert(key, value.into());
                        }
                    }
                }
            }
            Ok(KeyValueStore { store })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::{self},
        fs::{self},
    };

    use rstest::rstest;
    use serde_json::Value;

    use crate::{
        fileformat::fileformat::FileFormat,
        store::store::{get_store, KeyValueStore, Store},
    };

    const FILENAME: &str = "test-file";

    #[derive(Clone)]
    struct Address<'a> {
        city: &'a str,
        street: &'a str,
        street_number: u32,
    }

    impl fmt::Display for Address<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {} {})", self.city, self.street, self.street_number)
        }
    }

    const TEST_ADDRESS: Address<'_> = Address {
        city: "NYC",
        street: "Karlston",
        street_number: 12_u32,
    };

    fn setup_store(store: &mut KeyValueStore) {
        store.set("name".into(), "John".into());
        store.set("age".into(), 26.into());
        store.set("married".into(), true.into());
        store.set("citizenships".into(), ["American", "Swiss"].into());
        store.set("address".into(), TEST_ADDRESS.clone().to_string().into()); // TODO: fix to persist the object itself, not its display value(aka String)
        store.set("job_occupation".into(), Value::Null);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = get_store();
        setup_store(&mut store);

        // String
        assert_eq!(store.get("name".into()), Some("John".into()));
        // Number
        assert_eq!(store.get("age".into()), Some(26.into()));
        // Bool
        assert_eq!(store.get("married".into()), Some(true.into()));
        // Array
        assert_eq!(
            store.get("citizenships".into()),
            Some(["American", "Swiss"].into())
        );
        // Object
        assert_eq!(
            store.get("address".into()),
            Some(TEST_ADDRESS.to_string().into())
        );
        // Null
        assert_eq!(store.get("job_occupation".into()), Some(Value::Null));
    }

    #[test]
    fn test_remove() {
        let mut store = get_store();
        setup_store(&mut store);

        let keys: [&str; 6] = ["name", "age", "married", "citizenships", "address", "job_occupation"];

        for key in keys {
            store.remove(&key.to_string().into());
            assert_eq!(store.get(key.into()), None);
        }
    }

    #[rstest]
    #[case(FileFormat::JSON)]
    #[case(FileFormat::CSV)]
    fn test_save_and_load(#[case] file_format: FileFormat) {
        let mut store = get_store();
        setup_store(&mut store);

        assert!(store.save_to_file(FILENAME, &file_format).is_ok());

        let store = KeyValueStore::load_from_file(FILENAME, &file_format);

        assert!(store.is_ok());

        fs::remove_file(format!("{}.{}", FILENAME, file_format))
            .expect("Failed to delete test file");
    }

    #[rstest]
    #[case(FileFormat::JSON)]
    #[case(FileFormat::CSV)]
    fn test_load_from_nonexisting_file(#[case] file_format: FileFormat) {
        let store = KeyValueStore::load_from_file("nonexisting", &file_format);
        assert!(
            store.is_err(),
            "Expected error for nonexistent file, but got success"
        );
    }
}

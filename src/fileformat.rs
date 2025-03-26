pub mod fileformat {
    use core::fmt;

    pub enum FileFormat {
        JSON,
        CSV,
    }

    impl fmt::Display for FileFormat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                FileFormat::JSON => write!(f, "json"),
                FileFormat::CSV => write!(f, "csv"),
            }
        }
    }
}

use std::fs::File;
use std::io::{BufReader, BufWriter, Error as IoError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

pub struct FileStorage {
    pub path: String
}

impl FileStorage {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn save<T: Serialize>(&self, item: &T) -> Result<(), IoError> {
        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, item)?;
        Ok(())
    }

    pub fn load<T: DeserializeOwned>(&self) -> Result<T, IoError> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }
}
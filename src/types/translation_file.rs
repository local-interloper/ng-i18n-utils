use std::collections::BTreeMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TranslationFile {
    locale: String,
    translations: BTreeMap<String, String>,
}

impl TranslationFile {
    pub fn extend_with(&mut self, other: &TranslationFile) {
        for (key, text) in &other.translations {
            if self.translations.contains_key(key) {
                continue;
            }

            self.translations.insert(key.clone(), text.clone());
        }
    }

    pub fn try_from_path(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;

        Ok(serde_json::from_reader(BufReader::new(file))?)
    }

    pub fn write(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)?;

        serde_json::to_writer_pretty(BufWriter::new(file), self)?;

        Ok(())
    }
}

impl Default for TranslationFile {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            translations: BTreeMap::new(),
        }
    }
}
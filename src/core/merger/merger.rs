use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::types::translation_file::TranslationFile;

pub struct MergerConfig {
    pub source_path: String,
    pub target_languages: Vec<String>,
    pub no_sort: bool,
}

#[derive(Default)]
pub struct Merger {
    source_path: PathBuf,
    translation_file_paths: Vec<PathBuf>,
    no_sort: bool,
}

impl Merger {
    pub fn new(config: MergerConfig) -> Result<Self, Box<dyn Error>> {
        let mut translation_files = Vec::new();

        for language in &config.target_languages {
            let mut path = config.source_path.clone();

            let Some(index) = config.source_path.rfind('.') else {
                continue;
            };

            path.insert_str(index, &format!(".{language}"));

            translation_files.push(PathBuf::from(path));
        }

        Ok(Merger {
            source_path: PathBuf::from(config.source_path),
            translation_file_paths: translation_files,
            no_sort: config.no_sort,
        })
    }

    pub fn merge(&mut self) -> Result<(), Box<dyn Error>> {
        let source_file = OpenOptions::new()
            .read(true)
            .open(&self.source_path)?;

        let mut source: TranslationFile = serde_json::from_reader(BufReader::new(source_file))?;

        if !self.no_sort {
            source.write(&self.source_path)?;
        }

        for path in &self.translation_file_paths {
            let mut translation_file = TranslationFile::try_from_path(path)
                .unwrap_or(TranslationFile::default());

            translation_file.extend_with(&source);

            translation_file.write(path)?
        }

        Ok(())
    }
}
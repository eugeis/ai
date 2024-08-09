use std::{fs, io};
use std::path::PathBuf;
use crate::db::storage::Storage;

#[derive(Debug)]
pub struct Patterns {
    storage: Storage,
    system_pattern_file: String,
    unique_patterns_file_path: PathBuf,
}

impl Patterns {
    pub fn new(storage: Storage, system_pattern_file: &str, unique_patterns_file_path: PathBuf) -> Self {
        Patterns {
            storage,
            system_pattern_file: system_pattern_file.to_string(),
            unique_patterns_file_path: PathBuf::from(unique_patterns_file_path),
        }
    }
    pub fn configure(&self) -> Result<(), io::Error> {
        return self.storage.configure()
    }

    pub fn get_pattern(&self, name: &str) -> Result<Pattern, io::Error> {
        let pattern_path = self.storage.dir.join(name).join(&self.system_pattern_file);
        let content = fs::read(&pattern_path)?;
        let pattern_str = String::from_utf8(content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?; // Convert FromUtf8Error to io::Error
        Ok(Pattern {
            name: name.to_string(),
            description: "".to_string(),
            pattern: pattern_str,
        })
    }

    pub fn print_latest_patterns(&self, latest_number: usize) -> Result<(), io::Error> {
        let contents = fs::read_to_string(&self.unique_patterns_file_path)?;
        let unique_patterns: Vec<&str> = contents.split('\n').collect();
        let latest_number = std::cmp::min(latest_number, unique_patterns.len());
        for line in unique_patterns.iter().rev().take(latest_number) {
            println!("{}", line);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub pattern: String,
}

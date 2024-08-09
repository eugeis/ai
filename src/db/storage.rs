use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use crate::db::sessions::Message;

#[derive(Debug)]
pub struct Storage {
    pub label: String,
    pub dir: PathBuf,
    pub is_dir: bool,
    pub extension: Option<String>,
}

impl Storage {
    pub fn configure(&self) -> Result<(), io::Error> {
        fs::create_dir_all(&self.dir)?;
        Ok(())
    }

    pub fn get_names(&self) -> Result<Vec<String>, io::Error> {
        let mut entries = fs::read_dir(&self.dir)?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?; // Get metadata
            if self.is_dir && metadata.is_dir() {
                names.push(entry.file_name().into_string().unwrap());
            } else if !self.is_dir {
                if let Some(ext) = &self.extension {
                    if !metadata.is_dir() && *entry.path().extension().unwrap_or_default() == **ext {
                        names.push(entry.file_name().into_string().unwrap());
                    }
                } else {
                    if !metadata.is_dir() {
                        names.push(entry.file_name().into_string().unwrap());
                    }
                }
            }
        }
        Ok(names)
    }

    pub fn list_names(&self) -> Result<(), io::Error> {
        let names = self.get_names()?;
        if names.is_empty() {
            println!("No {}", self.label);
        } else {
            println!("\n{}:", self.label);
            for name in names {
                println!("\t{}", name);
            }
        }
        Ok(())
    }

    pub fn build_file_path(&self, name: &str) -> PathBuf {
        self.dir.join(self.build_file_name(name))
    }

    fn build_file_name(&self, name: &str) -> String {
        if let Some(ext) = &self.extension {
            format!("{}{}", name, ext)
        } else {
            name.to_string()
        }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.build_file_path(name).exists()
    }

    pub fn delete(&self, name: &str) -> Result<(), io::Error> {
        fs::remove_file(self.build_file_path(name))
    }

    pub fn rename(&self, old_name: &str, new_name: &str) -> Result<(), io::Error> {
        fs::rename(self.build_file_path(old_name), self.build_file_path(new_name))
    }

    pub fn save(&self, name: &str, content: &[u8]) -> Result<(), io::Error> {
        let mut file = File::create(self.build_file_path(name))?;
        file.write_all(content)?;
        Ok(())
    }

    pub fn load(&self, name: &str) -> Result<Vec<u8>, io::Error> {
        let mut content = Vec::new();
        let mut file = File::open(self.build_file_path(name))?;
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    pub fn save_as_json<T: serde::Serialize>(&self, name: &str, item: &T) -> Result<(), io::Error> {
        let content = serde_json::to_string(item)?;
        self.save(name, content.as_bytes())
    }

    pub fn load_as_json<T: serde::de::DeserializeOwned>(&self, name: &str, x: &mut Vec<Message>) -> Result<T, io::Error> {
        let content = self.load(name)?;
        let item = serde_json::from_slice(&content)?;
        Ok(item)
    }
}

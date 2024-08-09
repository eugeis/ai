use std::fs;
use std::io;
use std::path::{PathBuf};
use crate::db::contexts::Contexts;
use crate::db::sessions::Sessions;
use crate::db::patterns::Patterns;
use crate::db::storage::Storage;

#[derive(Debug)]
pub struct Db {
    dir: PathBuf,
    patterns: Patterns,
    sessions: Sessions,
    contexts: Contexts,
    env_file_path: PathBuf,
}

impl Db {
    pub fn new(dir: &str) -> Result<Self, io::Error> {
        let dir_path = PathBuf::from(dir);
        let env_file_path = dir_path.join(".env");

        let patterns = Patterns::new(
            Storage {
                label: "Patterns".to_string(),
                dir: dir_path.join("patterns"),
                is_dir: true,
                extension: None,
            },
            "system.md",
            Default::default()
        )?;

        let sessions = Sessions::new(
            Storage {
                label: "Sessions".to_string(),
                dir: dir_path.join("sessions"),
                is_dir: false,
                extension: Some(".json".to_string()),
            }
        )?;

        let contexts = Contexts::new(
            Storage {
                label: "Contexts".to_string(),
                dir: dir_path.join("contexts"),
                ..Default::default()
            }
        )?;

        Ok(Db {
            dir: dir_path,
            patterns,
            sessions,
            contexts,
            env_file_path,
        })
    }

    pub fn configure(&self) -> Result<(), io::Error> {
        fs::create_dir_all(&self.dir)?;
        self.load_env_file()?;
        self.patterns.configure()?;
        self.sessions.configure()?;
        self.contexts.configure()?;
        Ok(())
    }

    pub fn is_env_file_exists(&self) -> bool {
        self.env_file_path.exists()
    }

    pub fn save_env(&self, content: &str) -> Result<(), io::Error> {
        fs::write(&self.env_file_path, content.as_bytes())?;
        Ok(())
    }

    fn file_path(&self, file_name: &str) -> PathBuf {
        self.dir.join(file_name)
    }
}
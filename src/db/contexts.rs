use std::io;
use crate::db::storage::Storage;

#[derive(Debug)]
pub struct Contexts {
    storage: Storage,
}

impl Contexts {
    pub fn new(storage: Storage) -> Self {
        Contexts { storage }
    }

    pub fn configure(&self) -> Result<(), io::Error> {
        return self.storage.configure()
    }

    pub fn get_context(&self, name: &str) -> Result<Context, io::Error> {
        let content = self.storage.load(name)?;
        let content_str = String::from_utf8(content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?; // Convert FromUtf8Error to io::Error
        Ok(Context {
            name: name.to_string(),
            content: content_str,
        })
    }
}

#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub content: String,
}
use std::io;
use serde::{Deserialize, Serialize};
use crate::db::storage::Storage;

#[derive(Debug)]
pub struct Sessions {
    storage: Storage,
}

impl Sessions {
    pub fn new(storage: Storage) -> Self {
        Sessions { storage }
    }

    pub fn get_or_create_session(&mut self, name: &str) -> Result<Session, io::Error> {
        let mut session = Session {
            name: name.to_string(),
            messages: Vec::new(),
        };

        if self.storage.exists(name) {
            self.storage.load_as_json(name, &mut session.messages)?;
        } else {
            println!("Creating new session: {}", name);
        }

        Ok(session)
    }

    pub fn save_session(&self, session: &Session) -> Result<(), io::Error> {
        self.storage.save_as_json(session.name.as_str(), &session.messages)
    }
}

#[derive(Debug)]
pub struct Session {
    pub name: String,
    pub messages: Vec<Message>,
}

impl Session {
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn append(&mut self, messages: &[Message]) {
        self.messages.extend_from_slice(messages);
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

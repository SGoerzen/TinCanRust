use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LanguageMap {
    pub map: HashMap<String, String>
}

impl LanguageMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn from_json() -> Self {
        panic!("Not implemented yet.");
    }

    pub fn from_string_of_json() -> Self {
        panic!("Not implemented yet.");
    }

    pub fn from_dict(map: HashMap<String, String>) -> Self {
        LanguageMap {
            map
        }
    }

    pub fn to_json(&self) {
        panic!("Not implemented yet.");
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn add(&mut self, lang: String, value: String) -> Option<String> {
        self.map.insert(lang, value)
    }
}
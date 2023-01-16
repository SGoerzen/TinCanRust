use http::Uri;
use crate::language_map::LanguageMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Verb {
    pub id: Uri,
    pub display: LanguageMap,

}

impl From<Uri> for Verb {
    fn from(id: Uri) -> Self {
        Verb {
            id,
            display: LanguageMap::new()
        }
    }
}

impl From<&str> for Verb {
    fn from(id: &str) -> Self {
        let uri = Uri::try_from(id).unwrap();
        Self {
            id: uri,
            display: LanguageMap::new(),
        }
    }
}

impl Verb {
    pub fn new() -> Self {
        Self {
            id: Uri::default(),
            display: LanguageMap::new()
        }
    }

    pub fn from_string_of_json() -> Self {
        panic!("Not implemented yet.");
    }

    pub fn from_json() -> Self {
        panic!("Not implemented yet.");
    }
}
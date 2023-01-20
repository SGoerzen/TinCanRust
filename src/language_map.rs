use std::collections::HashMap;
use std::ops;
use serde_json::Value;
use crate::JsonModel;

use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeMap;

#[derive(Deserialize, Debug, Default)]
pub struct LanguageMap(HashMap<String, String>);


impl LanguageMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add(&mut self, lang: &str, value: &str) -> Option<String> {
        self.0.insert(String::from(lang), String::from(value))
    }

    pub fn len(&self) -> usize { self.0.len() }
}

impl Serialize for LanguageMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}

impl ops::Index<&'static str> for LanguageMap {
    type Output = String;

    fn index(&self, index: &'static str) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl From<HashMap<String, String>> for LanguageMap {
    fn from(map: HashMap<String, String>) -> Self {
        LanguageMap(map)
    }
}

impl From<Value> for LanguageMap {
    fn from(value: Value) -> Self {
        LanguageMap::from_value(value)
    }
}

impl JsonModel for LanguageMap {
    fn to_value(&self) -> Value {
        match serde_json::to_value(&self) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }

    fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }

    fn from_value(value: Value) -> Self {
        match serde_json::from_value(value) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn create_language_map() -> LanguageMap {
        let mut language_map = LanguageMap::new();
        language_map.add("de-DE", "erstellte");
        language_map.add("en-US", "created");
        language_map
    }

    #[test]
    fn language_map() {
        let language_map = create_language_map();
        assert_eq!(language_map["en-US"], "created");
        assert_eq!(language_map["de-DE"], "erstellte");
    }

    #[test]
    fn to_value() {
        let language_map = create_language_map();
        let jobj = language_map.to_value();
        assert_eq!(jobj["en-US"], "created");
        assert_eq!(jobj["de-DE"], "erstellte");
    }

    #[test]
    fn to_json() {
        let language_map = create_language_map();
        assert_eq!(language_map.to_json().len(), "{\"de-DE\":\"erstellte\",\"en-US\":\"created\"}".len());
    }

    #[test]
    fn from_value() {
        let mut map = serde_json::Map::new();
        map.insert(String::from("en-US"), Value::String(String::from("created")));
        map.insert(String::from("de-DE"), Value::String(String::from("erstellte")));
        let value = Value::Object(map);
        let language_map = LanguageMap::from_value(value);
        assert_eq!(language_map["en-US"], "created");
        assert_eq!(language_map["de-DE"], "erstellte");
        assert_eq!(language_map.len(), 2);
    }
}

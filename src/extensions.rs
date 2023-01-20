use std::collections::HashMap;
use std::ops;

use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeMap;
use serde_json::{Value};

use crate::json::JsonModel;
use crate::{MapAdd};

#[derive(Deserialize, Debug, Default)]
pub struct Extensions(HashMap<String, Value>); // todo: Exchange to HashMap<Uri, Value>

impl Extensions {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Serialize for Extensions {
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

impl MapAdd<String, Value> for Extensions {
    fn add(&mut self, key: String, value: Value) {
        self.0.insert(key, value);
    }
}

impl MapAdd<&'static str, &'static str> for Extensions {
    fn add(&mut self, key: &'static str, value: &'static str) {
        self.0.insert(String::from(key), Value::String(String::from(value)));
    }
}

impl ops::Index<&'static str> for Extensions {
    type Output = Value;

    fn index(&self, index: &'static str) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl From<Value> for Extensions {
    fn from(value: Value) -> Self {
        Extensions::from_value(value)
    }
}

impl JsonModel for Extensions {
    fn to_value(&self) -> Value {
        match serde_json::to_value(&self) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }

    fn to_json(&self) -> String{
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

    pub static EXT_KEY: &str = "http://example.com/gliderClubId";
    pub static EXT_VALUE: &str = "course-435";
    pub static EXT_JSON: &str = r#"{"http://example.com/gliderClubId":"course-435"}"#;

    pub fn create_extensions() -> Extensions {
        let mut extensions = Extensions::new();
        extensions.add(EXT_KEY, EXT_VALUE);
        extensions
    }

    #[test]
    fn extensions() {
        let extensions = create_extensions();
        assert_eq!(extensions[EXT_KEY], EXT_VALUE);
        assert_eq!(extensions.len(), extensions.0.len());
    }

    #[test]
    fn to_value() {
        let extensions = create_extensions();
        let jobj = extensions.to_value();
        assert_eq!(jobj[EXT_KEY], EXT_VALUE);
    }

    #[test]
    fn to_json() {
        let extensions = create_extensions();
        assert_eq!(extensions.to_json(), EXT_JSON);
    }

    #[test]
    fn from_value() {
        let mut map = serde_json::Map::new();
        map.insert(String::from(EXT_KEY), Value::String(String::from(EXT_VALUE)));
        let value = Value::Object(map);
        let extensions = Extensions::from_value(value);
        assert_eq!(extensions[EXT_KEY], "course-435");
        assert_eq!(extensions.len(), 1);
    }
}
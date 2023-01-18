use std::collections::HashMap;
use std::ops;

use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::json::JsonModel;
use crate::{MapAdd};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Extensions(HashMap<String, Value>);

impl Extensions {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

impl JsonModel for Extensions {
    fn to_jobject(&self) -> Value {
        let str = self.to_json();
        serde_json::from_str(&str).unwrap()
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn from_jobject(value: Value) -> Self {
        let str = value.to_string();
        serde_json::from_str(&str).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use serde::de::Unexpected::Str;
    use super::*;

    static EXT_KEY: &str = "http://example.com/gliderClubId";

    fn create_extensions() -> Extensions {
        let mut extensions = Extensions::new();
        extensions.add(EXT_KEY, "course-435");
        extensions
    }

    #[test]
    fn extensions() {
        let extensions = create_extensions();
        assert_eq!(extensions[EXT_KEY], "course-435");
        assert_eq!(extensions.len(), extensions.0.len());
    }

    #[test]
    fn to_jobject() {
        let extensions = create_extensions();
        let jobj = extensions.to_jobject();
        assert_eq!(jobj[EXT_KEY], "course-435");
    }

    #[test]
    fn to_json() {
        let extensions = create_extensions();
        assert_eq!(extensions.to_json(), "{\"http://example.com/gliderClubId\":\"course-435\"}");
    }

    #[test]
    fn from_jobject() {
        let mut map = serde_json::Map::new();
        map.insert(String::from(EXT_KEY), Value::String(String::from("course-435")));
        let value = Value::Object(map);
        let extensions = Extensions::from_jobject(value);
        assert_eq!(extensions[EXT_KEY], "course-435");
        assert_eq!(extensions.len(), 1);
    }
}
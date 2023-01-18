use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::JsonModel;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TCAPIVersion(String);

impl TCAPIVersion {
    pub fn v103() -> TCAPIVersion {
        TCAPIVersion::from("1.0.3")
    }

    pub fn v102() -> TCAPIVersion {
        TCAPIVersion::from("1.0.2")
    }

    pub fn v101() -> TCAPIVersion {
        TCAPIVersion::from("1.0.1")
    }

    pub fn v100() -> TCAPIVersion {
        TCAPIVersion::from("1.0.0")
    }

    pub fn v095() -> TCAPIVersion {
        TCAPIVersion::from("0.95")
    }

    pub fn v090() -> TCAPIVersion {
        TCAPIVersion::from("0.9")
    }

    pub fn get_supported() -> [TCAPIVersion; 4] {
        [Self::v103(), Self::v102(), Self::v101(), Self::v100()]
    }

    pub fn get_known() -> [TCAPIVersion; 6] {
        [Self::v103(), Self::v102(), Self::v101(), Self::v100(), Self::v095(), Self::v090()]
    }

    pub fn latest() -> TCAPIVersion {
        Self::v103()
    }
}

impl From<&'static str> for TCAPIVersion {
    fn from(version: &'static str) -> Self {
        Self(String::from(version))
    }
}

impl From<Value> for TCAPIVersion {
    fn from(value: Value) -> Self {
        match serde_json::from_value(value) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }
}

impl Clone for TCAPIVersion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ToString for TCAPIVersion {
    fn to_string(&self) -> String {
        String::from(self.0.clone())
    }
}

impl JsonModel for TCAPIVersion {
    fn to_jobject(&self) -> Value {
        Value::String(self.0.clone())
    }

    fn to_json(&self) -> String {
        self.0.clone()
    }

    fn from_jobject(value: Value) -> Self {
        TCAPIVersion(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let version = TCAPIVersion::from("1.0.0");
        assert_eq!(version.0, "1.0.0");
    }

    #[test]
    fn to_string() {
        let version = TCAPIVersion::from("1.0.0");
        assert_eq!(version.to_string(), String::from("1.0.0"));
    }

    #[test]
    fn get_supported() {
        let supported = TCAPIVersion::get_supported();
        assert_eq!(supported.len(), 4);
    }

    #[test]
    fn get_known() {
        let known = TCAPIVersion::get_known();
        assert_eq!(known.len(), 6);
    }

    #[test]
    fn latest() {
        assert_eq!(TCAPIVersion::latest().0, "1.0.3");
    }

    #[test]
    fn to_jobject() {
        assert_eq!(TCAPIVersion::v103().to_jobject(), Value::String(String::from("1.0.3")))
    }

    #[test]
    fn to_json() {
        assert_eq!(TCAPIVersion::v103().to_json(), String::from("1.0.3"));
    }

    #[test]
    fn from_jobject() {
        let value = Value::String(String::from("1.0.3"));
        assert_eq!(TCAPIVersion::from_jobject(value).0, "\"1.0.3\"")
    }
}
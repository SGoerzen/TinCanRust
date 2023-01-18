use serde_json::{json, Value};
use crate::extensions::Extensions;
use crate::json::JsonModel;
use crate::version::TCAPIVersion;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct About {
    pub version: Vec<TCAPIVersion>,
    pub extensions: Extensions
}

impl JsonModel for About {
    fn to_jobject(&self) -> Value {
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

    fn from_jobject(value: Value) -> Self {
        About::from(value)
    }
}

impl From<Value> for About {
    fn from(value: Value) -> Self {
        match serde_json::from_value(value) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ABOUT_JSON: &str = r#"{
        "version" : ["1.0.3"],
        "extensions" :
        {
            "http://example.com/gliderClubId" : "course-435"
         }
     }"#;

    fn create_about() -> About {
        About {
            version: vec![TCAPIVersion::v103()],
            extensions: crate::extensions::tests::create_extensions(),
        }
    }

    #[test]
    fn about() {
        let a = create_about();
        assert_eq!(a.version[0].to_string(), "1.0.3");
        assert_eq!(a.extensions.len(), 1);
    }

    #[test]
    fn from() {
        let a: About = serde_json::from_str(ABOUT_JSON).unwrap();
        println!("{:?}", a);
    }

    #[test]
    fn to_jobject() {
        let a = create_about();

        let jobj_about = a.to_jobject();
        let jobj_version = a.version[0].clone().to_jobject();
        let jobj_extensions = a.extensions.to_jobject();
        assert_eq!(jobj_about["version"], Value::Array(vec![jobj_version]));
        assert_eq!(jobj_extensions[crate::extensions::tests::EXT_KEY], crate::extensions::tests::EXT_VALUE);
    }

    #[test]
    fn to_json() {
        let a = create_about();
        assert_eq!(a.to_json(), ABOUT_JSON.replace(" ", "").replace("\n", ""));
    }

    #[test]
    fn from_jobject() {
        let a = About::from_jobject(json!({
            "version" : ["1.0.3"],
            "extensions" :
            {"http://example.com/gliderClubId" : "course-435"}
        }));
        assert_eq!(a.version[0].to_string(), "1.0.3");
    }
}
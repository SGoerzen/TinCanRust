use serde_json::{json, Value};
use crate::extensions::Extensions;
use crate::json::JsonModel;
use crate::version::TCAPIVersion;

pub struct About {
    pub version: Vec<TCAPIVersion>,
    pub extensions: Extensions
}

impl About {

}

impl JsonModel for About {
    fn to_jobject(&self) -> Value {
        let versions = self.version.clone().into_iter().map(|v| {
            Value::String(v.to_string())
        }).collect();
        json!({
            "version": Value::Array(versions),
            "extensions": self.extensions.to_jobject()
        })
    }

    fn to_json(&self) -> String {
        self.to_jobject().to_string()
    }

    fn from(value: Value) -> Self {
        panic!("Not implemented yet.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_jobject() {
        let about = About {
            version: vec![crate::version::V102, crate::version::V103],
            extensions: Extensions::new(),
        };
        
        let jobj = about.to_jobject();
        println!("{:?}", jobj.to_string());
    }
}
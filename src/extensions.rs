use http::Uri;
use phf::Map;
use serde_json::Value;
use crate::json::JsonModel;

pub struct Extensions {
    map: Map<Uri, Value>
}

impl Extensions {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl JsonModel for Extensions {
    fn to_jobject(&self) -> Value {
        //self.map.into_iter().map(|| {});
    }

    fn to_json(&self) -> String {
        panic!("Not implemented yet.");
    }

    fn from(value: Value) -> Self {
        panic!("Not implemented yet.");
    }
}
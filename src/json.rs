use serde_json::{Value};

pub trait JsonModel {
    fn to_jobject(&self) -> Value;
    fn to_json(&self) -> String;
    fn from(value: Value) -> Self;
}

pub struct JsonString(&'static str);
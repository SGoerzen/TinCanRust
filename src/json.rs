use serde_json::{Value};

pub trait JsonModel {
    fn to_jobject(&self) -> Value;
    fn to_json(&self) -> String;
    fn from_jobject(value: Value) -> Self;
}
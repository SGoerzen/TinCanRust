use serde_json::{Value};

pub trait JsonModel {
    fn to_value(&self) -> Value;
    fn to_json(&self) -> String;
    fn from_value(value: Value) -> Self;
}
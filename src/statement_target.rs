use serde_json::Value;
use crate::version::TCAPIVersion;

pub trait StatementTarget {
    fn to_jobject(&self, version: TCAPIVersion) -> Value;
    fn object_type(&self) -> String;
}
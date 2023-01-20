use http::Uri;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::JsonModel;
use crate::statement_target::StatementTarget;
use crate::version::TCAPIVersion;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AgentAccount {
    #[serde(with = "http_serde::uri")]
    #[serde(rename = "homePage")]
    pub home_page: Uri,
    pub name: String
}

impl AgentAccount {
    pub fn create(home_page: Uri, name: String) -> Self {
        Self {
            home_page,
            name
        }
    }
}

impl JsonModel for AgentAccount {
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
        AgentAccount::from(value)
    }
}

impl StatementTarget for AgentAccount {
    fn to_jobject(&self, version: TCAPIVersion) -> Value {
        todo!()
    }

    fn object_type(&self) -> String {
        todo!()
    }
}

impl From<Value> for AgentAccount {
    fn from(value: Value) -> Self {
        match serde_json::from_value(value) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }
}

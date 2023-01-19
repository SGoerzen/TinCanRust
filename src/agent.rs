use serde_json::Value;
use crate::agent_account::AgentAccount;
use crate::JsonModel;
use crate::statement_target::StatementTarget;
use crate::version::TCAPIVersion;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Agent {
    name: Option<String>,
    mbox: Option<String>,
    mbox_sha1sum: Option<String>,
    openid: Option<String>,
    account: Option<AgentAccount>
}

impl Agent {
    pub fn object_type() -> String {
        String::from("Agent")
    }
}

impl JsonModel for Agent {
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
        Agent::from(value)
    }
}

impl From<Value> for Agent {
    fn from(value: Value) -> Self {
        match serde_json::from_value(value) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        }
    }
}

impl StatementTarget for Agent {
    fn to_jobject(&self, version: TCAPIVersion) -> Value {
        let mut value = self.to_value();
        value["objectType"] = Value::String(Agent::object_type());
        value
    }

    fn object_type(&self) -> String {
        Agent::object_type()
    }
}
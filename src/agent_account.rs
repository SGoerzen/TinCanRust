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

mod tests {
    use std::str::FromStr;
    use serde_json::json;
    use super::*;

    static ACCOUNT_JSON: &str = r#"{
        "homePage": "https://twitter.com/",
        "name" : "projecttincan"
     }"#;

    fn create_agent_account() -> AgentAccount {
        AgentAccount {
            name: String::from("projecttincan"),
            home_page: Uri::from_str("https://twitter.com/").unwrap()
        }
    }

    #[test]
    fn agent_account() {
        let account = create_agent_account();
        assert_eq!(account.name, "projecttincan");
        assert_eq!(account.home_page, "https://twitter.com/");
    }

    #[test]
    fn from() {
        let account: AgentAccount = serde_json::from_str(ACCOUNT_JSON).unwrap();
        assert_eq!(account.name, "projecttincan");
        assert_eq!(account.home_page, "https://twitter.com/");
    }

    #[test]
    fn to_value() {
        let account = create_agent_account();
        let jobj: Value = account.to_value();
        assert_eq!(jobj["name"], "projecttincan");
        assert_eq!(jobj["homePage"].as_str().unwrap(), "https://twitter.com/");
    }

    #[test]
    fn to_json() {
        let account = create_agent_account();
        assert_eq!(account.to_json(),
                   ACCOUNT_JSON.replace(" ", "")
                       .replace("\n", ""));
    }

    #[test]
    fn from_value() {
        let value: Value = json!({
            "homePage": "https://twitter.com",
            "name" : "projecttincan"
        });

        let account = AgentAccount::from_value(value);
        assert_eq!(account.name, "projecttincan");
        assert_eq!(account.home_page, "https://twitter.com");
    }
}
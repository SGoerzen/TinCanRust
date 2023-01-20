use serde_json::Value;
use crate::agent_account::AgentAccount;
use crate::JsonModel;
use crate::statement_target::StatementTarget;
use crate::version::TCAPIVersion;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Agent {
    pub name: Option<String>,
    pub mbox: Option<String>,
    pub mbox_sha1sum: Option<String>,
    #[serde(rename = "openid")]
    pub open_id: Option<String>,
    pub account: Option<AgentAccount>
}

impl Agent {
    pub fn object_type() -> String {
        String::from("Agent")
    }
}

impl JsonModel for Agent {
    fn to_value(&self) -> Value {
        let mut value = match serde_json::to_value(&self) {
            Ok(v) => v,
            Err(e) => panic!("error: {e:?}")
        };

        value
    }

    fn to_json(&self) -> String{
        let mut v = self.to_value();
        v["objectType"] = Value::String(Agent::object_type());
        match serde_json::to_string(&v) {
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

        value["account"] = match &self.account {
            None => Value::Null,
            Some(account) => account.to_jobject(version)
        };

        value
    }

    fn object_type(&self) -> String {
        Agent::object_type()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use http::Uri;
    use serde_json::json;
    use super::*;

    // See: https://learningpool.com/xapi-statement-101-actor/
    static NAME: &str = "John Smith";
    static MBOX: &str = "mailto:johnsmith@example.com";
    static MBOX_SHA1: &str = "4445904ac65039ef7a91506207f19162ac4dea73";
    static OPEN_ID: &str = "https://www.example.com/johnsmith";
    static ACCOUNT_NAME: &str = "123";
    static ACCOUNT_HOME_PAGE: &str = "https://www.example.com/users/";

    static AGENT_ACCOUNT_JSON: &str = r#"{
        "objectType": "Agent",
        "name" : "John Smith",
        "account" : {
            "name" : "123",
            "homePage": "https://www.example.com/users/"
         }
     }"#;

    static AGENT_MBOX_JSON: &str = r#"{
        "objectType": "Agent",
        "name": "John Smith",
        "mbox": "mailto:johnsmith@example.com"
    }"#;

    static AGENT_SHA1_JSON: &str = r#"{
        "objectType": "Agent",
        "name": "John Smith",
        "mbox_sha1sum": "4445904ac65039ef7a91506207f19162ac4dea73"
    }"#;

    static AGENT_OPENID_JSON: &str = r#"{
        "objectType": "Agent",
        "name": "John Smith",
        "openid": "https://www.example.com/johnsmith"
    }"#;

    static AGENT_JSON: &str = r#"{
        "account" : {
            "homePage": "https://www.example.com/users/",
            "name" : "123"
         },
        "mbox": "mailto:johnsmith@example.com",
        "mbox_sha1sum": "4445904ac65039ef7a91506207f19162ac4dea73",
        "name" : "John Smith",
        "objectType": "Agent",
        "openid": "https://www.example.com/johnsmith"
     }"#;

    fn create_agent() -> Agent {
        Agent {
            name: Some(String::from(NAME)),
            mbox: Some(String::from(MBOX)),
            mbox_sha1sum: Some(String::from(MBOX_SHA1)),
            open_id: Some(String::from(OPEN_ID)),
            account: Some(AgentAccount {
                home_page: Uri::from_str(ACCOUNT_HOME_PAGE).unwrap(),
                name: String::from(ACCOUNT_NAME)
            })
        }
    }

    #[test]
    fn agent() {
        let a = create_agent();
        let account = &a.account.unwrap();
        assert_eq!(a.name.unwrap(), NAME);
        assert_eq!(a.mbox.unwrap(), MBOX);
        assert_eq!(a.mbox_sha1sum.unwrap(), MBOX_SHA1);
        assert_eq!(a.open_id.unwrap(), OPEN_ID);
        assert_eq!(account.name, ACCOUNT_NAME);
        assert_eq!(account.home_page.to_string(), ACCOUNT_HOME_PAGE);
    }

    #[test]
    fn from() {
        let a: Agent = serde_json::from_str(AGENT_ACCOUNT_JSON).unwrap();
        assert_eq!(a.name.unwrap(), NAME);
        let acc = a.account.unwrap();
        assert_eq!(acc.home_page.to_string(), ACCOUNT_HOME_PAGE);
        assert_eq!(acc.name, ACCOUNT_NAME);
        assert_eq!(a.open_id, None);
        assert_eq!(a.mbox, None);
        assert_eq!(a.mbox_sha1sum, None);

        let m: Agent = serde_json::from_str(AGENT_MBOX_JSON).unwrap();
        assert_eq!(m.name.unwrap(), NAME);
        assert_eq!(m.mbox.unwrap(), MBOX);
        assert_eq!(m.account.is_none(), true);
        assert_eq!(m.open_id, None);
        assert_eq!(m.mbox_sha1sum, None);

        let s: Agent = serde_json::from_str(AGENT_SHA1_JSON).unwrap();
        assert_eq!(s.name.unwrap(), NAME);
        assert_eq!(s.mbox_sha1sum.unwrap(), MBOX_SHA1);
        assert_eq!(s.mbox, None);
        assert_eq!(s.account.is_none(), true);
        assert_eq!(s.open_id, None);

        let o: Agent = serde_json::from_str(AGENT_OPENID_JSON).unwrap();
        assert_eq!(o.name.unwrap(), NAME);
        assert_eq!(o.open_id.unwrap(), OPEN_ID);
        assert_eq!(o.mbox, None);
        assert_eq!(o.mbox_sha1sum, None);
        assert_eq!(o.account.is_none(), true);
    }

    #[test]
    fn to_value() {
        let agent = create_agent();
        let jobj: Value = agent.to_value();
        assert_eq!(jobj["name"], NAME);
        assert_eq!(jobj["mbox"], MBOX);
        assert_eq!(jobj["mbox_sha1sum"], MBOX_SHA1);
        assert_eq!(jobj["openid"], OPEN_ID);
        assert_eq!(jobj["account"]["name"], ACCOUNT_NAME);
        assert_eq!(jobj["account"]["homePage"], ACCOUNT_HOME_PAGE);
    }

    #[test]
    fn to_json() {
        let agent = create_agent();
        assert_eq!(agent.to_json(),
            AGENT_JSON.replace(" ", "")
                .replace("\n", "")
                .replace("JohnSmith", "John Smith"));
    }

    #[test]
    fn from_value() {
        let value: Value = json!({
            "objectType": "Agent",
            "name" : "John Smith",
            "mbox": "mailto:johnsmith@example.com",
            "mbox_sha1sum": "4445904ac65039ef7a91506207f19162ac4dea73",
            "account" : {
                "name" : "123",
                "homePage": "https://www.example.com/users/"
             },
             "openid": "https://www.example.com/johnsmith"
        });

        let agent = Agent::from_value(value);
        assert_eq!(agent.name.unwrap(), NAME);
    }
}
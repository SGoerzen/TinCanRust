use crate::agent_account::AgentAccount;

pub struct Agent {
    name: String,
    mbox: String,
    mbox_sha1sum: String,
    openid: String,
    account: AgentAccount
}

impl Agent {
    pub fn from_string_of_json() -> Self {
        panic!("Not implemented yet.");
    }
    pub fn from_json() -> Self {
        panic!("Not implemented yet.");
    }
    pub fn to_json(&self) {
        panic!("Not implemented yet.");
    }
}
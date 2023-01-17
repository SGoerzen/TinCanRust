use http::Uri;

#[derive(Default)]
pub struct AgentAccount {
    home_page: Uri,
    name: String
}

impl AgentAccount {
    pub fn new(home_page: Uri, name: String) -> Self {
        Self {
            home_page,
            name
        }
    }

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
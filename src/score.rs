

pub struct Score {
    scaled: Option<f64>,
    raw: Option<f64>,
    min: Option<f64>,
    max: Option<f64>
}

impl Score {
    pub fn new() -> Self {
        Self {
            scaled: None,
            raw: None,
            min: None,
            max: None,
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
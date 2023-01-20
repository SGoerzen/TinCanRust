use crate::Agent;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Group {
    pub member: Vec<Agent>
}

// todo traits
// todo tests
use crate::activity::Activity;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContextActivities {
    pub parent: Vec<Activity>,
    pub grouping: Vec<Activity>,
    pub category: Vec<Activity>,
    pub other: Vec<Activity>,
}

// todo traits
// todo tests
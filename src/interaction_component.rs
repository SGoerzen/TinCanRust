use crate::LanguageMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InteractionComponent {
    pub id: String,
    pub description: LanguageMap
}

// todo traits
// todo tests
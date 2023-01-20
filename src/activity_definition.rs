use http::{Extensions, Uri};
use crate::{InteractionType, LanguageMap};

use serde::{Serialize, Deserialize}
use crate::interaction_component::InteractionComponent;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ActivityDefinition {
    #[serde(with = "http_serde::uri")]
    #[serde(rename = "type")]
    pub activity_type: Uri,
    #[serde(with = "http_serde::uri")]
    #[serde(rename = "moreInfo")]
    pub more_info: Uri,
    pub name: LanguageMap,
    pub description: LanguageMap,
    pub extensions: Extensions,
    #[serde(rename = "interactionType")]
    pub interaction_type: InteractionType,
    #[serde(rename = "correctResponsesPattern")]
    pub correct_responses_pattern: Vec<String>,
    pub choices: Vec<InteractionComponent>,
    pub scale: Vec<InteractionComponent>,
    pub source: Vec<InteractionComponent>,
    pub target: Vec<InteractionComponent>,
    pub steps: Vec<InteractionComponent>
}

// todo traits
// todo tests
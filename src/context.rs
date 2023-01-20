use http::Extensions;
use crate::agent::Agent;
use crate::context_activities::ContextActivities;
use crate::statement_ref::StatementRef;

type Guid = String;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Context {
    //todo: https://github.com/RusticiSoftware/TinCan.NET/blob/master/TinCan/Context.cs
    pub registration: Option<Guid>, // todo: change to Option<GUID>
    pub instructor: Agent,
    pub team: Agent,
    #[serde(rename = "contextActivities")]
    pub context_activities: ContextActivities,
    pub revision: String,
    pub platform: String,
    pub language: String,
    pub statement: StatementRef,
    pub extensions: Extensions
}


// todo traits
// todo tests
use http::Extensions;
use crate::agent::Agent;

pub struct Context {
    //todo: https://github.com/RusticiSoftware/TinCan.NET/blob/master/TinCan/Context.cs
    registration: Option<String>, // todo: change to Option<GUID>
    instructor: Agent,
    team: Agent,
    // contextActivities,
    revision: String,
    platform: String,
    language: String,
    // statement: StatementRef,
    extensions: Extensions
}
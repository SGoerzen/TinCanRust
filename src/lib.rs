mod about;
mod activity;
mod activity_definition;
mod agent;
mod agent_account;
mod context;
mod context_activities;
mod extensions;
mod group;
mod interaction_component;
mod interaction_type;
mod json;
mod language_map;
mod result;
mod score;
mod statement;
mod statement_ref;
mod statement_target;
mod verb;
pub mod version;

pub use about::About;
pub use agent::Agent;
pub use agent_account::AgentAccount;
pub use context::Context;
pub use extensions::Extensions;
pub use interaction_type::InteractionType;
pub use json::JsonModel;
pub use language_map::LanguageMap;
pub use result::Result;
pub use score::Score;
pub use statement::Statement;
pub use verb::Verb;

pub trait ILRS {
    // todo: https://github.com/RusticiSoftware/TinCan.NET/blob/master/TinCan/ILRS.cs

}

pub trait MapAdd<K, V>: Sized {
    fn add(&mut self, key: K, value: V);
}
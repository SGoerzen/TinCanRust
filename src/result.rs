use timespan::DateTimeSpan;
use chrono::Utc;
use crate::extensions::Extensions;
use crate::score::Score;

pub struct Result {
    completion: Option<bool>,
    success: Option<bool>,
    response: String,
    duration: DateTimeSpan<Utc>,
    score: Score,
    extensions: Extensions
}

impl Result {
    pub fn new() -> Self {
        panic!("Not implemented yet.");
    }
}
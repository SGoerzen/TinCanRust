use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InteractionType {
    Choice,
    Sequencing,
    Likert,
    Matching,
    Performance,
    TrueFalse,
    FillIn,
    LongFillIn,
    Numeric,
    Other,
    Unknown
}

impl InteractionType {
    pub fn from_value(value: &str) -> Self {
        match value {
            "choice" => InteractionType::Choice,
            "sequencing" => InteractionType::Sequencing,
            "likert" => InteractionType::Likert,
            "matching" => InteractionType::Matching,
            "performance" => InteractionType::Performance,
            "true-false" => InteractionType::TrueFalse,
            "fill-in" => InteractionType::FillIn,
            "long-fill-in" => InteractionType::LongFillIn,
            "numeric" => InteractionType::Numeric,
            "other" => InteractionType::Other,
            _ => InteractionType::Unknown
        }
    }

    pub fn to_value(&self) -> &str {
        match self {
            InteractionType::Choice => "choice",
            InteractionType::Sequencing => "sequencing",
            InteractionType::Likert => "likert",
            InteractionType::Matching => "matching",
            InteractionType::Performance => "performance",
            InteractionType::TrueFalse => "true-false",
            InteractionType::FillIn => "fill-in",
            InteractionType::LongFillIn => "long-fill-in",
            InteractionType::Numeric => "numeric",
            InteractionType::Other => "other",
            _ => "unknown"
        }
    }
}

impl fmt::Display for InteractionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        let interaction_type = InteractionType::Other;
        assert_eq!(interaction_type, InteractionType::Other);
    }

    #[test]
    fn test_from_value() {
        let interaction_type = InteractionType::from_value("choice");
        assert_eq!(interaction_type, InteractionType::Choice);

        let interaction_type = InteractionType::from_value("xxx");
        assert_eq!(interaction_type, InteractionType::Unknown);
    }

    #[test]
    fn test_to_value() {
        let interaction_type = InteractionType::FillIn;
        assert_eq!(interaction_type.to_value(), "fill-in");
    }

    #[test]
    fn test_to_string() {
        let interaction_type = InteractionType::TrueFalse;

    }
}
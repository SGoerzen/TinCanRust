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
        assert_eq!(InteractionType::from_value("choice"), InteractionType::Choice);
        assert_eq!(InteractionType::from_value("sequencing"), InteractionType::Sequencing);
        assert_eq!(InteractionType::from_value("likert"), InteractionType::Likert);
        assert_eq!(InteractionType::from_value("matching"), InteractionType::Matching);
        assert_eq!(InteractionType::from_value("true-false"), InteractionType::TrueFalse);
        assert_eq!(InteractionType::from_value("fill-in"), InteractionType::FillIn);
        assert_eq!(InteractionType::from_value("long-fill-in"), InteractionType::LongFillIn);
        assert_eq!(InteractionType::from_value("numeric"), InteractionType::Numeric);
        assert_eq!(InteractionType::from_value("other"), InteractionType::Other);
        assert_eq!(InteractionType::from_value("xxx"), InteractionType::Unknown);
    }

    #[test]
    fn test_to_value() {
        assert_eq!(InteractionType::Choice.to_value(), "choice");
        assert_eq!(InteractionType::Sequencing.to_value(), "sequencing");
        assert_eq!(InteractionType::Likert.to_value(), "likert");
        assert_eq!(InteractionType::Matching.to_value(), "matching");
        assert_eq!(InteractionType::TrueFalse.to_value(), "true-false");
        assert_eq!(InteractionType::FillIn.to_value(), "fill-in");
        assert_eq!(InteractionType::LongFillIn.to_value(), "long-fill-in");
        assert_eq!(InteractionType::Numeric.to_value(), "numeric");
        assert_eq!(InteractionType::Other.to_value(), "other");
        assert_eq!(InteractionType::Unknown.to_value(), "unknown");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(InteractionType::Choice.to_string(), "Choice");
        assert_eq!(InteractionType::Sequencing.to_string(), "Sequencing");
        assert_eq!(InteractionType::Likert.to_string(), "Likert");
        assert_eq!(InteractionType::Matching.to_string(), "Matching");
        assert_eq!(InteractionType::TrueFalse.to_string(), "TrueFalse");
        assert_eq!(InteractionType::FillIn.to_string(), "FillIn");
        assert_eq!(InteractionType::LongFillIn.to_string(), "LongFillIn");
        assert_eq!(InteractionType::Numeric.to_string(), "Numeric");
        assert_eq!(InteractionType::Other.to_string(), "Other");
        assert_eq!(InteractionType::Unknown.to_string(), "Unknown");
    }
}
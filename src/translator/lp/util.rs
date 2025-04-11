use crate::errors::{TranslationErr, VariablesFromDifferentEnvsErr};

use super::keywords::{CommentKeywords, EndKeywords};

impl From<VariablesFromDifferentEnvsErr> for TranslationErr {
    fn from(value: VariablesFromDifferentEnvsErr) -> Self {
        TranslationErr::new(value.to_string())
    }
}

pub fn starts_with_any(s: &str, prefixes: &Vec<String>) -> bool {
    prefixes
        .iter()
        .any(|prefix| s.to_lowercase().starts_with(prefix))
}

pub fn is_comment(line: &str) -> bool {
    line.trim().is_empty() || starts_with_any(line, &CommentKeywords::all())
}

pub fn is_end(line: &str) -> bool {
    line.trim().is_empty() || starts_with_any(line, &EndKeywords::all())
}

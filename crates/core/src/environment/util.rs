//! Environment-level validation helpers.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::VarIdx;
use std::collections::HashMap;
use std::sync::LazyLock;

const ALLOWED_EXTRA: &str = r#"_.,;(){}"‘’'@#!$%&?"#;

static ALLOWED_MSG: LazyLock<String> = LazyLock::new(|| {
    let allowed = ALLOWED_EXTRA
        .chars()
        .map(|c| format!("'{c}'"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "must only contain alphanumeric characters or one of:
  {allowed}"
    )
});

/// Validates whether a proposed variable name fits LunaModel's naming rules.
///
/// The current rules are intentionally conservative because many translation
/// targets have more restrictive identifier grammars than Rust itself.
pub(super) fn ensure_name_valid(name: &str) -> LunaModelResult<()> {
    if !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
        Err(LunaModelError::VariableNameInvalid(
            name.to_string(),
            "must start with an alphabetic character.".into(),
        ))
    } else if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || ALLOWED_EXTRA.contains(c))
    {
        Err(LunaModelError::VariableNameInvalid(
            name.to_string(),
            ALLOWED_MSG.as_str().into(),
        ))
    } else {
        Ok(())
    }
}

/// Ensures that a variable name is not already present in the environment lookup.
pub(super) fn ensure_unused(lookup: &HashMap<String, VarIdx>, name: &str) -> LunaModelResult<()> {
    match lookup.contains_key(name) {
        true => Err(LunaModelError::VariableExists(name.into())),
        false => Ok(()),
    }
}

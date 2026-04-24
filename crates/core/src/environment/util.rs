use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::VarIdx;
use std::collections::HashMap;

// pub(super) fn freeidx(freeidx: &mut Vec<VarIdx>, nvars: VarIdx) -> VarIdx {
//     freeidx.pop().map_or_else(|| nvars, |i| i)
// }

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
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ',' || c == ')' || c == '(')
    {
        Err(LunaModelError::VariableNameInvalid(
            name.to_string(),
            "must only contain alphanumeric characters, '_', ',', ')' or '('".into(),
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

use lunamodel_types::VarIdx;

use crate::error::CapiError;

/// Separator of the canonical string key exposed through the C API.
const KEY_SEP: &str = "-";

/// Encodes a sorted variable tuple as the canonical string key, e.g. `[0, 1, 2]` -> `"0-1-2"`.
pub(crate) fn key_to_string(indices: &[VarIdx]) -> String {
    indices
        .iter()
        .map(VarIdx::to_string)
        .collect::<Vec<_>>()
        .join(KEY_SEP)
}

/// Decodes a canonical string key back into its variable tuple.
pub(crate) fn key_from_str(key: &str) -> Result<Vec<VarIdx>, CapiError> {
    if key.is_empty() {
        return Ok(Vec::new());
    }
    key.split(KEY_SEP)
        .map(|idx| {
            idx.parse::<VarIdx>().map_err(|_| {
                CapiError::invalid_argument(format!("invalid higher-order key: {key:?}"))
            })
        })
        .collect()
}

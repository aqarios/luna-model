//! Runtime check that transformations only modify the model parts they declare.

use lunamodel_core::Model;
use lunamodel_hashing::hash_model_part;
use lunamodel_types::{EnumSet, ModelPart};

use crate::{TranspileErrorKind, TranspileKindResult};

/// Hashes of the model parts a pass declared it would not modify.
pub(crate) struct Untouched(Vec<(ModelPart, u64)>);

impl Untouched {
    /// Hashes every part outside `modifies`.
    pub(crate) fn snapshot(model: &Model, modifies: EnumSet<ModelPart>) -> Self {
        let parts = (!modifies).iter();
        Self(parts.map(|p| (p, hash_model_part(model, p))).collect())
    }

    /// Fails if any snapshotted part changed.
    pub(crate) fn check(&self, model: &Model, pass_name: &str) -> TranspileKindResult<()> {
        let changed: EnumSet<ModelPart> = self
            .0
            .iter()
            .filter(|(p, hash)| hash_model_part(model, *p) != *hash)
            .map(|(p, _)| *p)
            .collect();
        if changed.is_empty() {
            return Ok(());
        }
        Err(TranspileErrorKind::UndeclaredModification {
            pass_name: pass_name.to_owned(),
            parts: changed,
        })
    }
}

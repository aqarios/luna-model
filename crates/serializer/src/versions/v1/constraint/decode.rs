//! Version 1 decoding for constraints.

use indexmap::IndexMap;
use lunamodel_core::{ArcEnv, ConstraintCollection, prelude::Constraint};
use lunamodel_error::LunaModelResult;
use lunamodel_types::Comparator;
use prost::Message;

use crate::encode::{BytesDecodable, Decodable};
use crate::versionize::Unversionizable;

use super::SerConstraintCollection;

impl BytesDecodable<ConstraintCollection, ArcEnv> for SerConstraintCollection {
    /// Decodes version-0 bytes into a constraint collection tied to `payload`.
    fn decode_from_bytes(bytes: &[u8], payload: ArcEnv) -> LunaModelResult<ConstraintCollection> {
        Self::decode(bytes)?.extract(payload)
    }
}

impl SerConstraintCollection {
    /// Extracts the runtime constraint collection from the protobuf structure.
    pub fn extract(&self, env: ArcEnv) -> LunaModelResult<ConstraintCollection> {
        let mut constraints = IndexMap::new();

        for constr in self.elements.iter() {
            let lhs = constr
                .lhs
                .as_slice()
                .unversionize_nested()
                .decode(env.clone())?;
            let cmp = match constr.cmp {
                0 => Comparator::Le,
                1 => Comparator::Eq,
                2 => Comparator::Ge,
                _ => unreachable!("undefined comparator '{}'", constr.cmp),
            };
            let name = match &constr.name {
                n if n == "<NN>" => None,
                x => Some(x.clone()),
            };
            let constr = Constraint::new(lhs, constr.rhs, cmp, name)?;
            constraints.insert(constr.name().to_owned(), constr);
        }
        Ok(constraints.into())
    }
}

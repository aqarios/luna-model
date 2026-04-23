use indexmap::IndexMap;
use lunamodel_core::{ArcEnv, ConstraintCollection, prelude::Constraint};
use lunamodel_error::LunaModelResult;
use lunamodel_types::Comparator;
use prost::Message;

use crate::encode::{BytesDecodable, Decodable};

use super::SerConstraintCollection;

impl BytesDecodable<ConstraintCollection, ArcEnv> for SerConstraintCollection {
    fn decode_from_bytes(bytes: &[u8], payload: ArcEnv) -> LunaModelResult<ConstraintCollection> {
        Self::decode(bytes)?.extract(payload)
    }
}

impl SerConstraintCollection {
    pub fn extract(&self, env: ArcEnv) -> LunaModelResult<ConstraintCollection> {
        let mut constraints = IndexMap::new();

        for (((name, lhs), cmp), rhs) in self
            .names
            .iter()
            .zip(&self.lhsides)
            .zip(&self.comparators)
            .zip(&self.rhsides)
        {
            let lhs = lhs.decode(env.clone())?;
            let comparator = match cmp {
                0 => Comparator::Le,
                1 => Comparator::Eq,
                2 => Comparator::Ge,
                _ => unreachable!("undefined comparator '{}'", cmp),
            };
            let name = match name {
                n if n == "<NN>" => None,
                x => Some(x.clone()),
            };
            let constr = Constraint::new(lhs, *rhs, comparator, name.clone()).unwrap();
            constraints.insert(constr.name().to_string(), constr);
        }
        Ok(constraints.into())
    }
}

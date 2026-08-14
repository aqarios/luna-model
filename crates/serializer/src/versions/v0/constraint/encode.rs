//! Version 0 encoding for constraints.

use lunamodel_core::prelude::ConstraintCollection;
use lunamodel_types::Comparator;
use prost::Message;

use crate::encode::{BytesEncodable, Encodable};
use crate::versionize::Versionizable;

use super::SerConstraintCollection;

impl BytesEncodable for SerConstraintCollection {
    /// Encodes the protobuf structure into raw bytes.
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl SerConstraintCollection {
    /// Fills the protobuf structure from the runtime constraint collection.
    pub fn fill(mut self, cc: &ConstraintCollection) -> Self {
        for (_, c) in cc.iter() {
            // Tagged with the LHS expression's own version so a decoder can
            // dispatch to the right version-specific decoder even if the
            // Expression format has moved ahead of this SerConstraintCollection
            // format. See Issue(474): <https://github.com/aqarios/luna-model/issues/474>
            let lhs_bytes = c.lhs.serialize().versionize(c.lhs.version());
            let cmp = match c.comparator {
                Comparator::Le => 0,
                Comparator::Eq => 1,
                Comparator::Ge => 2,
            };
            self.lhsides.push(lhs_bytes);
            self.rhsides.push(c.rhs);
            self.comparators.push(cmp);
            self.names.push(c.name().to_string());
        }
        self
    }
}

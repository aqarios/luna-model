use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerExpression as SerExprV0;
// use crate::versions::v1::SerExpression as SerExprV1;

use lunamodel_core::{ArcEnv, Expression};
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of an [Expression]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerExprLatest = SerExprV0;
/// Makes an [Expression] encodable.
impl Encodable<SerExprV0> for Expression {
    fn version(&self) -> Version {
        Version::V0
    }
}

// type SerExprLatest = SerExprV1;
// /// Makes an [Expression] encodable.
// impl Encodable<SerExprV1> for Expression {
//     fn version(&self) -> Version {
//         Version::V1
//     }
// }

/// Default implementation to make a bytes vector ([Vec<u8>]) deserializable to an [Expression].
/// For the decoding of a [Vec<u8>] to an [Expression] a pointer to
/// it's [Environment] is required (given by the Payload type)
impl Decodable<Expression> for Vec<u8> {
    type Latest = SerExprLatest;
    type Payload = ArcEnv;
}
/// Makes a [Version]ized representation of the [Expression] decodable.
/// For the decoding of a bytes vector to an [Expression] a pointer to
/// it's [Environment] is required (given by the Payload type)
impl Decodable<Expression> for Versioned<Vec<u8>> {
    type Latest = SerExprLatest;
    type Payload = ArcEnv;

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Expression> {
        match self.version {
            Some(Version::V0) => SerExprV0::decoder(self.data.as_slice(), payload),
            // Some(Version::V1) => SerExprV1::decoder(self.data.as_slice(), payload),
            _ => SerExprLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

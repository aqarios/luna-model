use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerExpression as SerExprV0;
use crate::versions::v1::SerExpression as SerExprV1;

use lunamodel_core::{ArcEnv, Expression};
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of an [Expression]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
// TODO: delete until TODO(@HERE) and uncomment from TODO(@HERE) for V1 activation.
type SerExprLatest = SerExprV0;
/// Makes an [Expression] encodable.
impl Encodable<SerExprV0> for Expression {
    fn version(&self) -> Version {
        Version::V0
    }
}
impl Decoder<Expression, ArcEnv> for SerExprV1 {}
// TODO: delete ABOVE code and activate below code for V1 activation.
// type SerExprLatest = SerExprV1;
// /// Makes an [Expression] encodable.
// impl Encodable<SerExprV1> for Expression {
//     fn version(&self) -> Version {
//         Version::V1
//     }
// }
// impl Decoder<Expression, ArcEnv> for SerExprV0 {}

/// Makes a raw byte vector (`Vec<u8>`) decodable into an [`Expression`].
///
/// Decoding an expression requires the target [`ArcEnv`] because serialized
/// terms refer to variable ids owned by an environment.
impl Decodable<Expression> for Vec<u8> {
    type Latest = SerExprLatest;
    type Payload = ArcEnv;
}
/// Makes a versioned byte representation decodable into an [`Expression`].
///
/// Decoding still requires the target [`ArcEnv`] because serialized terms refer
/// to variable ids owned by an environment.
impl Decodable<Expression> for Versioned<Vec<u8>> {
    type Latest = SerExprLatest;
    type Payload = ArcEnv;

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Expression> {
        match self.version {
            Some(Version::V0) => SerExprV0::decoder(self.data.as_slice(), payload),
            Some(Version::V1) => SerExprV1::decoder(self.data.as_slice(), payload),
            _ => SerExprLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

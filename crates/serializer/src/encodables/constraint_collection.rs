use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerConstraintCollection as SerConstrCollV0;

use lunamodel_core::{ArcEnv, ConstraintCollection};
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of [ConstraintCollection]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerConstrLatest = SerConstrCollV0;

/// Makes a [ConstraintCollection] encodable.
impl Encodable<SerConstrCollV0> for ConstraintCollection {
    fn version(&self) -> Version {
        Version::V0
    }
}

/// Makes a raw byte vector decodable into a [`ConstraintCollection`].
///
/// Decoding requires the target [`ArcEnv`] because the contained expressions
/// refer to variable ids owned by an environment.
impl Decodable<ConstraintCollection> for Vec<u8> {
    type Latest = SerConstrLatest;
    type Payload = ArcEnv;
}
/// Makes a versioned byte representation decodable into a [`ConstraintCollection`].
///
/// Decoding requires the target [`ArcEnv`] because the contained expressions
/// refer to variable ids owned by an environment.
impl Decodable<ConstraintCollection> for Versioned<Vec<u8>> {
    type Latest = SerConstrLatest;
    type Payload = ArcEnv;

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<ConstraintCollection> {
        match self.version {
            Some(Version::V0) => SerConstrCollV0::decoder(self.data.as_slice(), payload),
            _ => SerConstrLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

use crate::encode::{Decodable, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerConstraintCollection as SerConstrCollV0;

use lunamodel_core::{ConstraintCollection, ArcEnv};
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

/// Default implementation to make a bytes vector deserializable to a [ConstraintCollection].
/// For the decoding of a bytes vector to an [ConstraintCollection] a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConstraintCollection> for Vec<u8> {
    type Latest = SerConstrLatest;
    type Payload = ArcEnv;
}
/// Makes a versionized representation of the [ConstraintCollection] decodable.
/// For the decoding of a bytes vector to a [ConstraintCollection] a reference counted pointer to
/// it's environment is required (given by the Payload type)
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

use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerTransformationRecord as SerTransRecV0;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::TransformationRecord;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of [TransformationRecord]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerCompRecLatest = SerTransRecV0;

/// Makes a [TransformationRecord] encodable.
impl Encodable<SerTransRecV0> for TransformationRecord {
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<TransformationRecord> for Vec<u8> {
    type Latest = SerCompRecLatest;
    type Payload = ();
}

impl Decodable<TransformationRecord> for Versioned<Vec<u8>> {
    type Latest = SerCompRecLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<TransformationRecord> {
        match self.version {
            Some(Version::V0) => SerTransRecV0::decoder(self.data.as_slice(), payload),
            _ => SerCompRecLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

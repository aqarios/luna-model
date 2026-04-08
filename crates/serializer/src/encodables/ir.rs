use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerIR as SerIRV0;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::TransformationOutput;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of [IR]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerCompRecLatest = SerIRV0;

/// Makes a [IR] encodable.
impl Encodable<SerIRV0> for TransformationOutput {
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<TransformationOutput> for Vec<u8> {
    type Latest = SerCompRecLatest;
    type Payload = ();
}

impl Decodable<TransformationOutput> for Versioned<Vec<u8>> {
    type Latest = SerCompRecLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<TransformationOutput> {
        match self.version {
            Some(Version::V0) => SerIRV0::decoder(self.data.as_slice(), payload),
            _ => SerCompRecLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

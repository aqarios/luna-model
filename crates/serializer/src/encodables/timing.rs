use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerTiming as SerTimingV0;

use lunamodel_core::Timing;
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of a [Timing]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerTimingLatest = SerTimingV0;

/// Makes a [Timing] encodable.
impl Encodable<SerTimingV0> for Timing {
    fn version(&self) -> Version {
        Version::V0
    }
}

/// Default implementation to make a bytes vector ([Vec<u8>]) deserializable to a [Timing].
impl Decodable<Timing> for Vec<u8> {
    type Latest = SerTimingLatest;
    type Payload = ();
}
/// Makes a [Version]ized representation of the [Timing] decodable.
impl Decodable<Timing> for Versioned<Vec<u8>> {
    type Latest = SerTimingLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Timing> {
        match self.version {
            Some(Version::V0) => SerTimingV0::decoder(self.data.as_slice(), payload),
            _ => SerTimingV0::decoder(self.data.as_slice(), payload),
        }
    }
}

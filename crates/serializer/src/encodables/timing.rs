//! Version-independent encoding glue for timing values.

use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerTiming as SerTimingV0;
use crate::versions::v1::SerTiming as SerTimingV1;

use lunamodel_core::Timing;
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of a [Timing]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerTimingLatest = SerTimingV1;

impl Decoder<Timing, ()> for SerTimingV0 {}

/// Makes a [Timing] encodable.
impl Encodable<SerTimingV1> for Timing {
    fn version(&self) -> Version {
        Version::V1
    }
}

/// Makes a raw byte vector (`Vec<u8>`) decodable into a [`Timing`].
impl Decodable<Timing> for Vec<u8> {
    type Latest = SerTimingLatest;
    type Payload = ();
}
/// Makes a versioned byte representation decodable into a [`Timing`].
impl Decodable<Timing> for Versioned<Vec<u8>> {
    type Latest = SerTimingLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Timing> {
        match self.version {
            Some(Version::V0) => SerTimingV0::decoder(self.data.as_slice(), payload),
            Some(Version::V1) => SerTimingV1::decoder(self.data.as_slice(), payload),
            // fallback to make solution decode work with unversionized data, was v0.
            _ => SerTimingV0::decoder(self.data.as_slice(), payload),
        }
    }
}

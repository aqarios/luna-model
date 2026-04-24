//! Version-independent encoding glue for models.

use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerModel as SerModelV0;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of a [Model]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerModelLatest = SerModelV0;

/// Makes a [Model] encodable.
impl Encodable<SerModelV0> for Model {
    fn version(&self) -> Version {
        Version::V0
    }
}

/// Makes a raw byte vector decodable into a [`Model`].
impl Decodable<Model> for Vec<u8> {
    type Latest = SerModelLatest;
    type Payload = ();
}
/// Makes a versioned byte representation decodable into a [`Model`].
impl Decodable<Model> for Versioned<Vec<u8>> {
    type Latest = SerModelLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Model> {
        match self.version {
            Some(Version::V0) => SerModelV0::decoder(self.data.as_slice(), payload),
            _ => SerModelLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

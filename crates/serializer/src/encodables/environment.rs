use crate::encode::{Decodable, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerEnvironment as SerEnvV0;

use lunamodel_core::Environment;
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of an [Environment]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerEnvLatest = SerEnvV0;

/// Makes an [Environment] encodable.
impl Encodable<SerEnvV0> for Environment {
    fn version(&self) -> Version {
        Version::V0
    }
}

/// Default implementation to make a bytes vector deserializable to an [Environment].
impl Decodable<Environment> for Vec<u8> {
    type Latest = SerEnvLatest;
    type Payload = ();
}
/// Makes a versionized representation of the [Environment] decodable.
impl Decodable<Environment> for Versioned<Vec<u8>> {
    type Latest = SerEnvLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Environment> {
        match self.version {
            Some(Version::V0) => SerEnvV0::decoder(self.data.as_slice(), payload),
            _ => SerEnvLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerEnvironment as SerEnvV0;
use crate::versions::v1::SerEnvironment as SerEnvV1;

use lunamodel_core::Environment;
use lunamodel_error::LunaModelResult;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of an [Environment]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
// TODO: delete until TODO(@HERE) and uncomment from TODO(@HERE) for V1 activation.
type SerEnvLatest = SerEnvV0;
/// Makes an [Environment] encodable.
impl Encodable<SerEnvV0> for Environment {
    fn version(&self) -> Version {
        Version::V0
    }
}
impl Decoder<Environment, ()> for SerEnvV1 {}
// TODO(@HERE): delete ABOVE code and activate below code for V1 activation.
// type SerEnvLatest = SerEnvV1;
// /// Makes an [Environment] encodable.
// impl Encodable<SerEnvV1> for Environment {
//     fn version(&self) -> Version {
//         Version::V1
//     }
// }
// impl Decoder<Environment, ()> for SerEnvV0 {}

/// Makes a raw byte vector decodable into an [`Environment`].
impl Decodable<Environment> for Vec<u8> {
    type Latest = SerEnvLatest;
    type Payload = ();
}
/// Makes a versioned byte representation decodable into an [`Environment`].
impl Decodable<Environment> for Versioned<Vec<u8>> {
    type Latest = SerEnvLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<Environment> {
        match self.version {
            Some(Version::V0) => SerEnvV0::decoder(self.data.as_slice(), payload),
            Some(Version::V1) => SerEnvV1::decoder(self.data.as_slice(), payload),
            _ => SerEnvLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

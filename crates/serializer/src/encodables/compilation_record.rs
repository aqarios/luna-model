use crate::encode::{Decodable, Decoder, Encodable};
use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerCompilationRecord as SerComRecV0;

use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::CompilationRecord;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of [CompilationRecord]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerCompRecLatest = SerComRecV0;

/// Makes a [CompilationRecord] encodable.
impl Encodable<SerComRecV0> for CompilationRecord {
    fn version(&self) -> Version {
        Version::V0
    }
}

impl Decodable<CompilationRecord> for Vec<u8> {
    type Latest = SerCompRecLatest;
    type Payload = ();
}
/// Makes a versionized representation of the [ConstraintCollection] decodable.
/// For the decoding of a bytes vector to a [ConstraintCollection] a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<CompilationRecord> for Versioned<Vec<u8>> {
    type Latest = SerCompRecLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> LunaModelResult<CompilationRecord> {
        match self.version {
            Some(Version::V0) => SerComRecV0::decoder(self.data.as_slice(), payload),
            _ => SerCompRecLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

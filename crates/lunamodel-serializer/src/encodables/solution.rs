use crate::encode::{Decodable, Encodable};

use crate::versionize::{Version, Versioned};
use crate::versions::v0::SerSolution as SerSolutionV0;
use crate::versions::v1::SerSolution as SerSolutionV1;

use lunamodel_core::Solution;
use lunamodel_error::LunaModelError;

/// Helper type to ensure easier version updates to a new serialization implementation
/// of a [Solution]. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerSolutionLatest = SerSolutionV1;

/// Makes a Solution encodable.
impl Encodable<SerSolutionV1> for Solution {
    fn version(&self) -> Version {
        Version::V1
    }
}

/// Default implementation to make a bytes vector deserializable to a [Solution].
impl Decodable<Solution> for Vec<u8> {
    type Latest = SerSolutionLatest;
    type Payload = ();
}
/// Makes a versionized representation of the [Solution] decodable.
impl Decodable<Solution> for Versioned<Vec<u8>> {
    type Latest = SerSolutionLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Solution, LunaModelError> {
        match self.version {
            Some(Version::V0) => SerSolutionV0::decoder(self.data.as_slice(), payload),
            Some(Version::V1) => SerSolutionV1::decoder(self.data.as_slice(), payload),
            _ => SerSolutionLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

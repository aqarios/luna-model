use super::encodable::{Decodable, DecodeError, Decoder, Encodable};
use super::versionizable::Versioned;
use super::versions::v0::SerConstraints as SerConstrV0;
use super::versions::v0::SerEnvironment as SerEnvV0;
use super::versions::v0::SerExpression as SerExprV0;
use super::versions::v0::SerModel as SerModelV0;
use super::versions::v0::SerSolution as SerSolutionV0;
use super::versions::v0::SerTiming as SerTimingV0;
use super::Version;
use crate::core::{
    ConcreteConstraints, ConcreteEnvironment, ConcreteExpression, ConcreteModel,
    ConcreteMutRcEnvironment, ConcreteSolution, Timing,
};

/// Helper type to ensure easier version updates to a new serialization implementation
/// of an Expression. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerExprLatest = SerExprV0;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of Constraints. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerConstrLatest = SerConstrV0;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of an Environment. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerEnvLatest = SerEnvV0;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of a Model. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerModelLatest = SerModelV0;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of a Solution. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerSolutionLatest = SerSolutionV0;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of a Timing. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerTimingLatest = SerTimingV0;

/// Makes an Expression with Index = VarId and Bias = f64 encodable.
impl Encodable<SerExprV0> for ConcreteExpression {}
/// Makes a Constraints with Index = VarId and Bias = f64 encodable.
impl Encodable<SerConstrV0> for ConcreteConstraints {}
/// Makes an Environment with Index = VarId encodable.
impl Encodable<SerEnvV0> for ConcreteEnvironment {}
/// Makes a Model with Index = VarId and Bias = f64 encodable.
impl Encodable<SerModelV0> for ConcreteModel {}
/// Makes a Solution encodable.
impl Encodable<SerSolutionV0> for ConcreteSolution {}
/// Makes a Timing encodable.
impl Encodable<SerTimingV0> for Timing {}

/// Default implementation to make a bytes vector deserializable to an Expression.
/// For the decoding of a bytes vector to an Expression a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConcreteExpression> for Vec<u8> {
    type Latest = SerExprLatest;
    type Payload = ConcreteMutRcEnvironment;
}
/// Makes a versionized representation of the Expression decodable.
/// For the decoding of a bytes vector to an Expression a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConcreteExpression> for Versioned<Vec<u8>> {
    type Latest = SerExprLatest;
    type Payload = ConcreteMutRcEnvironment;

    fn decode(&self, payload: Self::Payload) -> Result<ConcreteExpression, DecodeError> {
        match self.version {
            Some(Version::V0) => SerExprV0::decoder(self.data.as_slice(), payload),
            None => SerExprLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a Constraints.
/// For the decoding of a bytes vector to an Constraints a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConcreteConstraints> for Vec<u8> {
    type Latest = SerConstrLatest;
    type Payload = ConcreteMutRcEnvironment;
}
/// Makes a versionized representation of the Constraints decodable.
/// For the decoding of a bytes vector to a Constraints a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConcreteConstraints> for Versioned<Vec<u8>> {
    type Latest = SerConstrLatest;
    type Payload = ConcreteMutRcEnvironment;

    fn decode(&self, payload: Self::Payload) -> Result<ConcreteConstraints, DecodeError> {
        match self.version {
            Some(Version::V0) => SerConstrV0::decoder(self.data.as_slice(), payload),
            None => SerConstrV0::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to an Environment.
impl Decodable<ConcreteEnvironment> for Vec<u8> {
    type Latest = SerEnvLatest;
    type Payload = ();
}
/// Makes a versionized representation of the Environment decodable.
impl Decodable<ConcreteEnvironment> for Versioned<Vec<u8>> {
    type Latest = SerEnvLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<ConcreteEnvironment, DecodeError> {
        match self.version {
            Some(Version::V0) => SerEnvV0::decoder(self.data.as_slice(), payload),
            None => SerEnvLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a Model.
impl Decodable<ConcreteModel> for Vec<u8> {
    type Latest = SerModelLatest;
    type Payload = ();
}
/// Makes a versionized representation of the Model decodable.
impl Decodable<ConcreteModel> for Versioned<Vec<u8>> {
    type Latest = SerModelLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<ConcreteModel, DecodeError> {
        match self.version {
            Some(Version::V0) => SerModelV0::decoder(self.data.as_slice(), payload),
            None => SerModelLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a Solution.
impl Decodable<ConcreteSolution> for Vec<u8> {
    type Latest = SerSolutionLatest;
    type Payload = ();
}
/// Makes a versionized representation of the Model decodable.
impl Decodable<ConcreteSolution> for Versioned<Vec<u8>> {
    type Latest = SerSolutionLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<ConcreteSolution, DecodeError> {
        match self.version {
            Some(Version::V0) => SerSolutionV0::decoder(self.data.as_slice(), payload),
            None => SerSolutionLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a Timing.
impl Decodable<Timing> for Vec<u8> {
    type Latest = SerTimingLatest;
    type Payload = ();
}

/// Makes a versionized representation of the Timing decodable.
impl Decodable<Timing> for Versioned<Vec<u8>> {
    type Latest = SerTimingLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Timing, DecodeError> {
        match self.version {
            Some(Version::V0) => SerTimingV0::decoder(self.data.as_slice(), payload),
            None => SerTimingLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

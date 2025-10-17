use super::encodable::{Decodable, DecodeError, Decoder, Encodable};
use super::versionizable::Versioned;
use super::versions::v0::SerConstraints as SerConstrV0;
use super::versions::v0::SerEnvironment as SerEnvV0;
use super::versions::v0::SerExpression as SerExprV0;
use super::versions::v0::SerModel as SerModelV0;
use super::versions::v0::SerSolution as SerSolutionV0;
use super::versions::v0::SerTiming as SerTimingV0;
use super::versions::v1::SerSolution as SerSolutionV1;
use super::Version;
use crate::core::environment::SharedEnvironment;
use crate::core::{ConstraintCollection, Environment, Expression, Model, Solution, Timing};

/// Helper type to ensure easier version updates to a new serialization implementation
/// of an Expression. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerExprLatest = SerExprV0;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of ConstraintCollection. In case a new serialization format is defined update this value
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
type SerSolutionLatest = SerSolutionV1;
/// Helper type to ensure easier version updates to a new serialization implementation
/// of a Timing. In case a new serialization format is defined update this value
/// to ensure all uses of serialization throught the entire library use the most recent
/// serialization implementation.
type SerTimingLatest = SerTimingV0;

/// Makes an Expression with Index = VarId and Bias = f64 encodable.
impl Encodable<SerExprV0> for Expression {
    fn version(&self) -> Version {
        Version::V0
    }
}
/// Makes a ConstraintCollection with Index = VarId and Bias = f64 encodable.
impl Encodable<SerConstrV0> for ConstraintCollection {
    fn version(&self) -> Version {
        Version::V0
    }
}
/// Makes an Environment with Index = VarId encodable.
impl Encodable<SerEnvV0> for Environment {
    fn version(&self) -> Version {
        Version::V0
    }
}
/// Makes a Model with Index = VarId and Bias = f64 encodable.
impl Encodable<SerModelV0> for Model {
    fn version(&self) -> Version {
        Version::V0
    }
}
/// Makes a Solution encodable.
impl Encodable<SerSolutionV1> for Solution {
    fn version(&self) -> Version {
        Version::V1
    }
}
/// Makes a Timing encodable.
impl Encodable<SerTimingV0> for Timing {
    fn version(&self) -> Version {
        Version::V0
    }
}
/// Makes a Solution decodable for V0.
impl Decoder<Solution, ()> for SerSolutionV0 {}

/// Default implementation to make a bytes vector deserializable to an Expression.
/// For the decoding of a bytes vector to an Expression a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<Expression> for Vec<u8> {
    type Latest = SerExprLatest;
    type Payload = SharedEnvironment;
}
/// Makes a versionized representation of the Expression decodable.
/// For the decoding of a bytes vector to an Expression a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<Expression> for Versioned<Vec<u8>> {
    type Latest = SerExprLatest;
    type Payload = SharedEnvironment;

    fn decode(&self, payload: Self::Payload) -> Result<Expression, DecodeError> {
        match self.version {
            Some(Version::V0) => SerExprV0::decoder(self.data.as_slice(), payload),
            _ => SerExprLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a ConstraintCollection.
/// For the decoding of a bytes vector to an ConstraintCollection a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConstraintCollection> for Vec<u8> {
    type Latest = SerConstrLatest;
    type Payload = SharedEnvironment;
}
/// Makes a versionized representation of the ConstraintCollection decodable.
/// For the decoding of a bytes vector to a ConstraintCollection a reference counted pointer to
/// it's environment is required (given by the Payload type)
impl Decodable<ConstraintCollection> for Versioned<Vec<u8>> {
    type Latest = SerConstrLatest;
    type Payload = SharedEnvironment;

    fn decode(&self, payload: Self::Payload) -> Result<ConstraintCollection, DecodeError> {
        match self.version {
            Some(Version::V0) => SerConstrV0::decoder(self.data.as_slice(), payload),
            _ => SerConstrLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to an Environment.
impl Decodable<Environment> for Vec<u8> {
    type Latest = SerEnvLatest;
    type Payload = ();
}
/// Makes a versionized representation of the Environment decodable.
impl Decodable<Environment> for Versioned<Vec<u8>> {
    type Latest = SerEnvLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Environment, DecodeError> {
        match self.version {
            Some(Version::V0) => SerEnvV0::decoder(self.data.as_slice(), payload),
            _ => SerEnvLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a Model.
impl Decodable<Model> for Vec<u8> {
    type Latest = SerModelLatest;
    type Payload = ();
}
/// Makes a versionized representation of the Model decodable.
impl Decodable<Model> for Versioned<Vec<u8>> {
    type Latest = SerModelLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Model, DecodeError> {
        match self.version {
            Some(Version::V0) => SerModelV0::decoder(self.data.as_slice(), payload),
            _ => SerModelLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

/// Default implementation to make a bytes vector deserializable to a Solution.
impl Decodable<Solution> for Vec<u8> {
    type Latest = SerSolutionLatest;
    type Payload = ();
}
/// Makes a versionized representation of the Model decodable.
impl Decodable<Solution> for Versioned<Vec<u8>> {
    type Latest = SerSolutionLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Solution, DecodeError> {
        match self.version {
            Some(Version::V0) => SerSolutionV0::decoder(self.data.as_slice(), payload),
            Some(Version::V1) => SerSolutionV1::decoder(self.data.as_slice(), payload),
            _ => SerSolutionLatest::decoder(self.data.as_slice(), payload),
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
            _ => SerTimingLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

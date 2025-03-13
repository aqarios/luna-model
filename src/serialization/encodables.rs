use std::cell::RefCell;
use std::rc::Rc;

use super::encodable::{Decodable, DecodeError, Decoder, Encodable};
use super::versionizable::Versioned;
use super::versions::v0::SerConstraints as SerConstrV0;
use super::versions::v0::SerEnvironment as SerEnvV0;
use super::versions::v0::SerExpression as SerExprV0;
use super::versions::v0::SerModel as SerModelV0;
use super::Version;
use crate::core::{Constraints, Environment, Model};
use crate::core::{Expression, VarId};

type SerExprLatest = SerExprV0;
type SerConstrLatest = SerConstrV0;
type SerEnvLatest = SerEnvV0;
type SerModelLatest = SerModelV0;

impl Encodable<SerExprV0> for Expression<VarId, f64> {}
impl Encodable<SerConstrV0> for Constraints<VarId, f64> {}
impl Encodable<SerEnvV0> for Environment<VarId> {}
impl Encodable<SerModelV0> for Model<VarId, f64> {}

impl Decodable<Expression<VarId, f64>> for Vec<u8> {
    type Latest = SerExprLatest;
    type Payload = Rc<RefCell<Environment<VarId>>>;
}
impl Decodable<Expression<VarId, f64>> for Versioned<Vec<u8>> {
    type Latest = SerExprLatest;
    type Payload = Rc<RefCell<Environment<VarId>>>;

    fn decode(&self, payload: Self::Payload) -> Result<Expression<VarId, f64>, DecodeError> {
        match self.version {
            Some(Version::V0) => SerExprV0::decoder(self.data.as_slice(), payload),
            None => SerExprLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

impl Decodable<Constraints<VarId, f64>> for Vec<u8> {
    type Latest = SerConstrLatest;
    type Payload = Rc<RefCell<Environment<VarId>>>;
}
impl Decodable<Constraints<VarId, f64>> for Versioned<Vec<u8>> {
    type Latest = SerConstrLatest;
    type Payload = Rc<RefCell<Environment<VarId>>>;

    fn decode(&self, payload: Self::Payload) -> Result<Constraints<VarId, f64>, DecodeError> {
        match self.version {
            Some(Version::V0) => SerConstrV0::decoder(self.data.as_slice(), payload),
            None => SerConstrV0::decoder(self.data.as_slice(), payload),
        }
    }
}

impl Decodable<Environment<VarId>> for Vec<u8> {
    type Latest = SerEnvLatest;
    type Payload = ();
}
impl Decodable<Environment<VarId>> for Versioned<Vec<u8>> {
    type Latest = SerEnvLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Environment<VarId>, DecodeError> {
        match self.version {
            Some(Version::V0) => SerEnvV0::decoder(self.data.as_slice(), payload),
            None => SerEnvLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

impl Decodable<Model<VarId, f64>> for Vec<u8> {
    type Latest = SerModelLatest;
    type Payload = ();
}
impl Decodable<Model<VarId, f64>> for Versioned<Vec<u8>> {
    type Latest = SerModelLatest;
    type Payload = ();

    fn decode(&self, payload: Self::Payload) -> Result<Model<VarId, f64>, DecodeError> {
        match self.version {
            Some(Version::V0) => SerModelV0::decoder(self.data.as_slice(), payload),
            None => SerModelLatest::decoder(self.data.as_slice(), payload),
        }
    }
}

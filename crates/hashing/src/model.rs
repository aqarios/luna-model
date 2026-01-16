use lunamodel_core::Model;
use prost::Message;

use super::constr::HashConstr;
use super::env::HashEnv;
use super::expr::HashExpr;

#[derive(Clone, PartialEq, Message)]
pub struct HashModel {
    /// Representation of the objective as a byte vector, i.e. an encoded Expression.
    #[prost(bytes, tag = "1")]
    pub objective: Vec<u8>,
    /// Representation of the constraints as a byte vector, i.e. an encoded Constraints.
    #[prost(bytes, tag = "2")]
    pub constraints: Vec<u8>,
    /// Representation of the environment as a byte vector, i.e., an encoded Environment.
    #[prost(bytes, tag = "3")]
    pub environment: Vec<u8>,
    /// The name of the model.
    #[prost(string, tag = "4")]
    pub name: String,
    /// The sense of the model.
    #[prost(string, tag = "5")]
    pub sense: String,
}

impl HashModel {
    pub fn build(model: &Model) -> Vec<u8> {
        let m = HashModel {
            objective: HashExpr::build(&model.objective),
            constraints: HashConstr::build(&model.constraints),
            environment: HashEnv::build(&model.environment),
            sense: model.sense.to_string(),
            name: model.name.clone(),
        };
        // dbg!(&m);
        m.encode_to_vec()
    }
}

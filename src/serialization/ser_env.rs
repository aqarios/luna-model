use std::cell::Ref;

use prost::{Enumeration, Message};

use crate::core::{Bounds, Environment, VarId, Variable, Vtype};

#[derive(Clone, PartialEq, Message)]
pub struct SerializableBounds {
    #[prost(double, optional, tag = "1")]
    lower: Option<f64>,
    #[prost(double, optional, tag = "2")]
    upper: Option<f64>,
}

impl From<Bounds> for SerializableBounds {
    fn from(value: Bounds) -> Self {
        Self {
            lower: value.lower,
            upper: value.upper,
        }
    }
}

impl Into<Bounds> for SerializableBounds {
    fn into(self) -> Bounds {
        Bounds::new(self.lower, self.upper)
    }
}

impl Into<Bounds> for &SerializableBounds {
    fn into(self) -> Bounds {
        Bounds::new(self.lower, self.upper)
    }
}

#[derive(Debug, Clone, PartialEq, Enumeration)]
#[repr(i32)]
pub enum SerializableVtype {
    Real = 0,
    Integer = 1,
    Binary = 2,
    Spin = 3,
}

impl From<Vtype> for SerializableVtype {
    fn from(value: Vtype) -> Self {
        match value {
            Vtype::Binary => SerializableVtype::Binary,
            Vtype::Spin => SerializableVtype::Spin,
            Vtype::Integer => SerializableVtype::Integer,
            Vtype::Real => SerializableVtype::Real,
        }
    }
}

impl Into<Vtype> for SerializableVtype {
    fn into(self) -> Vtype {
        match self {
            SerializableVtype::Binary => Vtype::Binary,
            SerializableVtype::Spin => Vtype::Spin,
            SerializableVtype::Integer => Vtype::Integer,
            SerializableVtype::Real => Vtype::Real,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerializableVariable {
    #[prost(string, tag = "1")]
    name: String,
    #[prost(enumeration = "SerializableVtype", tag = "2")]
    vtype: i32,
    #[prost(message, tag = "3")]
    bounds: Option<SerializableBounds>,
    #[prost(uint32, tag = "4")]
    env_id: u32,
}

impl SerializableVariable {
    fn new(
        name: String,
        vtype: SerializableVtype,
        bounds: Option<SerializableBounds>,
        env_id: u32,
    ) -> Self {
        Self {
            name,
            vtype: vtype.into(),
            bounds,
            env_id,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct SerializableEnvironment {
    #[prost(uint32, tag = "1")]
    id: u32,
    #[prost(message, repeated, tag = "2")]
    variables: Vec<SerializableVariable>,
    #[prost(uint32, tag = "3")]
    varcount: u32,
}

impl SerializableEnvironment {
    pub fn new(environment: Ref<'_, Environment<VarId>>) -> Self {
        Self {
            id: environment.id as u32,
            variables: Self::build_variables(&environment.variables),
            varcount: environment.varcount.0,
        }
    }

    fn build_variables(variables: &Vec<Variable>) -> Vec<SerializableVariable> {
        variables
            .iter()
            .map(|var| {
                // gen
                SerializableVariable::new(
                    var.name.clone(),
                    var.vtype.into(),
                    Some(var.bounds.into()),
                    var.env_id as u32,
                )
            })
            .collect()
    }

    pub fn extract(&self) -> Environment<VarId> {
        let env_id: u8 = self.id.try_into().expect("the env id is not a u8");
        let mut env = Environment::new_for(env_id);
        env.varcount = VarId(self.varcount);
        env.variables = self
            .variables
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let var = Variable::new(
                    v.name.clone(),
                    Some(&v.vtype().into()),
                    v.bounds.as_ref().map(move |b| b.into()),
                    env_id,
                );
                env.variables_lookup.insert(v.name.clone(), VarId(i as u32));
                var
            })
            .collect();
        env
    }
}

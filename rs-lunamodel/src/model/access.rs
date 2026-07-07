use lunamodel_types::{Sense, Vtype};

use crate::{
    ConstraintCollection, Env, Expression, Model, Result, Specs, VarRef, traits::TryAsPtr,
};

impl Model {
    pub fn name(&self) -> Result<String> {
        out!(model_name(@self))
    }

    pub fn sense(&self) -> Result<Sense> {
        out!(model_sense(@self))
    }

    pub fn env(&self) -> Result<Env> {
        out!(model_env(@self))
    }

    pub fn specs(&self) -> Result<Specs> {
        out!(model_specs(@self))
    }

    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        out!(model_vtypes(@self))
    }

    pub fn num_variables(&self) -> Result<usize> {
        out!(model_num_variables(@self))
    }

    pub fn vars(&self) -> Result<Vec<VarRef>> {
        out!(model_vars(@self))
    }

    pub fn var(&self, name: &str) -> Result<VarRef> {
        out!(model_var(@self, name.try_as_ptr()?))
    }

    pub fn objective(&self) -> Result<Expression> {
        out!(model_objective(@self))
    }

    pub fn constraints(&self) -> Result<ConstraintCollection> {
        out!(model_constraints(@self))
    }
}

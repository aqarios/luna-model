use lunamodel_types::Vtype;

use super::VarRef;
use crate::bounds::Bounds;

use crate::{Error, Result, Status};

impl VarRef {
    /// Returns the raw variable index inside the environment.
    pub fn id(&self) -> Result<u32> {
        out!(vref_id(@self))
    }

    /// Checks that the variable is still live in its environment, failing early
    /// if it is not.
    pub fn check_living(&self) -> Result<()> {
        match out!(vref_check_living(@self))? {
            true => Ok(()),
            false => Err(Error {
                status: Status::Absent,
                message: "variable is not live in its environment".to_string(),
                kind_tag: None,
            }),
        }
    }

    /// Returns the variable name.
    pub fn name(&self) -> Result<String> {
        out!(vref_name(@self))
    }

    /// Returns the variable type.
    pub fn vtype(&self) -> Result<Vtype> {
        out!(vref_vtype(@self))
    }

    /// Returns a stable hash derived from the variable's resolved name.
    pub fn hash(&self) -> Result<u64> {
        out!(vref_hash(@self))
    }

    /// Returns a copy of the variable's bounds.
    pub fn bounds(&self) -> Result<Bounds> {
        out!(vref_bounds(@self))
    }

    /// Returns the inverted companion variable, or `None` if the variable has no
    /// inverted companion.
    pub fn inverted(&self) -> Result<Option<VarRef>> {
        out!(vref_inverted(@self))
    }
}

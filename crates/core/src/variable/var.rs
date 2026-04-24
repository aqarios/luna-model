//! Core variable record stored inside environments.

use lunamodel_error::LunaModelResult;
use lunamodel_types::{VarIdx, VarName, Vtype};

use crate::bounds::{Bounds, Concretize, LazyBounds};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub(crate) name: VarName,
    pub(crate) vtype: Vtype,
    pub(crate) bounds: Bounds,
    pub inverted: Option<VarIdx>,
}

impl Variable {
    /// Creates a new variable and concretizes its bounds for the chosen type.
    ///
    /// Bounds are validated here instead of at every call site so the rest of
    /// the core crate can assume stored variables already satisfy the type/bound
    /// invariants.
    pub fn new(name: &str, vtype: Vtype, bounds: Option<LazyBounds>) -> LunaModelResult<Self> {
        let bounds = bounds.concretize(&vtype)?;
        Ok(Self {
            name: name.into(),
            vtype,
            bounds,
            inverted: None,
        })
    }

    #[inline]
    /// Returns the stored variable name.
    pub fn name(&self) -> &VarName {
        &self.name
    }

    #[inline]
    /// Returns the variable type.
    pub fn vtype(&self) -> Vtype {
        self.vtype
    }

    #[inline]
    /// Returns the concrete bounds stored on the variable.
    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

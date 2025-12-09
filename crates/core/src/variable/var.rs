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
    pub fn name(&self) -> &VarName {
        &self.name
    }

    #[inline]
    pub fn vtype(&self) -> Vtype {
        self.vtype
    }

    #[inline]
    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

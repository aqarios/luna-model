use crate::variable::LazyBounds;
use lunamodel_error::LunaModelResult;
use lunamodel_types::{EnvIdx, VarIdx, VarName, Vtype};

use super::bounds::{Bounds, Concretize};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub(crate) name: VarName,
    pub(crate) vtype: Vtype,
    pub(crate) bounds: Bounds,
    pub(crate) envid: EnvIdx,
    pub(crate) inverted: Option<VarIdx>,
}

impl Variable {
    pub fn new(
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        envid: EnvIdx,
    ) -> LunaModelResult<Self> {
        let bounds = bounds.concretize(&vtype)?;
        Ok(Self {
            name: name.into(),
            vtype,
            bounds,
            envid,
            inverted: None,
        })
    }
}

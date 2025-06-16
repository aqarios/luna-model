use pyo3::FromPyObject;

use crate::transformations::base_passes::ConcretePass;

use super::{py_change_sense::PyChangeSensePass, py_pass_base::PyPass};

#[derive(FromPyObject)]
pub enum AnyPass {
    ChangeSense(PyChangeSensePass)
}

impl AnyPass {
    pub fn as_pass(self) -> ConcretePass {
        match self {
            Self::ChangeSense(x) => x.as_pass()
        }
    }
}

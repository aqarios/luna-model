//! Mutation operations for Python constraint collections.

use lunamodel_error::LunaModelResult;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, Python, pymethods};

use super::PyConstraintCollection;
use crate::{
    args::PyCArg,
    constraint::utils::{ConstraintsIn, NameIn, constraints_from_input},
};

#[derive(FromPyObject, Debug)]
enum Other {
    // SINGLE
    Constr(PyCArg),
    Tuple((PyCArg, String)),
    // MULTIPLE, with a name prefix: `(constraints, name)`
    Prefixed((ConstraintsIn, String)),
    // MULTIPLE
    Multi(ConstraintsIn),
}

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    /// In-place constraint addition using `+=`.
    fn __iadd__(&mut self, py: Python, other: Other) -> PyResult<()> {
        let _: () = match other {
            Other::Constr(constr) => {
                _ = self
                    .inner_mut()
                    .add_constraint(constr.c.read_arc().clone(), None)?
            }
            Other::Tuple((constr, name)) => {
                _ = self
                    .inner_mut()
                    .add_constraint(constr.c.read_arc().clone(), Some(name))?
            }
            Other::Prefixed((o, name)) => _ = self.add_many_py(py, o, Some(NameIn::S(name)))?,
            Other::Multi(o) => _ = self.add_many_py(py, o, None)?,
        };
        Ok(())
    }

    /// Add a constraint to the collection.
    fn add_constraint(&mut self, constr: PyCArg, name: Option<String>) -> PyResult<String> {
        Ok(self
            .inner_mut()
            .add_constraint(constr.c.read_arc().clone(), name)?)
    }

    fn add_constraints(
        &mut self,
        py: Python,
        constraints: ConstraintsIn,
        name: Option<NameIn>,
    ) -> PyResult<Vec<String>> {
        Ok(self.add_many_py(py, constraints, name)?)
    }

    fn __setitem__(&mut self, key: String, constr: PyCArg) -> PyResult<()> {
        Ok(self
            .inner_mut()
            .set_constraint(&key, constr.c.read_arc().clone())?)
    }

    fn remove(&mut self, key: String) -> PyResult<()> {
        Ok(self.inner_mut().remove_constraint(&key)?)
    }
}

impl PyConstraintCollection {
    pub fn add_many_py(
        &mut self,
        py: Python,
        ccin: ConstraintsIn,
        nin: Option<NameIn>,
    ) -> LunaModelResult<Vec<String>> {
        let constrs = constraints_from_input(py, ccin, nin)?;
        self.inner_mut().add_many_constraints(constrs)
    }
}

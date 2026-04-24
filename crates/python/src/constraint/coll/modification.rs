//! Mutation operations for Python constraint collections.

use lunamodel_error::LunaModelResult;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, Python, pymethods};

use super::PyConstraintCollection;
use crate::{
    args::PyCArg,
    constraint::utils::{ConstraintsIn, NameIn, add_many_constraint},
};

#[derive(FromPyObject, Debug)]
enum Other {
    // SINGLE
    Constr(PyCArg),
    Tuple((PyCArg, String)),
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
                    .write()
                    .add_constraint(constr.c.read_arc().clone(), None)?
            }
            Other::Tuple((constr, name)) => {
                _ = self
                    .write()
                    .add_constraint(constr.c.read_arc().clone(), Some(name))?
            }
            Other::Multi(o) => _ = self.add_many_py(py, o, None),
        };
        Ok(())
    }

    /// Add a constraint to the collection.
    fn add_constraint(&mut self, constr: PyCArg, name: Option<String>) -> PyResult<String> {
        Ok(self
            .write()
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
            .write()
            .set_constraint(&key, constr.c.read_arc().clone())?)
    }

    fn remove(&mut self, key: String) -> PyResult<()> {
        Ok(self.write().remove_constraint(&key)?)
    }
}

impl PyConstraintCollection {
    pub fn add_many_py(
        &mut self,
        py: Python,
        ccin: ConstraintsIn,
        nin: Option<NameIn>,
    ) -> LunaModelResult<Vec<String>> {
        add_many_constraint(py, &mut self.write(), ccin, nin)
    }
}

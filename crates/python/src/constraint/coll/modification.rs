use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, Python, pymethods};

use super::PyConstraintCollection;
use crate::{
    args::PyCArg,
    constraint::utils::{ConstraintsIn, NameIn},
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
    ///
    /// Parameters
    /// ----------
    /// constraint : Constraint | tuple[Constraint, str] | ConstraintCollection |
    /// tuple[ConstraintCollection, str] | Sequence[Constraint]
    ///     The constraint to add.
    ///
    /// Returns
    /// -------
    /// ConstraintCollection
    ///     The updated collection.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the value is not a `Constraint` or valid symbolic comparison.
    fn __iadd__(&mut self, py: Python, other: Other) -> PyResult<()> {
        Ok(match other {
            Other::Constr(constr) => {
                _ = self.c.add_constraint(constr.c.read_arc().clone(), None)?
            }
            Other::Tuple((constr, name)) => {
                _ = self
                    .c
                    .add_constraint(constr.c.read_arc().clone(), Some(name))?
            }
            Other::Multi(o) => _ = self.c.add_many_py(py, o, None),
        })
    }

    /// Add a constraint to the collection.
    ///
    /// Parameters
    /// ----------
    /// constraint : Constraint
    ///     The constraint to be added.
    /// name : str, optional
    ///     The name of the constraint to be added.
    fn add_constraint(&mut self, constr: PyCArg, name: Option<String>) -> PyResult<String> {
        Ok(self.c.add_constraint(constr.c.read_arc().clone(), name)?)
    }

    fn add_constraints(
        &mut self,
        py: Python,
        constraints: ConstraintsIn,
        name: Option<NameIn>,
    ) -> PyResult<Vec<String>> {
        Ok(self.c.add_many_py(py, constraints, name)?)
    }

    fn __setitem__(&mut self, key: String, constr: PyCArg) -> PyResult<()> {
        Ok(self.c.set_constraint(&key, constr.c.read_arc().clone())?)
    }

    fn remove(&mut self, key: String) -> PyResult<()> {
        Ok(self.c.remove_constraint(&key)?)
    }
}

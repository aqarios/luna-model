use pyo3::{FromPyObject, PyResult, pymethods};

use crate::PyConstraint;

use super::PyConstraintCollection;

#[derive(FromPyObject)]
enum Other {
    Constr(PyConstraint),
    Tuple((PyConstraint, String)),
}

#[pymethods]
impl PyConstraintCollection {
    /// In-place constraint addition using `+=`.
    ///
    /// Parameters
    /// ----------
    /// constraint : Constraint | tuple[Constraint, str]
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
    fn __iadd__(&mut self, other: Other) -> PyResult<()> {
        Ok(match other {
            Other::Constr(constr) => self.c.add_constraint(constr.c.read_arc().clone(), None)?,
            Other::Tuple((constr, name)) => self
                .c
                .add_constraint(constr.c.read_arc().clone(), Some(name))?,
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
    fn add_constraint(&mut self, constr: PyConstraint, name: Option<String>) -> PyResult<()> {
        Ok(self.c.add_constraint(constr.c.read_arc().clone(), name)?)
    }

    fn __setitem(&mut self, key: String, constr: PyConstraint) -> PyResult<()> {
        Ok(self.c.set_constraint(&key, constr.c.read_arc().clone())?)
    }

    fn remove(&mut self, key: String) -> PyResult<()> {
        Ok(self.c.remove_constraint(&key)?)
    }
}

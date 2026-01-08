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
    fn __iadd__(&mut self, other: Other) -> PyResult<()> {
        Ok(match other {
            Other::Constr(constr) => self
                .c
                .write_arc()
                .add_constraint(constr.c.read_arc().clone(), None)?,
            Other::Tuple((constr, name)) => self
                .c
                .write_arc()
                .add_constraint(constr.c.read_arc().clone(), Some(name))?,
        })
    }

    fn add_constraint(&mut self, constr: PyConstraint, name: Option<String>) -> PyResult<()> {
        Ok(self
            .c
            .write_arc()
            .add_constraint(constr.c.read_arc().clone(), name)?)
    }
}

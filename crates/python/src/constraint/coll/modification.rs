use lunamodel_core::prelude::Constraint;
use lunamodel_unwind::*;
use pyo3::{FromPyObject, PyResult, pymethods};

use super::PyConstraintCollection;
use crate::PyConstraint;

#[derive(FromPyObject)]
enum MaybeNamed {
    Named((PyConstraint, String)),
    Not(PyConstraint),
}

impl Into<(Constraint, Option<String>)> for MaybeNamed {
    fn into(self) -> (Constraint, Option<String>) {
        match self {
            Self::Named((c, n)) => (c.c.read_arc().clone(), Some(n)),
            Self::Not(c) => (c.c.read_arc().clone(), None),
        }
    }
}

#[derive(FromPyObject)]
enum Other {
    Constr(PyConstraint),
    Tuple((PyConstraint, String)),
    Coll(PyConstraintCollection),
    CollWithPrefix((PyConstraintCollection, String)),
    Many(Vec<MaybeNamed>),
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
    fn __iadd__(&mut self, other: Other) -> PyResult<()> {
        Ok(match other {
            Other::Many(others) => self.c.add_many(others.into_iter().map(|c| c.into()))?,
            Other::Coll(coll) => self.c.add_collection(coll.c, None)?,
            Other::CollWithPrefix((coll, prefix)) => self.c.add_collection(coll.c, Some(prefix))?,
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

    fn __setitem__(&mut self, key: String, constr: PyConstraint) -> PyResult<()> {
        Ok(self.c.set_constraint(&key, constr.c.read_arc().clone())?)
    }

    fn remove(&mut self, key: String) -> PyResult<()> {
        Ok(self.c.remove_constraint(&key)?)
    }
}

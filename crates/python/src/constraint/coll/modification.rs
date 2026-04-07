use lunamodel_core::prelude::Constraint;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_unwind::*;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::{FromPyObject, Py, PyAny, PyErr, PyResult, Python, pymethods};

use super::PyConstraintCollection;
use crate::PyConstraint;

#[derive(FromPyObject, Debug)]
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

#[derive(FromPyObject, Debug)]
enum Other {
    // BELOW Needs higher prio in resolving than Vec.
    Arr(Py<PyArray1<Py<PyAny>>>),
    ArrBaseName((Py<PyArray1<Py<PyAny>>>, String)),
    // ABOVE Needs higher prio in resolving than Vec.
    Constr(PyConstraint),
    Tuple((PyConstraint, String)),
    Coll(PyConstraintCollection),
    CollWithPrefix((PyConstraintCollection, String)),
    Many(Vec<MaybeNamed>),
    ManyBaseName((Vec<PyConstraint>, String)),
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
            Other::ArrBaseName((others, name)) => add_nparr(py, self, others, Some(name))?,
            Other::Arr(others) => add_nparr(py, self, others, None)?,
            Other::ManyBaseName((others, base_name)) => self.c.add_many(
                others
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| (c.c.read_arc().clone(), Some(format!("{base_name}_{i}")))),
            )?,
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

fn add_nparr(
    py: Python,
    c: &mut PyConstraintCollection,
    arr: Py<PyArray1<Py<PyAny>>>,
    base_name: Option<String>,
) -> LunaModelResult<()> {
    let arr = arr.bind(py);
    let readonly = arr.readonly();
    let iter = readonly.as_array().into_iter();
    let data = match base_name {
        Some(name) => iter
            .enumerate()
            .map(|(i, e)| make_element(py, e, Some((&name, i))))
            .collect::<LunaModelResult<Vec<_>>>()?,
        None => iter
            .map(|e| make_element(py, e, None))
            .collect::<LunaModelResult<Vec<_>>>()?,
    };
    c.c.add_many(data.into_iter())
}

fn make_element(
    py: Python,
    elem: &Py<PyAny>,
    name: Option<(&str, usize)>,
) -> LunaModelResult<(Constraint, Option<String>)> {
    let mn = match name {
        Some((base, idx)) => Some(format!("{base}_{idx}")),
        None => None,
    };
    match elem.extract::<PyConstraint>(py) {
        Ok(c) => Ok((c.c.read_arc().clone(), mn)),
        Err(e) => {
            let mapped = LunaModelError::Dtype(e.to_string().into());
            let pye: PyErr = e.into();
            Err(LunaModelError::WithCause(Box::new(mapped), pye.into()))
        }
    }
}

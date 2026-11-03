//! Helper utilities for Python constraint-collection wrappers.

use std::sync::Arc;

use lunamodel_core::Constraint;
use lunamodel_error::{LunaModelError, LunaModelResult};
use numpy::{PyArray1, PyArrayMethods};
use pyo3::{FromPyObject, Py, PyAny, PyErr, Python};

use crate::args::{PyCArg, PyColArg};

#[derive(FromPyObject, Debug)]
pub enum MaybeNamedVariants {
    NamedA((String, PyCArg)),
    NamedB((PyCArg, String)),
    Not(PyCArg),
}

trait ToIter {
    fn to_iter(self) -> impl Iterator<Item = (Constraint, Option<String>)>;
}

impl ToIter for Vec<MaybeNamedVariants> {
    fn to_iter(self) -> impl Iterator<Item = (Constraint, Option<String>)> {
        use MaybeNamedVariants::*;
        self.into_iter().map(|e| match e {
            NamedA((name, pyc)) | NamedB((pyc, name)) => (pyc.c.read_arc().clone(), Some(name)),
            Not(pyc) => (pyc.c.read_arc().clone(), None),
        })
    }
}

#[derive(FromPyObject, Debug)]
pub enum ConstraintsIn {
    // NDArray
    Ca(Py<PyArray1<Py<PyAny>>>),
    // ConstraintCollection
    Cc(PyColArg),
    // [Constraint]
    Cv(Vec<PyCArg>),
    // [(String, Constraint) | (Constraint, String) | Constraint]
    Ncv(Vec<MaybeNamedVariants>),
}

#[derive(FromPyObject, Debug)]
pub enum NameIn {
    S(String),
    SV(Vec<String>),
}

/// Converts the accepted Python inputs into `(constraint, name)` pairs.
///
/// Only parses the input; adding the constraints is up to the caller.
pub fn constraints_from_input(
    py: Python,
    ccin: ConstraintsIn,
    nin: Option<NameIn>,
) -> LunaModelResult<Vec<(Constraint, Option<String>)>> {
    use ConstraintsIn::*;
    use NameIn::*;

    match (ccin, nin) {
        // NDArray
        (Ca(arr), None) => from_nparr(py, arr, None),
        (Ca(arr), Some(S(name))) => from_nparr(py, arr, Some(name)),
        (Ca(arr), Some(SV(names))) => from_nparr_many_named(py, arr, names),
        // ConstraintCollection
        (Cc(col), prefix) => {
            let prefix = match prefix {
                None => None,
                Some(S(p)) => Some(p),
                Some(SV(_)) => return Err(names_sequence_not_allowed()),
            };
            Ok(col
                .read()
                .clone()
                .into_iter()
                .map(|(name, constr)| {
                    let name = match &prefix {
                        Some(p) => format!("{p}_{name}"),
                        None => name,
                    };
                    (constr, Some(name))
                })
                .collect())
        }
        // [Constraint]
        (Cv(seq), None) => Ok(seq
            .into_iter()
            .map(|e| (e.c.read_arc().clone(), None))
            .collect()),
        (Cv(seq), Some(S(name))) => Ok(seq
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e.c.read_arc().clone(), Some(format!("{name}_{i}"))))
            .collect()),
        (Cv(seq), Some(SV(names))) => {
            check_names_len(names.len(), seq.len())?;
            Ok(seq
                .into_iter()
                .zip(names)
                .map(|(e, name)| (e.c.read_arc().clone(), Some(name)))
                .collect())
        }
        // [(String, Constraint) | (Constraint, String) | Constraint]
        (Ncv(tup), None) => Ok(tup.to_iter().collect()),
        (Ncv(tup), Some(S(base))) => Ok(tup
            .to_iter()
            .enumerate()
            .map(|(i, (e, n))| {
                (
                    e,
                    Some(match n {
                        Some(n) => format!("{base}_{n}"),
                        None => format!("{base}_{i}"),
                    }),
                )
            })
            .collect()),
        (Ncv(_), Some(SV(_))) => Err(names_sequence_not_allowed()),
    }
}

fn names_sequence_not_allowed() -> LunaModelError {
    LunaModelError::Internal("names sequence is not allowed when constraints are provided as ConstraintCollection or as a sequence of (name, constraint)/(constraint, name) items".into())
}

fn check_names_len(names: usize, constraints: usize) -> LunaModelResult<()> {
    if names != constraints {
        return Err(LunaModelError::Internal(
            format!(
                "names and constraints must have the same length (got {names}, expected {constraints})"
            )
            .into(),
        ));
    }
    Ok(())
}

fn from_nparr(
    py: Python,
    arr: Py<PyArray1<Py<PyAny>>>,
    base_name: Option<String>,
) -> LunaModelResult<Vec<(Constraint, Option<String>)>> {
    let arr = arr.bind(py);
    let readonly = arr.readonly();
    let iter = readonly.as_array().into_iter();
    match base_name {
        Some(name) => iter
            .enumerate()
            .map(|(i, e)| make_element(py, e, Some(format!("{name}_{i}"))))
            .collect(),
        None => iter.map(|e| make_element(py, e, None)).collect(),
    }
}

fn from_nparr_many_named(
    py: Python,
    arr: Py<PyArray1<Py<PyAny>>>,
    names: Vec<String>,
) -> LunaModelResult<Vec<(Constraint, Option<String>)>> {
    let arr = arr.bind(py);
    let readonly = arr.readonly();
    let iter = readonly.as_array().into_iter();
    check_names_len(names.len(), iter.len())?;
    iter.zip(names)
        .map(|(e, name)| make_element(py, e, Some(name)))
        .collect()
}

fn make_element(
    py: Python,
    elem: &Py<PyAny>,
    name: Option<String>,
) -> LunaModelResult<(Constraint, Option<String>)> {
    match elem.extract::<PyCArg>(py) {
        Ok(c) => Ok((c.c.read_arc().clone(), name)),
        Err(e) => {
            let mapped = LunaModelError::Dtype(e.to_string().into());
            let pye: PyErr = e;
            Err(LunaModelError::WithCause(Box::new(mapped), Arc::new(pye)))
        }
    }
}

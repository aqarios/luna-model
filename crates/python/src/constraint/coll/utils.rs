use lunamodel_core::{Constraint, ConstraintCollection};
use lunamodel_error::{LunaModelError, LunaModelResult};
use numpy::{PyArray1, PyArrayMethods};
use pyo3::{FromPyObject, Py, PyAny, PyErr, Python};

use crate::args::PyCArg;

use super::PyConstraintCollection;

#[derive(FromPyObject, Debug)]
pub enum MaybeNamedVariants {
    NamedA((String, PyCArg)),
    NamedB((PyCArg, String)),
    Not(PyCArg),
}

trait AsIter {
    fn as_iter(self) -> impl Iterator<Item = (Constraint, Option<String>)>;
}

impl AsIter for Vec<MaybeNamedVariants> {
    fn as_iter(self) -> impl Iterator<Item = (Constraint, Option<String>)> {
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
    Cc(PyConstraintCollection),
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

pub fn add_many_constraint(
    py: Python,
    cc: &mut ConstraintCollection,
    ccin: ConstraintsIn,
    nin: Option<NameIn>,
) -> LunaModelResult<Vec<String>> {
    use ConstraintsIn::*;
    use NameIn::*;

    match (ccin, nin) {
        // NDArray
        (Ca(arr), None) => add_nparr(py, cc, arr, None),
        (Ca(arr), Some(S(name))) => add_nparr(py, cc, arr, Some(name)),
        (Ca(arr), Some(SV(names))) => add_nparr_many_named(py, cc, arr, names),
        // ConstraintCollection
        (Cc(col), None) => cc.add_collection(col.read().clone(), None),
        (Cc(col), Some(S(name))) => cc.add_collection(col.read().clone(), Some(name)),
        // [Constraint]
        (Cv(seq), None) => {
            cc.add_many_constraints(seq.into_iter().map(|e| (e.c.read_arc().clone(), None)))
        }
        (Cv(seq), Some(S(name))) => cc.add_many_constraints(
            seq.into_iter()
                .enumerate()
                .map(|(i, e)| (e.c.read_arc().clone(), Some(format!("{name}_{i}")))),
        ),
        (Cv(seq), Some(SV(names))) => {
            if seq.len() != names.len() {
                return Err(LunaModelError::Internal(
                    format!(
                        "names and constraints must have the same length (got {}, expected {})",
                        names.len(),
                        seq.len()
                    )
                    .into(),
                ));
            }
            cc.add_many_constraints(
                seq.into_iter()
                    .zip(names)
                    .map(|(e, name)| (e.c.read_arc().clone(), Some(name))),
            )
        }
        // [(String, Constraint) | (Constraint, String) | Constraint]
        (Ncv(tup), None) => cc.add_many_constraints(tup.as_iter()),
        (Ncv(tup), Some(S(base))) => {
            cc.add_many_constraints(tup.as_iter().enumerate().map(|(i, (e, n))| {
                (
                    e,
                    Some(match n {
                        Some(n) => format!("{base}_{n}"),
                        None => format!("{base}_{i}"),
                    }),
                )
            }))
        }

        // not ok
        (Cc(_), Some(SV(_))) | (Ncv(_), Some(SV(_))) => {
            Err(LunaModelError::Internal("names sequence is not allowed when constraints are provided as ConstraintCollection or as a sequence of (name, constraint)/(constraint, name) 
items".into()))
        }
    }
}

fn add_nparr(
    py: Python,
    c: &mut ConstraintCollection,
    arr: Py<PyArray1<Py<PyAny>>>,
    base_name: Option<String>,
) -> LunaModelResult<Vec<String>> {
    let arr = arr.bind(py);
    let readonly = arr.readonly();
    let iter = readonly.as_array().into_iter();
    let data = match base_name {
        Some(name) => iter
            .enumerate()
            .map(|(i, e)| make_element(py, e, Some(format!("{name}_{i}"))))
            .collect::<LunaModelResult<Vec<_>>>()?,
        None => iter
            .map(|e| make_element(py, e, None))
            .collect::<LunaModelResult<Vec<_>>>()?,
    };
    c.add_many_constraints(data.into_iter())
}

fn add_nparr_many_named(
    py: Python,
    c: &mut ConstraintCollection,
    arr: Py<PyArray1<Py<PyAny>>>,
    names: Vec<String>,
) -> LunaModelResult<Vec<String>> {
    let arr = arr.bind(py);
    let readonly = arr.readonly();
    let iter = readonly.as_array().into_iter();
    if iter.len() != names.len() {
        return Err(LunaModelError::Internal(
            format!(
                "names and constraints must have the same length (got {}, expected {})",
                names.len(),
                iter.len()
            )
            .into(),
        ));
    }
    let data = iter
        .zip(names)
        .map(|(e, name)| make_element(py, e, Some(name)))
        .collect::<LunaModelResult<Vec<_>>>()?;
    c.add_many_constraints(data.into_iter())
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
            Err(LunaModelError::WithCause(Box::new(mapped), pye.into()))
        }
    }
}

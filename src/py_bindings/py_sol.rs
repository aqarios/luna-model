use crate::core::solution::sol::SampleCol;
use crate::core::{
    ConcreteAssignmentTypes, ConcreteBias, RcSolution, Samples, Solution, VarAssignment, Vtype,
};
use crate::errors::{SampleIncorrectLengthErr, SampleUnexpectedVariableErr};
use crate::py_bindings::py_env::{PyEnvironment, CURRENT_ENV};
use crate::py_bindings::py_exceptions::NoActiveEnvironmentFoundError;
use crate::py_bindings::py_model::PyModel;
use crate::py_bindings::py_res::{PyResultIterator, PyResultView};
use crate::py_bindings::py_sample::PySamples;
use crate::py_bindings::py_timing::PyTiming;
use crate::py_bindings::py_var::PyVariable;
use crate::serialization::{
    Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
};
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::{PyIndexError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::IntoPyObjectExt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(pub VarAssignment<ConcreteAssignmentTypes>);

#[derive(Debug, Clone)]
pub enum SampleKey {
    Str(String),
    Var(PyVariable),
}

#[pyclass(unsendable, name = "Solution", module = "aqmodels")]
#[derive(Deref, DerefMut, Debug)]
pub struct PySolution(pub RcSolution<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<RcSolution<ConcreteBias, ConcreteAssignmentTypes>> for PySolution {
    fn into(self) -> RcSolution<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

#[pymethods]
impl PySolution {
    #[staticmethod]
    #[pyo3(signature=(component_types, binary_cols=None, spin_cols=None, int_cols=None, real_cols=None, raw_energies=None, timing=None, counts=None)
    )]
    fn build(
        component_types: Vec<Vtype>,
        binary_cols: Option<Vec<Vec<u8>>>,
        spin_cols: Option<Vec<Vec<i8>>>,
        int_cols: Option<Vec<Vec<i64>>>,
        real_cols: Option<Vec<Vec<f64>>>,
        raw_energies: Option<Vec<Option<f64>>>,
        timing: Option<PyTiming>,
        counts: Option<Vec<usize>>,
    ) -> PyResult<Self> {
        // todo! change to numpy arrays instead of vecs.
        // todo! move further down in rust code.
        let mut sol = Solution::default();

        let (mut lb, mut ls, mut li, mut lr) = (0, 0, 0, 0);
        let binary_cols = binary_cols.unwrap_or(Vec::new());
        let spin_cols = spin_cols.unwrap_or(Vec::new());
        let int_cols = int_cols.unwrap_or(Vec::new());
        let real_cols = real_cols.unwrap_or(Vec::new());

        let mut num_samples: Option<usize> = None;
        for (i, ct) in component_types.iter().enumerate() {
            let len = match ct {
                Vtype::Binary => {
                    let bc = binary_cols[lb].clone();
                    let bc_len = bc.len();
                    sol.add_column(SampleCol::Binary(bc));
                    lb += 1;
                    bc_len
                }
                Vtype::Spin => {
                    let sc = spin_cols[ls].clone();
                    let sc_len = sc.len();
                    sol.add_column(SampleCol::Spin(sc));
                    ls += 1;
                    sc_len
                }
                Vtype::Integer => {
                    let ic = int_cols[li].clone();
                    let ic_len = ic.len();
                    sol.add_column(SampleCol::Integer(ic));
                    li += 1;
                    ic_len
                }
                Vtype::Real => {
                    let rc = real_cols[lr].clone();
                    let rc_len = rc.len();
                    sol.add_column(SampleCol::Real(rc));
                    lr += 1;
                    rc_len
                }
            };
            if let Some(ns) = num_samples {
                if ns != len {
                    return Err(PyRuntimeError::new_err(format!(
                        "The number of samples does not match for column {i}"
                    )));
                }
            } else {
                num_samples = Some(len)
            }
        }
        sol.n_samples = num_samples.unwrap_or(0);
        if let Some(re) = raw_energies {
            sol.raw_energies = re;
        } else {
            sol.raw_energies = vec![None; sol.n_samples];
        }
        if let Some(no) = counts {
            if no.len() != sol.n_samples {
                return Err(PyRuntimeError::new_err(
                    "counts does not match the number of samples given.",
                ));
            }
            sol.counts = no;
        } else {
            sol.counts = vec![1; sol.n_samples];
        }
        sol.obj_values = vec![None; sol.n_samples];
        sol.timing = timing.and_then(|t| Some(t.0));
        Ok(PySolution(RcSolution(Rc::new(sol))))
    }

    #[staticmethod]
    #[pyo3(signature=(data, env=None, model=None)
    )]
    fn from_dict(
        data: HashMap<SampleKey, f64>,
        env: Option<PyEnvironment>,
        model: Option<PyModel>,
    ) -> PyResult<PySolution> {
        if env.is_some() && model.is_some() {
            return Err(PyValueError::new_err(
                "either `env` or `model` has to be `None`",
            ));
        }
        let environment: PyEnvironment = if model.is_some() {
            PyEnvironment(Rc::clone(&model.as_ref().unwrap().borrow().environment))
        } else {
            match env {
                Some(env) => env.clone(),
                None => CURRENT_ENV.with(|current| {
                    current.borrow().clone().ok_or_else(|| {
                        NoActiveEnvironmentFoundError::new_err("no active environment found.")
                    })
                })?,
            }
        };

        let mut sol = Solution::default();
        for v in environment.borrow().variables.iter() {
            match v.vtype {
                Vtype::Binary => sol.add_column(SampleCol::Binary(Vec::with_capacity(1))),
                Vtype::Spin => sol.add_column(SampleCol::Spin(Vec::with_capacity(1))),
                Vtype::Integer => sol.add_column(SampleCol::Integer(Vec::with_capacity(1))),
                Vtype::Real => sol.add_column(SampleCol::Real(Vec::with_capacity(1))),
            }
        }

        let n_vars = environment.borrow().varcount.into();
        let mut sample = vec![f64::default(); n_vars];
        let mut mask = vec![false; n_vars];

        for (k, &v) in data.iter() {
            let var_name = match k {
                SampleKey::Str(s) => s,
                SampleKey::Var(v) => &v.name(),
            };
            let environ = environment.borrow();
            let maybe_var = environ.variables_lookup.get(var_name);
            if maybe_var.is_none() {
                return Err(SampleUnexpectedVariableErr {
                    var_name: var_name.clone(),
                })?;
            }
            let var = maybe_var.unwrap().0 as usize;
            sample[var] = v;
            mask[var] = true;
        }

        if !mask.iter().all(|&x| x) {
            return Err(SampleIncorrectLengthErr)?;
        }

        let energy: Option<f64> = None;
        let _ = sol.extend(sample, 1, energy)?;
        let mut sol_rc = RcSolution(Rc::new(sol));
        if let Some(m) = model {
            sol_rc = m.borrow().evaluate_solution(sol_rc);
        }

        Ok(PySolution(sol_rc))
    }

    #[getter]
    fn results<'a>(&self) -> PyResultIterator {
        PyResultIterator(self.0.iter_results())
    }

    #[getter]
    fn samples(&self) -> PySamples {
        PySamples(Samples(RcSolution::clone(&self)))
    }

    #[getter]
    fn obj_values<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.obj_values
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn raw_energies<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<PyObject>> {
        self.raw_energies
            .iter()
            .map(|x| x.into_py_any(py).unwrap())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn counts<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<usize>> {
        self.counts.to_pyarray(py)
    }

    #[getter]
    fn runtime(&self) -> Option<PyTiming> {
        self.timing.map(|t| PyTiming(t))
    }

    #[getter]
    fn best_sample_idx(&self) -> Option<usize> {
        self.0.best_sample_idx
    }

    #[pyo3(signature=(compress=true, level=3))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .0
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
        .into())
    }

    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    #[staticmethod]
    fn decode(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Ok(PySolution(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        ))
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(py, data)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        PyResultIterator(slf.0.iter_results())
    }

    fn __getitem__(&self, py: Python, item: PyObject) -> PyResult<PyResultView> {
        if let Ok(res_idx) = item.extract::<usize>(py) {
            match self.get_result_view(res_idx) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => Ok(PyResultView(r)),
            }
        } else {
            Err(PyTypeError::new_err("unsupported type for indexing"))
        }
    }

    fn __eq__(&self, other: &PySolution) -> bool {
        &self.0 == &other.0
    }
}

impl<'py> IntoPyObject<'py> for PyVarAssignment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            VarAssignment::Binary(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Spin(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Integer(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            VarAssignment::Real(x) => Ok(x.into_py_any(py)?.into_bound(py)),
        }
    }
}

impl<'py> IntoPyObject<'py> for SampleKey {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            SampleKey::Str(x) => Ok(x.into_py_any(py)?.into_bound(py)),
            SampleKey::Var(x) => Ok(x.into_py_any(py)?.into_bound(py)),
        }
    }
}

impl<'py> FromPyObject<'py> for SampleKey {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(SampleKey::Str(s))
        } else if let Ok(v) = ob.extract::<PyVariable>() {
            Ok(SampleKey::Var(v))
        } else {
            Err(PyTypeError::new_err("keys have to be 'str' or 'Variable'"))
        }
    }
}

impl Hash for SampleKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SampleKey::Str(s) => s.hash(state),
            SampleKey::Var(v) => v.hash(state),
        }
    }
}

impl PartialEq<Self> for SampleKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SampleKey::Str(s1), SampleKey::Str(s2)) => s1 == s2,
            (SampleKey::Var(v1), SampleKey::Var(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Eq for SampleKey {}

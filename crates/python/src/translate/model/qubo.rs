use lunamodel_translate::model::{Qubo, QuboTranslator};
use lunamodel_unwind::*;
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods, ToPyArray};
use pyo3::{Bound, FromPyObject, PyResult, Python, pyclass, pymethods};

use crate::{PyModel, args::PyModelArg, types::PyVtype};

#[derive(FromPyObject)]
enum QuboType<'py> {
    F64(PyReadonlyArray2<'py, f64>),
    I64(PyReadonlyArray2<'py, i64>),
}

impl<'py> QuboType<'py> {
    fn to_dense(&self) -> (Vec<f64>, usize) {
        match &self {
            QuboType::F64(q) => (
                q.as_slice()
                    .expect("failed to convert to slice")
                    .iter()
                    .map(|&v| v)
                    .collect::<Vec<f64>>(),
                q.shape()[0],
            ),
            QuboType::I64(q) => (
                q.as_slice()
                    .expect("failed to convert to slice")
                    .iter()
                    .map(|&v| v as f64)
                    .collect::<Vec<f64>>(),
                q.shape()[0],
            ),
        }
    }
}

#[pyclass]
pub struct PyQuboTranslator;

#[unwindable]
#[pymethods]
impl PyQuboTranslator {
    #[staticmethod]
    fn to_lm(
        qubo: QuboType,
        offset: Option<f64>,
        variable_names: Option<Vec<String>>,
        name: Option<String>,
        vtype: Option<PyVtype>,
    ) -> PyResult<PyModel> {
        let (dense, num_vars) = qubo.to_dense();
        Ok(QuboTranslator::translate(
            &dense,
            num_vars,
            vtype.map(|v| v.into()),
            offset,
            variable_names,
            name,
        )?
        .into())
    }

    #[staticmethod]
    fn from_lm(model: PyModelArg) -> PyResult<PyQubo> {
        let qubo = QuboTranslator::back_translate(&model.m.read_arc())?;
        Ok(PyQubo(qubo))
    }
}

#[pyclass]
pub struct PyQubo(pub Qubo);

#[unwindable]
#[pymethods]
impl PyQubo {
    #[getter]
    fn matrix<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        Ok(self
            .0
            .matrix_flat
            .to_pyarray(py)
            .reshape((self.0.num_variables, self.0.num_variables))?)
    }

    #[getter]
    fn variable_names(&self) -> Vec<String> {
        self.0.variable_names.clone()
    }

    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    fn offset(&self) -> f64 {
        self.0.offset
    }

    #[getter]
    fn vtype(&self) -> PyVtype {
        self.0.vtype.into()
    }
}

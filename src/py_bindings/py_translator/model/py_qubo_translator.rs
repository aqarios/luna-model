use crate::core::{ConcreteBias, ConcreteIndex, Qubo};
use crate::py_bindings::py_model::PyModel;
use crate::{core::Vtype, translator::MatrixTranslator};
use derive_more::{Deref, DerefMut};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, PyUntypedArray, PyUntypedArrayMethods, ToPyArray};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Qubo", module = "aqmodels.translator")]
#[derive(Deref, DerefMut)]
pub struct PyQubo(pub Qubo<ConcreteIndex, ConcreteBias>);

impl Into<Qubo<ConcreteIndex, ConcreteBias>> for PyQubo {
    fn into(self) -> Qubo<ConcreteIndex, ConcreteBias> {
        self.0
    }
}

#[pymethods]
impl PyQubo {
    #[getter(matrix)]
    fn get_np_matrix<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self
            .matrix_flat
            .to_pyarray(py)
            .reshape((self.num_variables.into(), self.num_variables.into()))?)
    }

    #[getter(variable_names)]
    fn get_variable_names(&self) -> Vec<String> {
        self.variable_names.clone()
    }

    #[getter(name)]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    #[getter(offset)]
    fn get_offset(&self) -> f64 {
        self.offset
    }

    #[getter(vtype)]
    fn get_vtype(&self) -> Vtype {
        self.vtype
    }
}

#[pyclass(unsendable, name = "QuboTranslator", module = "aqmodels.translator")]
pub struct PyQuboTranslator {}

#[derive(FromPyObject)]
enum QuboType<'py> {
    F64(PyReadonlyArray2<'py, f64>),
    I64(PyReadonlyArray2<'py, i64>),
}

#[pymethods]
impl PyQuboTranslator {
    #[staticmethod]
    #[pyo3(signature=(qubo, offset=None, variable_names=None, name=None, vtype=None))]
    fn to_aq(
        qubo: QuboType,
        offset: Option<f64>,
        variable_names: Option<Vec<String>>,
        name: Option<String>,
        vtype: Option<Vtype>,
    ) -> PyModel {
        let (dense, var_num): (&[f64], usize) = match qubo {
            QuboType::F64(q) => (&q.as_slice().expect("failed to convert to slice").iter().map(|&v|v).collect::<Vec<f64>>(), q.shape()[0]),
            QuboType::I64(q) => (&q.as_slice().expect("failed to convert to slice").iter().map(|&v| v as f64).collect::<Vec<f64>>(), q.shape()[0]),
        };
        PyModel(MatrixTranslator::model_from_dense(
            name,
            dense,
            var_num.into(),
            vtype,
            offset,
            variable_names,
        ))
    }

    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq(model: &PyModel) -> PyResult<PyQubo> {
        let qubo = MatrixTranslator::model_to_dense(&model.0)?;
        Ok(PyQubo(qubo))
    }
}

use crate::core::{ConcreteBias, ConcreteIndex, Qubo};
use crate::py_bindings::py_model::PyModel;
use crate::{core::Vtype, translator::MatrixTranslator};
use derive_more::{Deref, DerefMut};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods, ToPyArray};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Qubo", module = "aqmodels.translator")]
#[derive(Deref, DerefMut)]
pub struct PyQubo(pub Qubo<ConcreteIndex, ConcreteBias>);

impl Into<Qubo<ConcreteIndex, ConcreteBias>> for PyQubo {
    fn into(self) -> Qubo<ConcreteIndex, ConcreteBias> {
        self.0
    }
}

impl PyQubo {
    fn new(
        name: String,
        vtype: Vtype,
        matrix_flat: Vec<f64>,
        num_variables: usize,
        offset: f64,
        variable_names: Vec<String>,
    ) -> Self {
        Self(Qubo {
            name,
            vtype,
            matrix_flat,
            num_variables: num_variables.into(),
            offset,
            variable_names,
        })
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

#[pymethods]
impl PyQuboTranslator {
    #[staticmethod]
    #[pyo3(signature=(qubo, offset=None, variable_names=None, name=None, vtype=None))]
    fn to_aq(
        qubo: PyReadonlyArray2<f64>,
        offset: Option<f64>,
        variable_names: Option<Vec<String>>,
        name: Option<String>,
        vtype: Option<Vtype>,
    ) -> PyModel {
        let dense = qubo.as_slice().expect("failed to convert to slice");
        PyModel::new(MatrixTranslator::model_from_dense(
            name,
            dense,
            qubo.shape()[0].into(),
            vtype,
            offset,
            variable_names,
        ))
    }

    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq(model: &PyModel) -> PyResult<PyQubo> {
        let qubo = MatrixTranslator::model_to_dense(&model.concrete_model)?;
        Ok(PyQubo(qubo))
    }
}
